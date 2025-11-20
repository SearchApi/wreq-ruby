use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use http::{Extensions, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{Error, Module, RModule, Ruby};
use wreq::{Uri, header::HeaderMap};

use crate::{
    RUNTIME,
    error::{memory_error, wreq_error_to_magnus},
    http::{StatusCode, Version},
    nogvl,
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

    fn response(&self, stream: bool) -> Result<wreq::Response, Error> {
        nogvl::nogvl(|| {
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
                            let bytes = RUNTIME
                                .block_on(BodyExt::collect(body))
                                .map(|buf| buf.to_bytes())
                                .map_err(wreq_error_to_magnus)?;

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
        })
    }
}

impl Response {
    /// Get the response status code as a u16.
    pub fn code(&self) -> u16 {
        self.status.0.as_u16()
    }

    /// Get the response status code.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the response HTTP version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Get the response URI.
    pub fn uri(&self) -> String {
        self.uri.to_string()
    }

    /// Get the content length of the response, if known.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Get the local socket address, if available.
    pub fn local_addr(&self) -> Option<String> {
        self.local_addr.map(|addr| addr.to_string())
    }

    /// Get the remote socket address, if available.
    pub fn remote_addr(&self) -> Option<String> {
        self.remote_addr.map(|addr| addr.to_string())
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    gem_module.define_class("Response", ruby.class_object())?;
    gem_module.define_method("code", magnus::method!(Response::code, 0))?;
    gem_module.define_method("status", magnus::method!(Response::status, 0))?;
    gem_module.define_method("version", magnus::method!(Response::version, 0))?;
    gem_module.define_method("uri", magnus::method!(Response::uri, 0))?;
    gem_module.define_method(
        "content_length",
        magnus::method!(Response::content_length, 0),
    )?;
    gem_module.define_method("local_addr", magnus::method!(Response::local_addr, 0))?;
    gem_module.define_method("remote_addr", magnus::method!(Response::remote_addr, 0))?;
    Ok(())
}
