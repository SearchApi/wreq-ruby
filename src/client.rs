mod body;
mod param;
mod query;
mod req;
pub mod resp;

use std::{net::IpAddr, time::Duration};

use ::serde::Deserialize;
use magnus::{Module, Object, RModule, Ruby, TryConvert, Value, function, method, typed_data::Obj};
use wreq::Proxy;

use crate::{
    arch::{ProcessLocal, SUPPORTS_INTERFACE, SUPPORTS_TCP_USER_TIMEOUT},
    client::{req::execute_request, resp::Response},
    cookie::Jar,
    emulate::Emulation,
    error::wreq_error,
    extractor::Extractor,
    gvl,
    header::{Headers, OrigHeaders, UserAgent},
    http::Method,
    options::{NativeOption, Options},
};

/// A builder for `Client`.
#[derive(Default, Deserialize)]
struct Builder {
    // The emulation option for the client.
    #[serde(default)]
    emulation: NativeOption<Emulation>,
    /// The user agent to use for the client.
    #[serde(default)]
    user_agent: NativeOption<UserAgent>,
    /// The headers to use for the client.
    #[serde(default)]
    headers: NativeOption<Headers>,
    /// The original headers to use for the client.
    #[serde(default)]
    orig_headers: NativeOption<OrigHeaders>,
    /// Whether to use referer.
    referer: Option<bool>,
    /// Whether to allow redirects.
    allow_redirects: Option<bool>,
    /// The maximum number of redirects to follow.
    max_redirects: Option<usize>,

    // ========= Cookie options =========
    /// Whether to use cookie store.
    cookie_store: Option<bool>,
    /// Whether to use cookie store provider.
    #[serde(default)]
    cookie_provider: NativeOption<Obj<Jar>>,

    // ========= Timeout options =========
    /// The timeout to use for the client. (in seconds)
    timeout: Option<u64>,
    /// The connect timeout to use for the client. (in seconds)
    connect_timeout: Option<u64>,
    /// The read timeout to use for the client. (in seconds)
    read_timeout: Option<u64>,

    // ========= TCP options =========
    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration. (in seconds)
    tcp_keepalive: Option<u64>,
    /// Set the interval between TCP keepalive probes. (in seconds)
    tcp_keepalive_interval: Option<u64>,
    /// Set the number of retries for TCP keepalive.
    tcp_keepalive_retries: Option<u32>,
    /// Set an optional user timeout for TCP sockets. (in seconds)
    #[allow(dead_code)]
    tcp_user_timeout: Option<u64>,
    /// Set that all sockets have `NO_DELAY` set.
    tcp_nodelay: Option<bool>,
    /// Set that all sockets have `SO_REUSEADDR` set.
    tcp_reuse_address: Option<bool>,

    // ========= Connection pool options =========
    /// Set an optional timeout for idle sockets being kept-alive. (in seconds)
    pool_idle_timeout: Option<u64>,
    /// Sets the maximum idle connection per host allowed in the pool.
    pool_max_idle_per_host: Option<usize>,
    /// Sets the maximum number of connections in the pool.
    pool_max_size: Option<usize>,

    // ========= Protocol options =========
    /// Whether to use the HTTP/1 protocol only.
    http1_only: Option<bool>,
    /// Whether to use the HTTP/2 protocol only.
    http2_only: Option<bool>,
    /// Whether to use HTTPS only.
    https_only: Option<bool>,

    // ========= TLS options =========
    /// Whether to verify TLS certificates.
    verify: Option<bool>,
    /// Whether to retain peer certificate data on responses.
    tls_info: Option<bool>,

    // ========= Network options =========
    /// Whether to disable the proxy for the client.
    no_proxy: Option<bool>,
    /// The proxy to use for the client.
    #[serde(default)]
    proxy: NativeOption<Proxy>,
    /// Bind to a local IP Address.
    local_address: Option<IpAddr>,
    /// Bind to an interface by `SO_BINDTODEVICE`.
    #[allow(dead_code)]
    interface: Option<String>,

    // ========= Compression options =========
    /// Sets gzip as an accepted encoding.
    gzip: Option<bool>,
    /// Sets brotli as an accepted encoding.
    brotli: Option<bool>,
    /// Sets deflate as an accepted encoding.
    deflate: Option<bool>,
    /// Sets zstd as an accepted encoding.
    zstd: Option<bool>,
}

#[magnus::wrap(class = "Wreq::Client", free_immediately, size)]
pub struct Client(ProcessLocal<wreq::Client>);

// ===== impl Builder =====

impl Builder {
    /// Create a new [`Builder`] from a Ruby options Hash.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for unknown, duplicate, conflicting,
    /// ineffective, or platform-specific options. Known values retain their
    /// Ruby conversion error class and include the option name.
    fn from_options(options: Options<'_>) -> Result<Self, magnus::Error> {
        let options = options.validate_keys::<Self>()?;
        let mut builder = options
            .validator()
            .reject_unsupported(stringify!(tcp_user_timeout), SUPPORTS_TCP_USER_TIMEOUT)
            .reject_unsupported(stringify!(interface), SUPPORTS_INTERFACE)
            .finish()?
            .deserialize::<Self>()?;

        options
            .validator()
            .reject_conflicts([
                (stringify!(http1_only), builder.http1_only == Some(true)),
                (stringify!(http2_only), builder.http2_only == Some(true)),
            ])
            .reject_conflicts([
                (stringify!(proxy), options.is_non_nil(stringify!(proxy))),
                (stringify!(no_proxy), builder.no_proxy == Some(true)),
            ])
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
        extract_native_option!(options, builder, user_agent);
        extract_native_option!(options, builder, headers);
        extract_native_option!(options, builder, orig_headers);
        extract_native_option!(options, builder, cookie_provider);
        builder
            .proxy
            .set(Extractor::<Proxy>::try_convert(options.as_value())?.into_inner());

        Ok(builder)
    }
}

// ===== impl Client =====

impl Client {
    /// Create a new [`Client`] with the given keyword arguments.
    ///
    /// # Errors
    ///
    /// Returns Ruby configuration errors from [`Builder::from_options`] or the
    /// native fallible client builder. An inherited cookie provider returns
    /// `Wreq::ForkError`, and extra positional arguments return `ArgumentError`.
    pub fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, magnus::Error> {
        Options::from_args(ruby, args, "client")?
            .map(Builder::from_options)
            .transpose()
            .map(Option::unwrap_or_default)
            .and_then(|params| Self::build(ruby, params))
            .map(ProcessLocal::new)
            .map(Self)
    }

    /// Build the default client through the same fallible path as `new`.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::BuilderError`, `Wreq::TlsError`, or another mapped native
    /// initialization error without unwinding through Ruby.
    pub(crate) fn default_client(ruby: &Ruby) -> Result<wreq::Client, magnus::Error> {
        Self::build(ruby, Builder::default())
    }

    /// Apply validated parameters and build the native client without the GVL.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` if the cookie provider belongs to a parent
    /// process. Native build failures are mapped only after the GVL has been
    /// reacquired.
    fn build(ruby: &Ruby, mut params: Builder) -> Result<wreq::Client, magnus::Error> {
        let mut cookie_provider = params
            .cookie_provider
            .take()
            .map(|jar| jar.clone_store(ruby))
            .transpose()?;
        let result = gvl::nogvl(|| {
            let mut builder = wreq::Client::builder();

            // Emulation options.
            apply_option!(set_if_some_inner, builder, params.emulation, emulation);

            // User agent options.
            apply_option!(set_if_some_inner, builder, params.user_agent, user_agent);

            // Headers options.
            apply_option!(
                set_if_some_into_inner,
                builder,
                params.headers,
                default_headers
            );
            apply_option!(
                set_if_some_inner,
                builder,
                params.orig_headers,
                orig_headers
            );

            // Allow redirects options.
            apply_option!(set_if_some, builder, params.referer, referer);
            match params.allow_redirects {
                Some(false) => {
                    builder = builder.redirect(wreq::redirect::Policy::none());
                }
                Some(true) => {
                    builder = builder.redirect(
                        params
                            .max_redirects
                            .take()
                            .map(wreq::redirect::Policy::limited)
                            .unwrap_or_default(),
                    );
                }
                None => {}
            }

            // Cookie options.
            apply_option!(set_if_some, builder, params.cookie_store, cookie_store);
            apply_option!(set_if_some, builder, cookie_provider, cookie_provider);

            // TCP options.
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_keepalive,
                tcp_keepalive,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_keepalive_interval,
                tcp_keepalive_interval,
                Duration::from_secs
            );
            apply_option!(
                set_if_some,
                builder,
                params.tcp_keepalive_retries,
                tcp_keepalive_retries
            );
            #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
            apply_option!(
                set_if_some_map,
                builder,
                params.tcp_user_timeout,
                tcp_user_timeout,
                Duration::from_secs
            );
            apply_option!(set_if_some, builder, params.tcp_nodelay, tcp_nodelay);
            apply_option!(
                set_if_some,
                builder,
                params.tcp_reuse_address,
                tcp_reuse_address
            );

            // Timeout options.
            apply_option!(
                set_if_some_map,
                builder,
                params.timeout,
                timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.connect_timeout,
                connect_timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some_map,
                builder,
                params.read_timeout,
                read_timeout,
                Duration::from_secs
            );

            // Pool options.
            apply_option!(
                set_if_some_map,
                builder,
                params.pool_idle_timeout,
                pool_idle_timeout,
                Duration::from_secs
            );
            apply_option!(
                set_if_some,
                builder,
                params.pool_max_idle_per_host,
                pool_max_idle_per_host
            );
            apply_option!(set_if_some, builder, params.pool_max_size, pool_max_size);

            // Protocol options.
            apply_option!(set_if_true, builder, params.http1_only, http1_only, false);
            apply_option!(set_if_true, builder, params.http2_only, http2_only, false);
            apply_option!(set_if_some, builder, params.https_only, https_only);

            // TLS options.
            apply_option!(set_if_some, builder, params.verify, tls_cert_verification);
            apply_option!(set_if_some, builder, params.tls_info, tls_info);

            // Network options.
            apply_option!(set_if_some, builder, params.proxy, proxy);
            apply_option!(set_if_true, builder, params.no_proxy, no_proxy, false);
            apply_option!(set_if_some, builder, params.local_address, local_address);
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
            apply_option!(set_if_some, builder, params.interface, interface);

            // Compression options.
            apply_option!(set_if_some, builder, params.gzip, gzip);
            apply_option!(set_if_some, builder, params.brotli, brotli);
            apply_option!(set_if_some, builder, params.deflate, deflate);
            apply_option!(set_if_some, builder, params.zstd, zstd);

            builder.build()
        });

        // Ruby exceptions must be created after the GVL has been reacquired.
        result.map_err(|err| wreq_error(ruby, err))
    }

    /// Clone the native client handle in the process that created it.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the client was inherited from a parent
    /// process.
    fn native_client(&self, ruby: &Ruby) -> Result<wreq::Client, magnus::Error> {
        self.0.get(ruby).cloned()
    }
}

impl Client {
    /// Send a request through a newly built default client.
    ///
    /// Request arguments are validated before the native client is built, so
    /// invalid options fail without initializing a connection pool.
    pub(crate) fn request_with_default_client(
        ruby: &Ruby,
        args: &[Value],
    ) -> Result<Response, magnus::Error> {
        let ((method, url), request) = extract_request!(ruby, args, (Obj<Method>, String));
        let client = Self::default_client(ruby)?;
        execute_request(ruby, client, *method, url, request)
    }

    /// Send a request with `method` through a newly built default client.
    ///
    /// Request arguments are validated before the native client is built, so
    /// invalid options fail without initializing a connection pool.
    pub(crate) fn execute_with_default_client(
        ruby: &Ruby,
        method: Method,
        args: &[Value],
    ) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        let client = Self::default_client(ruby)?;
        execute_request(ruby, client, method, url, request)
    }

    /// Send a HTTP request.
    #[inline]
    pub fn request(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((method, url), request) = extract_request!(ruby, args, (Obj<Method>, String));
        execute_request(ruby, rb_self.native_client(ruby)?, *method, url, request)
    }

    /// Send a GET request.
    #[inline]
    pub fn get(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::GET,
            url,
            request,
        )
    }

    /// Send a POST request.
    #[inline]
    pub fn post(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::POST,
            url,
            request,
        )
    }

    /// Send a PUT request.
    #[inline]
    pub fn put(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::PUT,
            url,
            request,
        )
    }

    /// Send a DELETE request.
    #[inline]
    pub fn delete(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::DELETE,
            url,
            request,
        )
    }

    /// Send a HEAD request.
    #[inline]
    pub fn head(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::HEAD,
            url,
            request,
        )
    }

    /// Send an OPTIONS request.
    #[inline]
    pub fn options(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::OPTIONS,
            url,
            request,
        )
    }

    /// Send a TRACE request.
    #[inline]
    pub fn trace(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::TRACE,
            url,
            request,
        )
    }

    /// Send a PATCH request.
    #[inline]
    pub fn patch(ruby: &Ruby, rb_self: &Self, args: &[Value]) -> Result<Response, magnus::Error> {
        let ((url,), request) = extract_request!(ruby, args, (String,));
        execute_request(
            ruby,
            rb_self.native_client(ruby)?,
            Method::PATCH,
            url,
            request,
        )
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), magnus::Error> {
    let client_class = gem_module.define_class("Client", ruby.class_object())?;
    client_class.define_singleton_method("new", function!(Client::new, -1))?;
    client_class.define_method("request", method!(Client::request, -1))?;
    client_class.define_method("get", method!(Client::get, -1))?;
    client_class.define_method("post", method!(Client::post, -1))?;
    client_class.define_method("put", method!(Client::put, -1))?;
    client_class.define_method("delete", method!(Client::delete, -1))?;
    client_class.define_method("head", method!(Client::head, -1))?;
    client_class.define_method("options", method!(Client::options, -1))?;
    client_class.define_method("trace", method!(Client::trace, -1))?;
    client_class.define_method("patch", method!(Client::patch, -1))?;

    resp::include(ruby, gem_module)?;
    body::include(ruby, gem_module)?;
    Ok(())
}
