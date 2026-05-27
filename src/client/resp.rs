use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::{StreamExt, TryFutureExt};
use http::{Extensions, HeaderMap, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{
    Error, IntoValue, Module, RArray, RModule, Ruby, Value, block::Proc, scan_args::scan_args,
    value::ReprValue,
};
use wreq::Uri;

use crate::{
    client::body::Json,
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

    ///  Get the full response text given a specific encoding.
    pub fn text(&self, args: &[Value]) -> Result<String, Error> {
        let args = scan_args::<(), (Option<String>,), (), (), (), ()>(args)?;
        let response = self.response(false)?;
        match args.optional.0 {
            Some(encoding) => rt::try_block_on(
                response
                    .text_with_charset(encoding)
                    .map_err(wreq_error_to_magnus),
            ),
            None => rt::try_block_on(response.text().map_err(wreq_error_to_magnus)),
        }
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

    /// Stream the response body, yielding each chunk to the given block with
    /// proper GVL management.
    ///
    /// The iteration loop is driven from Rust:
    /// 1. GVL is released while waiting for the next chunk (network I/O)
    /// 2. GVL is re-acquired to yield the chunk to the Ruby block
    /// 3. GVL is released again for the next I/O operation
    ///
    /// This allows other Ruby threads to run during network I/O, and ensures
    /// streaming errors are properly propagated instead of silently swallowed.
    pub fn chunks(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        // Check for a block argument using the Ruby C API.
        // rb_block_given_p() returns c_int: 1 if block given, 0 otherwise.
        if unsafe { rb_sys::rb_block_given_p() == 0 } {
            return Err(Error::new(
                ruby.exception_local_jump_error(),
                "no block given (yield)",
            ));
        }

        // Heap-allocate the block VALUE for a stable address that can be
        // registered with Ruby's GC. This prevents the Proc from being
        // collected while the GVL is released during I/O.
        let mut block_raw = Box::new(unsafe { rb_sys::rb_block_proc() });
        let block_ptr: *mut rb_sys::VALUE = block_raw.as_mut();

        unsafe {
            rb_sys::rb_gc_register_address(block_ptr);
        }

        let response = rb_self.response(true)?;
        let stream = response.bytes_stream();

        // Drive the streaming loop inside a single nogvl_cancellable call,
        // using with_gvl to re-acquire the GVL only for Ruby block yields.
        let result = gvl::nogvl_cancellable(|flag| {
            rt::runtime().block_on(async move {
                let mut stream = Box::pin(stream);
                loop {
                    let chunk = tokio::select! {
                        biased;
                        _ = flag.cancelled() => return Err(crate::error::interrupt_error()),
                        result = stream.next() => result,
                    };

                    match chunk {
                        Some(Ok(bytes)) => {
                            // Read the current VALUE (GC compaction may have
                            // updated it via the registered address).
                            let current_block_raw = unsafe { *block_ptr };
                            // Re-acquire GVL to yield chunk to the Ruby block.
                            // Wrap in block_in_place to tell Tokio this thread
                            // will block, so it can schedule other tasks.
                            let yield_result: Result<(), Error> =
                                tokio::task::block_in_place(|| {
                                    gvl::with_gvl(|| {
                                        let block_value = unsafe {
                                            magnus::rb_sys::FromRawValue::from_raw(
                                                current_block_raw,
                                            )
                                        };
                                        let block =
                                            Proc::from_value(block_value).ok_or_else(|| {
                                                Error::new(
                                                    ruby.exception_runtime_error(),
                                                    "block was garbage collected",
                                                )
                                            })?;
                                        let chunk_value = bytes.into_value_with(ruby);
                                        block.call::<_, Value>((chunk_value,))?;
                                        Ok(())
                                    })
                                });
                            yield_result?;
                        }
                        Some(Err(e)) => return Err(wreq_error_to_magnus(e)),
                        None => return Ok(()),
                    }
                }
            })
        });

        // Unregister from GC now that we're done with the block
        unsafe {
            rb_sys::rb_gc_unregister_address(block_ptr);
        }

        result?;
        Ok(ruby.qnil().as_value())
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
    response_class.define_method("bytes", magnus::method!(Response::bytes, 0))?;
    response_class.define_method("text", magnus::method!(Response::text, -1))?;
    response_class.define_method("json", magnus::method!(Response::json, 0))?;
    response_class.define_method("chunks", magnus::method!(Response::chunks, 0))?;
    response_class.define_method("close", magnus::method!(Response::close, 0))?;
    Ok(())
}
