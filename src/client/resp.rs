use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, HeaderMap, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{Error, Module, RArray, RModule, Ruby, Value, scan_args::scan_args};
use wreq::Uri;

use crate::{
    arch::ProcessLocal,
    client::body::{json::Json, stream::BodyReceiver},
    cookie::Cookie,
    error::{memory_error, no_block_given_error, wreq_error},
    gvl,
    header::Headers,
    http::{StatusCode, Version},
    rt,
    tls::TlsInfo,
};

/// A response from a request.
#[magnus::wrap(class = "Wreq::Response", free_immediately, size)]
pub struct Response(ProcessLocal<ResponseInner>);

/// Inner response state owned by the process that received it.
struct ResponseInner {
    uri: Uri,
    version: Version,
    status: StatusCode,
    content_length: Option<u64>,
    headers: HeaderMap,
    local_addr: Option<SocketAddr>,
    remote_addr: Option<SocketAddr>,
    body: ArcSwapOption<Body>,
    extensions: Extensions,
}

/// Represents the state of the HTTP response body.
enum Body {
    /// The body can be streamed once (not yet buffered).
    Streamable(wreq::Body),
    /// The body has been fully read into memory and can be reused.
    Reusable(Bytes),
}

impl Response {
    /// Create a new [`Response`] instance.
    pub fn new(response: wreq::Response) -> Self {
        let uri = response.uri().clone();
        let content_length = response.content_length();
        let local_addr = response.local_addr();
        let remote_addr = response.remote_addr();
        let response = HttpResponse::from(response);
        let (parts, body) = response.into_parts();

        Response(ProcessLocal::new(ResponseInner {
            uri,
            local_addr,
            remote_addr,
            content_length,
            version: Version::from_ffi(parts.version),
            status: StatusCode::from(parts.status),
            headers: parts.headers,
            body: ArcSwapOption::from_pointee(Body::Streamable(body)),
            extensions: parts.extensions,
        }))
    }

    /// Internal method to get the wreq::Response, optionally streaming the body.
    fn response(&self, ruby: &Ruby, stream: bool) -> Result<wreq::Response, Error> {
        let state = self.0.get(ruby)?;

        let build_response = |body: wreq::Body| -> wreq::Response {
            let mut response = HttpResponse::new(body);
            *response.version_mut() = state.version.into_ffi();
            *response.status_mut() = state.status.0;
            *response.headers_mut() = state.headers.clone();
            *response.extensions_mut() = state.extensions.clone();
            wreq::Response::from(response)
        };

        if let Some(arc) = state.body.swap(None) {
            match Arc::try_unwrap(arc) {
                Ok(Body::Streamable(body)) => {
                    return if stream {
                        Ok(build_response(body))
                    } else {
                        let bytes = rt::block_on(
                            ruby,
                            BodyExt::collect(body).map_ok(|buf| buf.to_bytes()),
                        )?
                        .map_err(|err| wreq_error(ruby, err))?;

                        state
                            .body
                            .store(Some(Arc::new(Body::Reusable(bytes.clone()))));

                        Ok(build_response(wreq::Body::from(bytes)))
                    };
                }
                Ok(Body::Reusable(bytes)) => {
                    state
                        .body
                        .store(Some(Arc::new(Body::Reusable(bytes.clone()))));

                    if !stream {
                        return Ok(build_response(wreq::Body::from(bytes)));
                    }
                }
                _ => {}
            };
        }

        Err(memory_error(ruby))
    }
}

impl Response {
    /// Get the response status code as a u16.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn code(ruby: &Ruby, rb_self: &Self) -> Result<u16, Error> {
        rb_self
            .0
            .get(ruby)
            .map(|response| response.status.0.as_u16())
    }

    /// Get the response status code.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn status(ruby: &Ruby, rb_self: &Self) -> Result<StatusCode, Error> {
        rb_self.0.get(ruby).map(|response| response.status)
    }

    /// Get the response HTTP version.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn version(ruby: &Ruby, rb_self: &Self) -> Result<Version, Error> {
        rb_self.0.get(ruby).map(|response| response.version)
    }

    /// Get the response URL.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn url(ruby: &Ruby, rb_self: &Self) -> Result<String, Error> {
        rb_self.0.get(ruby).map(|response| response.uri.to_string())
    }

    /// Get the content length of the response, if known.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn content_length(ruby: &Ruby, rb_self: &Self) -> Result<Option<u64>, Error> {
        rb_self.0.get(ruby).map(|response| response.content_length)
    }

    /// Get the response cookies.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    pub fn cookies(ruby: &Ruby, rb_self: &Self) -> Result<RArray, Error> {
        let response = rb_self.0.get(ruby)?;
        let cookies = Cookie::extract_headers_cookies(&response.headers);
        let ary = ruby.ary_new_capa(cookies.len());
        for cookie in cookies {
            ary.push(cookie)?;
        }
        Ok(ary)
    }

    /// Get the response headers.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn headers(ruby: &Ruby, rb_self: &Self) -> Result<Headers, Error> {
        rb_self
            .0
            .get(ruby)
            .map(|response| Headers::from(response.headers.clone()))
    }

    /// Get the local socket address, if available.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn local_addr(ruby: &Ruby, rb_self: &Self) -> Result<Option<String>, Error> {
        rb_self
            .0
            .get(ruby)
            .map(|response| response.local_addr.map(|addr| addr.to_string()))
    }

    /// Get the remote socket address, if available.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    #[inline]
    pub fn remote_addr(ruby: &Ruby, rb_self: &Self) -> Result<Option<String>, Error> {
        rb_self
            .0
            .get(ruby)
            .map(|response| response.remote_addr.map(|addr| addr.to_string()))
    }

    /// Return peer certificate data retained for this response.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the response belongs to a parent process.
    fn tls_info(ruby: &Ruby, rb_self: &Self) -> Result<Option<TlsInfo>, Error> {
        Ok(rb_self
            .0
            .get(ruby)?
            .extensions
            .get::<wreq::tls::TlsInfo>()
            .cloned()
            .map(TlsInfo))
    }

    /// Get the response body as bytes.
    pub fn bytes(ruby: &Ruby, rb_self: &Self) -> Result<Bytes, Error> {
        let response = rb_self.response(ruby, false)?;
        rt::block_on(ruby, response.bytes())?.map_err(|err| wreq_error(ruby, err))
    }

    ///  Get the full response text given a specific encoding.
    pub fn text(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<String, Error> {
        let args = scan_args::<(), (Option<String>,), (), (), (), ()>(args)?;
        let response = rb_self.response(ruby, false)?;
        match args.optional.0 {
            Some(encoding) => rt::block_on(ruby, response.text_with_charset(encoding))?
                .map_err(|err| wreq_error(ruby, err)),
            None => rt::block_on(ruby, response.text())?.map_err(|err| wreq_error(ruby, err)),
        }
    }

    /// Get the response body as JSON.
    pub fn json(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let response = rb_self.response(ruby, false)?;
        let json =
            rt::block_on(ruby, response.json::<Json>())?.map_err(|err| wreq_error(ruby, err))?;
        crate::serde::serialize(ruby, &json)
    }

    /// Yield response body chunks to the given Ruby block.
    pub fn chunks(ruby: &Ruby, rb_self: &Self) -> Result<(), Error> {
        if !ruby.block_given() {
            return Err(no_block_given_error(ruby));
        }

        let receiver = rb_self
            .response(ruby, true)
            .map(wreq::Response::bytes_stream)
            .map(BodyReceiver::new)?;

        while let Some(chunk) = receiver.next(ruby)? {
            let _: Value = ruby.yield_value(chunk)?;
        }

        Ok(())
    }

    /// Close the response body, dropping any resources.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` before touching a response inherited from the
    /// parent process.
    pub fn close(ruby: &Ruby, rb_self: &Self) -> Result<(), Error> {
        let response = rb_self.0.get(ruby)?;
        gvl::nogvl(|| response.body.swap(None));
        Ok(())
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let response = gem_module.define_class("Response", ruby.class_object())?;
    response.define_method("code", magnus::method!(Response::code, 0))?;
    response.define_method("status", magnus::method!(Response::status, 0))?;
    response.define_method("version", magnus::method!(Response::version, 0))?;
    response.define_method("url", magnus::method!(Response::url, 0))?;
    response.define_method(
        "content_length",
        magnus::method!(Response::content_length, 0),
    )?;
    response.define_method("cookies", magnus::method!(Response::cookies, 0))?;
    response.define_method("headers", magnus::method!(Response::headers, 0))?;
    response.define_method("local_addr", magnus::method!(Response::local_addr, 0))?;
    response.define_method("remote_addr", magnus::method!(Response::remote_addr, 0))?;
    response.define_method("bytes", magnus::method!(Response::bytes, 0))?;
    response.define_method("text", magnus::method!(Response::text, -1))?;
    response.define_method("json", magnus::method!(Response::json, 0))?;
    response.define_method("chunks", magnus::method!(Response::chunks, 0))?;
    response.define_method("close", magnus::method!(Response::close, 0))?;
    response.define_method("tls_info", magnus::method!(Response::tls_info, 0))?;
    Ok(())
}
