use std::{net::IpAddr, time::Duration};

use http::{HeaderValue, header};
use indexmap::IndexMap;
use magnus::{RHash, TryConvert, typed_data::Obj, value::ReprValue};
use serde::Deserialize;
use wreq::{
    Client, Proxy, Version,
    header::{HeaderMap, OrigHeaderMap},
};

use super::body::{Body, Json};
use crate::{
    client::resp::Response, emulation::Emulation, error::wreq_error_to_magnus,
    extractor::Extractor, http::Method, rt,
};

/// The parameters for a request.
#[derive(Default, Deserialize)]
#[non_exhaustive]
pub struct Request {
    /// The emulation option for the request.
    #[serde(skip)]
    emulation: Option<Emulation>,

    /// The proxy to use for the request.
    #[serde(skip)]
    proxy: Option<Proxy>,

    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    interface: Option<String>,

    /// The timeout to use for the request.
    timeout: Option<u64>,

    /// The read timeout to use for the request.
    read_timeout: Option<u64>,

    /// The HTTP version to use for the request.
    #[serde(skip)]
    version: Option<Version>,

    /// The headers to use for the request.
    #[serde(skip)]
    headers: Option<HeaderMap>,

    /// The original headers to use for the request.
    #[serde(skip)]
    orig_headers: Option<OrigHeaderMap>,

    /// The option enables default headers.
    default_headers: Option<bool>,

    /// The cookies to use for the request.
    #[serde(skip)]
    cookies: Option<Vec<HeaderValue>>,

    /// Whether to allow redirects.
    allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    max_redirects: Option<usize>,

    /// Sets gzip as an accepted encoding.
    gzip: Option<bool>,

    /// Sets brotli as an accepted encoding.
    brotli: Option<bool>,

    /// Sets deflate as an accepted encoding.
    deflate: Option<bool>,

    /// Sets zstd as an accepted encoding.
    zstd: Option<bool>,

    /// The authentication to use for the request.
    auth: Option<String>,

    /// The bearer authentication to use for the request.
    bearer_auth: Option<String>,

    /// The basic authentication to use for the request.
    basic_auth: Option<(String, Option<String>)>,

    /// The query parameters to use for the request.
    query: Option<IndexMap<String, String>>,

    /// The form parameters to use for the request.
    form: Option<IndexMap<String, String>>,

    /// The JSON body to use for the request.
    json: Option<Json>,

    /// The body to use for the request.
    #[serde(skip)]
    body: Option<Body>,
}

impl Request {
    /// Create a new [`Request`] from Ruby keyword arguments.
    pub fn new(ruby: &magnus::Ruby, hash: RHash) -> Result<Self, magnus::Error> {
        let kwargs = hash.as_value();
        let mut builder: Self = serde_magnus::deserialize(ruby, kwargs)?;

        // extra emulation handling
        if let Some(v) = hash.get(ruby.to_symbol("emulation")) {
            let emulation_obj = Obj::<Emulation>::try_convert(v)?;
            builder.emulation = Some((*emulation_obj).clone());
        }

        // extra version handling
        builder.version = Extractor::<Version>::try_convert(kwargs)?.into_inner();

        // extra headers handling
        builder.headers = Extractor::<HeaderMap>::try_convert(kwargs)?.into_inner();

        // extra original headers handling
        builder.orig_headers = Extractor::<OrigHeaderMap>::try_convert(kwargs)?.into_inner();

        // extra cookies handling
        builder.cookies = Extractor::<Vec<HeaderValue>>::try_convert(kwargs)?.into_inner();

        // extra proxy handling
        builder.proxy = Extractor::<Proxy>::try_convert(kwargs)?.into_inner();

        // extra body handling
        if let Some(body) = hash.get(ruby.to_symbol("body")) {
            builder.body = Some(Body::try_convert(body)?);
        }

        Ok(builder)
    }
}

pub fn execute_request<U: AsRef<str>>(
    client: Client,
    method: Method,
    url: U,
    mut request: Request,
) -> Result<Response, magnus::Error> {
    rt::try_block_on(async move {
        let mut builder = client.request(method.into_ffi(), url.as_ref());

        // Emulation options.
        apply_option!(set_if_some_inner, builder, request.emulation, emulation);

        // Version options.
        apply_option!(set_if_some, builder, request.version, version);

        // Timeout options.
        apply_option!(
            set_if_some_map,
            builder,
            request.timeout,
            timeout,
            Duration::from_secs
        );
        apply_option!(
            set_if_some_map,
            builder,
            request.read_timeout,
            read_timeout,
            Duration::from_secs
        );

        // Network options.
        apply_option!(set_if_some, builder, request.proxy, proxy);
        apply_option!(set_if_some, builder, request.local_address, local_address);
        apply_option!(set_if_some, builder, request.interface, interface);

        // Headers options.
        apply_option!(set_if_some, builder, request.headers, headers);
        apply_option!(set_if_some, builder, request.orig_headers, orig_headers);
        apply_option!(
            set_if_some,
            builder,
            request.default_headers,
            default_headers
        );

        // Authentication options.
        apply_option!(
            set_if_some_map_ref,
            builder,
            request.auth,
            auth,
            AsRef::<str>::as_ref
        );
        apply_option!(set_if_some, builder, request.bearer_auth, bearer_auth);
        if let Some(basic_auth) = request.basic_auth.take() {
            builder = builder.basic_auth(basic_auth.0, basic_auth.1);
        }

        // Cookies options.
        if let Some(cookies) = request.cookies.take() {
            for cookie in cookies {
                builder = builder.header_append(header::COOKIE, cookie);
            }
        }

        // Allow redirects options.
        match request.allow_redirects {
            Some(false) => {
                builder = builder.redirect(wreq::redirect::Policy::none());
            }
            Some(true) => {
                builder = builder.redirect(
                    request
                        .max_redirects
                        .take()
                        .map(wreq::redirect::Policy::limited)
                        .unwrap_or_default(),
                );
            }
            None => {}
        };

        // Compression options.
        apply_option!(set_if_some, builder, request.gzip, gzip);
        apply_option!(set_if_some, builder, request.brotli, brotli);
        apply_option!(set_if_some, builder, request.deflate, deflate);
        apply_option!(set_if_some, builder, request.zstd, zstd);

        // Query options.
        apply_option!(set_if_some_ref, builder, request.query, query);

        // Form options.
        apply_option!(set_if_some_ref, builder, request.form, form);

        // JSON options.
        apply_option!(set_if_some_ref, builder, request.json, json);

        // Body options.
        if let Some(body) = request.body.take() {
            builder = builder.body(wreq::Body::from(body));
        }

        // Send request.
        builder
            .send()
            .await
            .map(Response::new)
            .map_err(wreq_error_to_magnus)
    })
}
