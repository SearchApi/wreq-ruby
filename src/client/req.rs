use std::net::IpAddr;

use ::serde::Deserialize;
use http::header;
use magnus::{RHash, TryConvert, typed_data::Obj, value::ReprValue};
use wreq::{Client, Proxy};

use super::body::{Body, form::Form, json::Json};
use crate::{
    arch::SUPPORTS_INTERFACE,
    client::{query::Query, resp::Response},
    cookie::Cookies,
    emulate::Emulation,
    error::wreq_error,
    extractor::Extractor,
    header::{Headers, OrigHeaders},
    http::{Method, Version},
    options::{NativeOption, Options},
    rt,
    time::Duration,
};

/// The parameters for a request.
#[derive(Default, Deserialize)]
#[non_exhaustive]
pub struct Request {
    /// The emulation option for the request.
    #[serde(default)]
    emulation: NativeOption<Emulation>,

    /// The proxy to use for the request.
    #[serde(default)]
    proxy: NativeOption<Proxy>,

    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,

    /// Bind to an interface by `SO_BINDTODEVICE`.
    #[allow(dead_code)]
    interface: Option<String>,

    /// Overall timeout for this request, overriding the client default.
    #[serde(default)]
    timeout: NativeOption<Duration>,

    /// Maximum idle duration between body reads, overriding the client default.
    #[serde(default)]
    read_timeout: NativeOption<Duration>,

    /// The HTTP version to use for the request.
    #[serde(default)]
    version: NativeOption<Version>,

    /// The option enables default headers.
    default_headers: Option<bool>,

    /// The headers to use for the request.
    #[serde(default)]
    headers: NativeOption<Headers>,

    /// The original headers to use for the request.
    #[serde(default)]
    orig_headers: NativeOption<OrigHeaders>,

    /// The cookies to use for the request.
    #[serde(default)]
    cookies: NativeOption<Cookies>,

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
    query: Option<Query>,

    /// The form parameters to use for the request.
    form: Option<Form>,

    /// The JSON body to use for the request.
    #[serde(default)]
    json: NativeOption<Json>,

    /// The body to use for the request.
    #[serde(default)]
    body: NativeOption<Body>,
}

impl Request {
    /// Create a new [`Request`] from Ruby keyword arguments.
    ///
    /// # Errors
    ///
    /// Returns before network I/O for unknown, duplicate, unsupported,
    /// conflicting, ineffective, or invalid option values.
    pub fn new(ruby: &magnus::Ruby, hash: RHash) -> Result<Self, magnus::Error> {
        let keyword = hash.as_value();
        let options = Options::new(ruby, hash);
        let mut builder = Self::deserialize_options(&options)?;
        options
            .validator()
            .require_when_present(
                stringify!(max_redirects),
                builder.max_redirects.is_some(),
                builder.allow_redirects == Some(true),
                ":allow_redirects to be true",
            )
            .finish()?;

        extract_native_option!(
            options,
            builder,
            emulation,
            Obj<Emulation> => |value| (*value).clone()
        );
        extract_native_option!(options, builder, version);
        extract_native_option!(options, builder, timeout);
        extract_native_option!(options, builder, read_timeout);
        extract_native_option!(options, builder, headers);
        extract_native_option!(options, builder, orig_headers);
        extract_native_option!(options, builder, cookies);
        extract_native_option!(options, builder, json, present);
        builder
            .proxy
            .set(Extractor::<Proxy>::try_convert(keyword)?.into_inner());
        extract_native_option!(options, builder, body);

        Ok(builder)
    }

    /// Validate pre-conversion rules and deserialize the request options.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for failed rules or the Ruby conversion error
    /// produced by an invalid option value.
    fn deserialize_options(options: &Options<'_>) -> Result<Self, magnus::Error> {
        options
            .validate_keys::<Self>()?
            .validator()
            .reject_unsupported(stringify!(interface), SUPPORTS_INTERFACE)
            .reject_conflicts([
                (stringify!(body), options.is_non_nil(stringify!(body))),
                (stringify!(form), options.is_non_nil(stringify!(form))),
                (stringify!(json), options.is_present(stringify!(json))),
            ])
            .reject_conflicts([
                (stringify!(auth), options.is_non_nil(stringify!(auth))),
                (
                    stringify!(bearer_auth),
                    options.is_non_nil(stringify!(bearer_auth)),
                ),
                (
                    stringify!(basic_auth),
                    options.is_non_nil(stringify!(basic_auth)),
                ),
            ])
            .finish()?
            .deserialize::<Self>()
    }
}

pub fn execute_request<U: AsRef<str>>(
    ruby: &magnus::Ruby,
    client: Client,
    method: Method,
    url: U,
    mut request: Request,
) -> Result<Response, magnus::Error> {
    rt::try_block_on(
        ruby,
        async move {
            let mut builder = client.request(method.into_ffi(), url.as_ref());

            // Emulation options.
            apply_option!(set_if_some_inner, builder, request.emulation, emulation);

            // Version options.
            apply_option!(
                set_if_some_map,
                builder,
                request.version,
                version,
                Version::into_ffi
            );

            // Timeout options.
            apply_option!(set_if_some_inner, builder, request.timeout, timeout);
            apply_option!(
                set_if_some_inner,
                builder,
                request.read_timeout,
                read_timeout
            );

            // Network options.
            apply_option!(set_if_some, builder, request.proxy, proxy);
            apply_option!(set_if_some, builder, request.local_address, local_address);
            #[cfg(any(
                target_os = "android",
                target_os = "fuchsia",
                target_os = "illumos",
                target_os = "ios",
                target_os = "linux",
                target_os = "macos",
                target_os = "solaris",
                target_os = "tvos",
                target_os = "visionos",
                target_os = "watchos",
            ))]
            apply_option!(set_if_some, builder, request.interface, interface);

            // Headers options.
            apply_option!(set_if_some_into_inner, builder, request.headers, headers);
            apply_option!(
                set_if_some_inner,
                builder,
                request.orig_headers,
                orig_headers
            );
            apply_option!(
                set_if_some,
                builder,
                request.default_headers,
                default_headers
            );

            // Cookies options.
            if let Some(cookies) = request.cookies.take() {
                for cookie in cookies.0 {
                    builder = builder.header(header::COOKIE, cookie);
                }
            }

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
            builder.send().await.map(Response::new)
        },
        wreq_error,
    )
}
