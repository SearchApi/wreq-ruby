use std::{net::SocketAddr, sync::Arc};

use arc_swap::ArcSwapOption;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http::{Extensions, HeaderMap, response::Response as HttpResponse};
use http_body_util::BodyExt;
use magnus::{
    Error, Module, RArray, RHash, RModule, Ruby, Value, scan_args::scan_args, value::ReprValue,
};
use wreq::Uri;
use wreq::redirect::History as WreqHistory;

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

/// A single redirect hop extracted from the response's redirect history.
#[magnus::wrap(class = "Wreq::RedirectHistoryEntry", free_immediately, size)]
struct RedirectHistoryEntry {
    status: u16,
    url: String,
    previous_url: String,
    headers: HeaderMap,
}

impl RedirectHistoryEntry {
    fn status(&self) -> u16 {
        self.status
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn previous_url(&self) -> &str {
        &self.previous_url
    }

    fn headers(&self) -> Headers {
        Headers::from(self.headers.clone())
    }

    fn to_h(ruby: &Ruby, rb_self: &Self) -> RHash {
        let hash = ruby.hash_new();
        let _ = hash.aset(ruby.to_symbol("status"), rb_self.status);
        let _ = hash.aset(ruby.to_symbol("url"), rb_self.url.as_str());
        let _ = hash.aset(
            ruby.to_symbol("previous_url"),
            rb_self.previous_url.as_str(),
        );
        let _ = hash.aset(ruby.to_symbol("headers"), rb_self.headers());
        hash
    }

    fn inspect(&self) -> String {
        format!(
            "#<Wreq::RedirectHistoryEntry {} {} -> {}>",
            self.status,
            redact_url(&self.previous_url),
            redact_url(&self.url)
        )
    }
}

/// Redact query string and userinfo from a URL for safe display.
fn redact_url(url: &str) -> String {
    match Uri::try_from(url) {
        Ok(uri) => {
            let mut result = String::new();
            if let Some(scheme) = uri.scheme_str() {
                result.push_str(scheme);
                result.push_str("://");
            }
            if let Some(host) = uri.host() {
                result.push_str(host);
            }
            if let Some(port) = uri.port() {
                result.push(':');
                result.push_str(&port.to_string());
            }
            result.push_str(uri.path());
            if uri.query().is_some() {
                result.push_str("?[REDACTED]");
            }
            result
        }
        Err(_) => "[invalid URI]".to_owned(),
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

    /// Get the redirect history as a frozen array of RedirectHistoryEntry values.
    fn history(ruby: &Ruby, rb_self: &Self) -> RArray {
        let state = rb_self.state.as_ref();
        let entries = state.extensions.get::<WreqHistory>();

        match entries {
            Some(history) => {
                let items: Vec<RedirectHistoryEntry> = history
                    .into_iter()
                    .map(|entry| RedirectHistoryEntry {
                        status: entry.status.as_u16(),
                        url: entry.uri.to_string(),
                        previous_url: entry.previous.to_string(),
                        headers: entry.headers.clone(),
                    })
                    .collect();

                let ary = ruby.ary_new_capa(items.len());
                for item in items {
                    let _ = ary.push(item);
                }
                let _: Result<Value, Error> = ary.funcall("freeze", ());
                ary
            }
            None => {
                let ary = ruby.ary_new();
                let _: Result<Value, Error> = ary.funcall("freeze", ());
                ary
            }
        }
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
    response.define_method("history", magnus::method!(Response::history, 0))?;

    let entry_class = gem_module.define_class("RedirectHistoryEntry", ruby.class_object())?;
    entry_class.define_method("status", magnus::method!(RedirectHistoryEntry::status, 0))?;
    entry_class.define_method("url", magnus::method!(RedirectHistoryEntry::url, 0))?;
    entry_class.define_method(
        "previous_url",
        magnus::method!(RedirectHistoryEntry::previous_url, 0),
    )?;
    entry_class.define_method("headers", magnus::method!(RedirectHistoryEntry::headers, 0))?;
    entry_class.define_method("to_h", magnus::method!(RedirectHistoryEntry::to_h, 0))?;
    entry_class.define_method("inspect", magnus::method!(RedirectHistoryEntry::inspect, 0))?;
    entry_class.define_method("to_s", magnus::method!(RedirectHistoryEntry::inspect, 0))?;
    Ok(())
}
