use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, HeaderMap, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{Error, Module, RArray, RModule, Ruby, Value, block::Yield};
use wreq::Uri;

use crate::{
    client::body::{BodyReceiver, Json},
    cookie::Cookie,
    error::{memory_error, wreq_error_to_magnus},
    gvl,
    header::Headers,
    http::{StatusCode, Version},
    rt,
};

/// A response from a request.
#[magnus::wrap(class = "Wreq::Response", free_immediately, size)]
pub struct Response {
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

// ===== impl Response =====

impl Response {
    /// Create a new [`Response`] instance.
    pub fn new(response: wreq::Response) -> Self {
        let uri = response.uri().clone();
        let content_length = response.content_length();
        let local_addr = response.local_addr();
        let remote_addr = response.remote_addr();
        let response = HttpResponse::from(response);
        let (parts, body) = response.into_parts();

        Response {
            uri,
            local_addr,
            remote_addr,
            content_length,
            extensions: parts.extensions,
            version: Version::from_ffi(parts.version),
            status: StatusCode::from(parts.status),
            headers: parts.headers,
            body: ArcSwapOption::from_pointee(Body::Streamable(body)),
        }
    }

    /// Internal method to get the wreq::Response, optionally streaming the body.
    fn response(&self, stream: bool) -> Result<wreq::Response, Error> {
        let build_response = |body: wreq::Body| -> wreq::Response {
            let mut response = HttpResponse::new(body);
            *response.version_mut() = self.version.into_ffi();
            *response.status_mut() = self.status.0;
            *response.headers_mut() = self.headers.clone();
            *response.extensions_mut() = self.extensions.clone();
            wreq::Response::from(response)
        };

        if let Some(arc) = self.body.swap(None) {
            match Arc::try_unwrap(arc) {
                Ok(Body::Streamable(body)) => {
                    return if stream {
                        Ok(build_response(body))
                    } else {
                        let bytes = rt::try_block_on(
                            BodyExt::collect(body)
                                .map_ok(|buf| buf.to_bytes())
                                .map_err(wreq_error_to_magnus),
                        )?;

                        self.body
                            .store(Some(Arc::new(Body::Reusable(bytes.clone()))));

                        Ok(build_response(wreq::Body::from(bytes)))
                    };
                }
                Ok(Body::Reusable(bytes)) => {
                    self.body
                        .store(Some(Arc::new(Body::Reusable(bytes.clone()))));

                    if !stream {
                        return Ok(build_response(wreq::Body::from(bytes)));
                    }
                }
                _ => {}
            };
        }

        Err(memory_error())
    }
}

impl Response {
    /// Get the response status code as a u16.
    #[inline]
    pub fn code(&self) -> u16 {
        self.status.0.as_u16()
    }

    /// Get the response status code.
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the response HTTP version.
    #[inline]
    pub fn version(&self) -> Version {
        self.version
    }

    /// Get the response URL.
    #[inline]
    pub fn url(&self) -> String {
        self.uri.to_string()
    }

    /// Get the content length of the response, if known.
    #[inline]
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Get the response cookies.
    pub fn cookies(ruby: &Ruby, rb_self: &Self) -> Result<RArray, Error> {
        let cookies = Cookie::extract_headers_cookies(&rb_self.headers);
        let ary = ruby.ary_new_capa(cookies.len());
        for cookie in cookies {
            ary.push(cookie)?;
        }
        Ok(ary)
    }

    /// Get the response headers.
    #[inline]
    pub fn headers(&self) -> Headers {
        Headers::from(self.headers.clone())
    }

    /// Get the local socket address, if available.
    #[inline]
    pub fn local_addr(&self) -> Option<String> {
        self.local_addr.map(|addr| addr.to_string())
    }

    /// Get the remote socket address, if available.
    #[inline]
    pub fn remote_addr(&self) -> Option<String> {
        self.remote_addr.map(|addr| addr.to_string())
    }

    /// Get the response body as bytes.
    pub fn bytes(&self) -> Result<Bytes, Error> {
        let response = self.response(false)?;
        rt::try_block_on(response.bytes().map_err(wreq_error_to_magnus))
    }

    /// Get the response body as JSON.
    pub fn json(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let response = rb_self.response(false)?;
        rt::try_block_on(async move {
            let json = response
                .json::<Json>()
                .await
                .map_err(wreq_error_to_magnus)?;
            serde_magnus::serialize(ruby, &json)
        })
    }

    /// Get a chunk iterator for the response body.
    pub fn chunks(&self) -> Result<Yield<BodyReceiver>, Error> {
        self.response(true)
            .map(wreq::Response::bytes_stream)
            .map(BodyReceiver::new)
            .map(Yield::Iter)
    }

    /// Close the response body, dropping any resources.
    #[inline]
    pub fn close(&self) {
        gvl::nogvl(|| self.body.swap(None));
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        // Ensure body is dropped in GVL
        self.body.swap(None);
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let response_class = gem_module.define_class("Response", ruby.class_object())?;
    response_class.define_method("code", magnus::method!(Response::code, 0))?;
    response_class.define_method("status", magnus::method!(Response::status, 0))?;
    response_class.define_method("version", magnus::method!(Response::version, 0))?;
    response_class.define_method("url", magnus::method!(Response::url, 0))?;
    response_class.define_method(
        "content_length",
        magnus::method!(Response::content_length, 0),
    )?;
    response_class.define_method("cookies", magnus::method!(Response::cookies, 0))?;
    response_class.define_method("headers", magnus::method!(Response::headers, 0))?;
    response_class.define_method("local_addr", magnus::method!(Response::local_addr, 0))?;
    response_class.define_method("remote_addr", magnus::method!(Response::remote_addr, 0))?;
    response_class.define_method("text", magnus::method!(Response::bytes, 0))?;
    response_class.define_method("json", magnus::method!(Response::json, 0))?;
    response_class.define_method("chunks", magnus::method!(Response::chunks, 0))?;
    response_class.define_method("close", magnus::method!(Response::close, 0))?;
    Ok(())
}
