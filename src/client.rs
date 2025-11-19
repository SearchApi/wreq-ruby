use std::{net::IpAddr, time::Duration};

use magnus::{
    Module, Object, RArray, RHash, RString, Ruby, Symbol, function, kwargs, method,
    r_hash::ForEach, value::ReprValue,
};
use serde::Deserialize;
use wreq::{
    Proxy, Uri,
    header::{HeaderMap, HeaderName, HeaderValue, OrigHeaderMap},
};

use crate::{
    error::{header_name_error_to_magnus, header_value_error_to_magnus, wreq_error_to_magnus},
    nogvl,
};

/// A builder for `Client`.
#[derive(Debug, Default, Deserialize)]
struct Builder {
    /// The user agent to use for the client.
    #[serde(skip)]
    user_agent: Option<HeaderValue>,
    /// The headers to use for the client.
    #[serde(skip)]
    headers: Option<HeaderMap>,
    /// The original headers to use for the client.
    #[serde(skip)]
    orig_headers: Option<OrigHeaderMap>,
    /// Whether to use referer.
    referer: Option<bool>,
    /// Whether to keep track of request history.
    history: Option<bool>,
    /// Whether to allow redirects.
    allow_redirects: Option<bool>,
    /// The maximum number of redirects to follow.
    max_redirects: Option<usize>,

    // ========= Cookie options =========
    /// Whether to use cookie store.
    cookie_store: Option<bool>,

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
    pool_max_size: Option<u32>,

    // ========= Protocol options =========
    /// Whether to use the HTTP/1 protocol only.
    http1_only: Option<bool>,
    /// Whether to use the HTTP/2 protocol only.
    http2_only: Option<bool>,
    /// Whether to use HTTPS only.
    https_only: Option<bool>,

    // ========= TLS options =========
    /// Whether to verify the SSL certificate or root certificate file path.
    verify: Option<bool>,

    // ========= Network options =========
    /// Whether to disable the proxy for the client.
    no_proxy: Option<bool>,
    /// The proxy to use for the client.
    #[serde(skip)]
    proxy: Option<Proxy>,

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

#[derive(Clone)]
#[magnus::wrap(class = "Wreq::Client", free_immediately, size)]
pub struct Client(wreq::Client);

// ===== impl Builder =====

impl Builder {
    /// Create a new [`Builder`] from Ruby keyword arguments.
    fn new(ruby: &magnus::Ruby, kwargs: RHash) -> Result<Self, magnus::Error> {
        let mut builder: Builder = serde_magnus::deserialize(&ruby, kwargs)?;

        // Handle user agent separately
        if let Some(user_agent) = kwargs
            .get(ruby.to_symbol("user_agent"))
            .and_then(RString::from_value)
        {
            let value = HeaderValue::from_maybe_shared(user_agent.to_bytes())
                .map_err(|err| header_value_error_to_magnus(ruby, err))?;
            builder.user_agent = Some(value);
        }

        // Handle headers separately
        if let Some(headers) = kwargs
            .get(ruby.to_symbol("headers"))
            .and_then(RHash::from_value)
        {
            let mut map = HeaderMap::new();
            headers.foreach(|name: RString, value: RString| {
                let name = HeaderName::from_bytes(&name.to_bytes())
                    .map_err(|err| header_name_error_to_magnus(ruby, err))?;
                let value = HeaderValue::from_maybe_shared(value.to_bytes())
                    .map_err(|err| header_value_error_to_magnus(ruby, err))?;
                map.insert(name, value);

                Ok(ForEach::Continue)
            });
            builder.headers = Some(map);
        }

        // Handle original headers separately
        if let Some(orig_headers) = kwargs
            .get(ruby.to_symbol("orig_headers"))
            .and_then(RArray::from_value)
        {
            let mut map = OrigHeaderMap::new();
            for value in orig_headers.into_iter().flat_map(RString::from_value) {
                map.insert(value.to_bytes());
            }
            builder.orig_headers = Some(map);
        }

        // Handle proxies separately
        if let Some(proxy) = kwargs
            .get(ruby.to_symbol("proxy"))
            .and_then(RString::from_value)
        {
            let uri = Proxy::all(proxy.to_bytes().as_ref())
                .map_err(|err| wreq_error_to_magnus(ruby, err))?;
            builder.proxy = Some(uri);
        }

        Ok(builder)
    }
}

impl Client {
    /// Create a new [`Client`] with the given keyword arguments.
    pub fn new(ruby: &Ruby, args: &[magnus::Value]) -> Result<Self, magnus::Error> {
        if let Some(kwargs) = args.first().cloned().and_then(RHash::from_value) {
            let mut params = Builder::new(ruby, kwargs)?;
            nogvl::nogvl(|| {
                let mut builder = wreq::Client::builder();

                // User agent options.
                apply_option!(set_if_some, builder, params.user_agent, user_agent);

                // Default headers options.
                apply_option!(set_if_some, builder, params.headers, default_headers);
                apply_option!(set_if_some, builder, params.orig_headers, orig_headers);

                // Allow redirects options.
                apply_option!(set_if_some, builder, params.referer, referer);
                apply_option!(set_if_some, builder, params.history, history);
                apply_option!(
                    set_if_true_with,
                    builder,
                    params.allow_redirects,
                    redirect,
                    false,
                    params
                        .max_redirects
                        .take()
                        .map(wreq::redirect::Policy::limited)
                        .unwrap_or_default()
                );

                // Cookie options.
                apply_option!(set_if_some, builder, params.cookie_store, cookie_store);

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
                apply_option!(set_if_some, builder, params.verify, cert_verification);

                // Network options.
                apply_option!(set_if_some, builder, params.proxy, proxy);
                apply_option!(set_if_true, builder, params.no_proxy, no_proxy, false);

                // Compression options.
                apply_option!(set_if_some, builder, params.gzip, gzip);
                apply_option!(set_if_some, builder, params.brotli, brotli);
                apply_option!(set_if_some, builder, params.deflate, deflate);
                apply_option!(set_if_some, builder, params.zstd, zstd);

                builder
                    .build()
                    .map(Client)
                    .map_err(|err| wreq_error_to_magnus(ruby, err))
            })
        } else {
            nogvl::nogvl(|| Ok(Self(wreq::Client::new())))
        }
    }
}

pub fn include(ruby: &magnus::Ruby, gem_module: &magnus::RModule) -> Result<(), magnus::Error> {
    let client_class = gem_module.define_class("Client", ruby.class_object())?;
    client_class.define_singleton_method("new", function!(Client::new, -1))?;
    Ok(())
}
