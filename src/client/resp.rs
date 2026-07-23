use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, HeaderMap, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{Error, Module, RArray, RModule, RString, Ruby, Value, scan_args::scan_args};
use wreq::Uri;
use wreq::tls::TlsInfo as WreqTlsInfo;
use magnus::value::ReprValue;

use crate::{
    arch::ProcessLocal,
    client::body::{json::Json, stream::BodyReceiver},
    cookie::Cookie,
    error::{memory_error, no_block_given_error, wreq_error},
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
    state: ProcessLocal<NativeResponseState>,
}

/// Represents the state of the HTTP response body.
enum Body {
    /// The body can be streamed once (not yet buffered).
    Streamable(wreq::Body),
    /// The body has been fully read into memory and can be reused.
    Reusable(Bytes),
}

/// Response state that may contain handles owned by the native runtime.
struct NativeResponseState {
    body: ArcSwapOption<Body>,
    extensions: Extensions,
}

/// TLS certificate information extracted from a response.
#[magnus::wrap(class = "Wreq::TlsInfo", free_immediately, size)]
struct TlsInfo {
    peer_certificate: Option<Vec<u8>>,
    peer_certificate_chain: Option<Vec<Vec<u8>>>,
}

impl TlsInfo {
    /// Get the DER-encoded leaf certificate of the peer as a binary Ruby String.
    fn peer_certificate(ruby: &Ruby, rb_self: &Self) -> Option<RString> {
        rb_self.peer_certificate.as_ref().map(|der| {
            ruby.str_from_slice(der)
        })
    }
    /// Get the full certificate chain as a frozen Array of binary Ruby Strings.
    fn peer_certificate_chain(ruby: &Ruby, rb_self: &Self) -> Option<RArray> {
        rb_self.peer_certificate_chain.as_ref().map(|chain| {
            let ary = ruby.ary_new_capa(chain.len());
            for cert in chain {
                let _ = ary.push(ruby.str_from_slice(cert));
            }
            let _: Result<Value, Error> = ary.funcall("freeze", ());
            ary
        })
    }

    fn inspect(&self) -> String {
        let cert_info = match &self.peer_certificate {
            Some(der) => format!("peer_certificate=({} bytes)", der.len()),
            None => "peer_certificate=nil".to_owned(),
        };
        let chain_info = match &self.peer_certificate_chain {
            Some(chain) => format!("peer_certificate_chain=({} certs)", chain.len()),
            None => "peer_certificate_chain=nil".to_owned(),
        };
        format!("#<Wreq::TlsInfo {cert_info} {chain_info}>")
    }
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

        Response {
            uri,
            local_addr,
            remote_addr,
            content_length,
            version: Version::from_ffi(parts.version),
            status: StatusCode::from(parts.status),
            headers: parts.headers,
            state: ProcessLocal::new(NativeResponseState {
                body: ArcSwapOption::from_pointee(Body::Streamable(body)),
                extensions: parts.extensions,
            }),
        }
    }

    /// Internal method to get the wreq::Response, optionally streaming the body.
    fn response(&self, ruby: &Ruby, stream: bool) -> Result<wreq::Response, Error> {
        rt::ensure_current(ruby)?;
        let state = self.state.as_ref();

        let build_response = |body: wreq::Body| -> wreq::Response {
            let mut response = HttpResponse::new(body);
            *response.version_mut() = self.version.into_ffi();
            *response.status_mut() = self.status.0;
            *response.headers_mut() = self.headers.clone();
            *response.extensions_mut() = state.extensions.clone();
            wreq::Response::from(response)
        };

        if let Some(arc) = state.body.swap(None) {
            match Arc::try_unwrap(arc) {
                Ok(Body::Streamable(body)) => {
                    return if stream {
                        Ok(build_response(body))
                    } else {
                        let bytes = rt::try_block_on(
                            ruby,
                            BodyExt::collect(body).map_ok(|buf| buf.to_bytes()),
                            wreq_error,
                        )?;

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

    /// Get TLS certificate information, if available.
    fn tls_info(&self) -> Option<TlsInfo> {
        self.extensions.get::<WreqTlsInfo>().map(|info| {
            TlsInfo {
                peer_certificate: info.peer_certificate().map(|der| der.to_vec()),
                peer_certificate_chain: info
                    .peer_certificate_chain()
                    .map(|chain| chain.map(|cert| cert.to_vec()).collect()),
            }
        })
    }

    /// Get the response body as bytes.
    pub fn bytes(ruby: &Ruby, rb_self: &Self) -> Result<Bytes, Error> {
        let response = rb_self.response(ruby, false)?;
        rt::try_block_on(ruby, response.bytes(), wreq_error)
    }

    ///  Get the full response text given a specific encoding.
    pub fn text(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<String, Error> {
        rt::ensure_current(ruby)?;
        let args = scan_args::<(), (Option<String>,), (), (), (), ()>(args)?;
        let response = rb_self.response(ruby, false)?;
        match args.optional.0 {
            Some(encoding) => {
                rt::try_block_on(ruby, response.text_with_charset(encoding), wreq_error)
            }
            None => rt::try_block_on(ruby, response.text(), wreq_error),
        }
    }

    /// Get the response body as JSON.
    pub fn json(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let response = rb_self.response(ruby, false)?;
        let json = rt::try_block_on(ruby, response.json::<Json>(), wreq_error)?;
        crate::serde::serialize(ruby, &json)
    }

    /// Yield response body chunks to the given Ruby block.
    pub fn chunks(ruby: &Ruby, rb_self: &Self) -> Result<(), Error> {
        rt::ensure_current(ruby)?;

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
    /// Returns `Wreq::ForkError` before touching a body inherited from the
    /// parent process.
    pub fn close(ruby: &Ruby, rb_self: &Self) -> Result<(), Error> {
        rt::ensure_current(ruby)?;
        gvl::nogvl(|| rb_self.state.as_ref().body.swap(None));
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

    let tls_info_class = gem_module.define_class("TlsInfo", ruby.class_object())?;
    tls_info_class.define_method(
        "peer_certificate",
        magnus::method!(TlsInfo::peer_certificate, 0),
    )?;
    tls_info_class.define_method(
        "peer_certificate_chain",
        magnus::method!(TlsInfo::peer_certificate_chain, 0),
    )?;
    tls_info_class.define_method("inspect", magnus::method!(TlsInfo::inspect, 0))?;
    tls_info_class.define_method("to_s", magnus::method!(TlsInfo::inspect, 0))?;
    Ok(())
}
