# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # HTTP client with extensive configuration options.
    #
    # This class wraps a native Rust implementation providing high-performance
    # HTTP/1.1 and HTTP/2 client functionality with support for connection pooling,
    # compression, redirects, proxies, and fine-grained timeout controls.
    #
    # The client is thread-safe and maintains an internal connection pool for
    # efficient request reuse.
    #
    # Client and request options are limited to the keys listed below. Unknown,
    # ambiguous, ineffective, and unavailable platform options raise
    # ArgumentError. Known values retain the error class from their Ruby or
    # native conversion, such as TypeError or Wreq::BuilderError. Request
    # validation finishes before network I/O.
    #
    # A client belongs to the process that created it. An inherited client
    # raises Wreq::ForkError before its connection pool is accessed. Loading
    # the gem before fork is supported, but clients must be created inside the
    # worker. If the parent already started the runtime, new clients can be
    # constructed in the child but cannot send requests.
    #
    # @note Fork safety Create each client in the worker that uses it. An
    #   inherited client is never rebuilt automatically.
    #
    # @example Basic usage
    #   client = Wreq::Client.new
    #   # Use client for HTTP requests
    #
    # @example With common options
    #   client = Wreq::Client.new(
    #     user_agent: "MyApp/1.0",
    #     timeout: 30,
    #     gzip: true,
    #     brotli: true
    #   )
    #
    # @see https://github.com/your-repo/wreq-ruby Full documentation
    class Client
      # Create a new HTTP client instance.
      #
      # All options are optional. Time-related numeric values are expressed in seconds.
      #
      # @param emulation [Wreq::Emulation, nil] Device and OS emulation settings.
      #   If specified, the client will modify request headers and behaviors
      #
      # @param user_agent [String, nil] Custom User-Agent header value.
      #   If not specified, a default user agent will be used.
      #
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Default headers to include
      #   in every request. Header names are case-insensitive. These headers
      #   can be overridden on a per-request basis.
      # @param orig_headers [Array<String>, nil] Original header names used to
      #   preserve raw header order and HTTP/1 case-sensitive header handling.
      #
      # @param referer [Boolean, nil] Whether to automatically send Referer
      #   headers when following redirects. When true, the previous URL will
      #   be sent as the Referer header.
      #
      # @param allow_redirects [Boolean, nil] Enable automatic following of
      #   HTTP redirects (3xx status codes). When false, redirect responses
      #   will be returned directly to the caller.
      #
      # @param max_redirects [Integer, nil] Maximum number of redirects to
      #   follow before returning an error. Requires `allow_redirects: true`.
      #
      # @param cookie_store [Boolean, nil] Enable an in-memory cookie jar
      #   that automatically handles Set-Cookie headers and sends appropriate
      #   Cookie headers on subsequent requests.
      #
      # @param cookie_provider [Wreq::Jar, nil] Custom cookie jar provider
      #   used to store and retrieve cookies for all requests made by this
      #   client. Typically used together with `cookie_store: true`.
      #
      # @param timeout [Integer, nil] Overall timeout for the entire request
      #   in seconds, including connection establishment, request transmission,
      #   and response reading. If not set, requests may wait indefinitely.
      #
      # @param connect_timeout [Integer, nil] Maximum time in seconds to wait
      #   when establishing a connection to the remote server. This is separate
      #   from the overall timeout.
      #
      # @param read_timeout [Integer, nil] Maximum time in seconds to wait
      #   between reading chunks of data from the server. Applies to each
      #   read operation, not the entire response.
      #
      # @param tcp_keepalive [Integer, nil] Time in seconds that a connection
      #   must be idle before TCP keepalive probes are sent. Helps detect
      #   broken connections.
      #
      # @param tcp_keepalive_interval [Integer, nil] Time in seconds between
      #   individual TCP keepalive probes. Only relevant if tcp_keepalive is set.
      #
      # @param tcp_keepalive_retries [Integer, nil] Number of failed keepalive
      #   probes before the connection is considered dead and closed.
      #
      # @param tcp_user_timeout [Integer, nil] Maximum time in seconds that
      #   transmitted data may remain unacknowledged before the connection is
      #   forcibly closed. Available on Android, Fuchsia, and Linux only.
      #
      # @param tcp_nodelay [Boolean, nil] Enable TCP_NODELAY socket option,
      #   which disables Nagle's algorithm. When true, small packets are sent
      #   immediately rather than being buffered. Useful for reducing latency
      #   in interactive protocols.
      #
      # @param tcp_reuse_address [Boolean, nil] Enable SO_REUSEADDR socket option,
      #   allowing the reuse of local addresses in TIME_WAIT state. Useful for
      #   reducing port exhaustion in high-throughput scenarios.
      #
      # @param pool_idle_timeout [Integer, nil] Time in seconds before idle
      #   connections in the pool are evicted and closed. Helps free up
      #   resources for long-running applications.
      #
      # @param pool_max_idle_per_host [Integer, nil] Maximum number of idle
      #   connections to maintain per host in the connection pool. Connections
      #   beyond this limit will be closed immediately after use.
      #
      # @param pool_max_size [Integer, nil] Total maximum size of the connection
      #   pool across all hosts. Once reached, new requests may need to wait
      #   for existing connections to become available.
      #
      # @param http1_only [Boolean, nil] Force the client to use HTTP/1.1 only,
      #   even if HTTP/2 is available. Useful for compatibility with servers
      #   that have problematic HTTP/2 implementations.
      #
      # @param http2_only [Boolean, nil] Force the client to use HTTP/2 only.
      #   Requests to servers that don't support HTTP/2 will fail. Cannot be
      #   combined with http1_only.
      #
      # @param https_only [Boolean, nil] Reject plain HTTP connections and
      #   only allow HTTPS. Provides an additional layer of security by
      #   preventing accidental cleartext connections.
      #
      # @param verify [Boolean, nil] Enable or disable TLS certificate
      #   verification. When false, the client will accept any certificate,
      #   including self-signed or expired ones. Should only be disabled
      #   for testing purposes.
      #
      # @param tls_info [Boolean, nil] Retain peer certificate data for HTTPS
      #   responses. When true, {Wreq::Response#tls_info} may return a
      #   {Wreq::TlsInfo} object. Disabled by default because retaining
      #   certificate data uses additional memory.
      #
      # @param ca_file [String, #to_path, nil] Path to a PEM-encoded CA bundle
      #   that **replaces** the default system trust store. Only certificates
      #   signed by CAs in this file will be trusted. Accepts any object
      #   responding to +to_path+ (e.g. +Pathname+). The file is read during
      #   client construction; a missing or unreadable file raises immediately.
      #   Mutually exclusive with +ca_pem+, +additional_ca_file+, and
      #   +additional_ca_pem+.
      #
      # @param ca_pem [String, nil] Raw PEM-encoded certificate content that
      #   **replaces** the default system trust store. Useful when certificate
      #   material comes from a secret store or environment variable rather
      #   than a file on disk. Must contain at least one
      #   +-----BEGIN CERTIFICATE-----+ block.
      #   Mutually exclusive with +ca_file+, +additional_ca_file+, and
      #   +additional_ca_pem+.
      #
      # @param additional_ca_file [String, #to_path, nil] Path to a PEM-encoded
      #   CA bundle loaded **alongside** the default system trust store.
      #   Public roots remain available; the supplied certificates are added
      #   on top. Accepts any object responding to +to_path+ (e.g. +Pathname+).
      #   Mutually exclusive with +ca_file+, +ca_pem+, and +additional_ca_pem+.
      #
      # @param additional_ca_pem [String, nil] Raw PEM-encoded certificate
      #   content loaded **alongside** the default system trust store. Public
      #   roots remain available; the supplied certificates are added on top.
      #   Must contain at least one +-----BEGIN CERTIFICATE-----+ block.
      #   Mutually exclusive with +ca_file+, +ca_pem+, and
      #   +additional_ca_file+.
      #
      # @param no_proxy [Boolean, nil] Disable use of any configured proxy
      #   for this client, even if proxy settings are detected from the
      #   environment.
      #
      # @param proxy [String, nil] Proxy server URI to use for all requests.
      #   Supports HTTP, HTTPS, and SOCKS5 proxies. Format: "protocol://host:port"
      #   Example: "http://proxy.example.com:8080"
      # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable.
      # @param interface [String, nil] Bind sockets to a network interface on
      #   platforms supported by the native client. Unsupported platforms raise
      #   ArgumentError.
      #
      # @param gzip [Boolean, nil] Accept and automatically decompress gzip
      #   content encoding. When true, adds "Accept-Encoding: gzip" header.
      #
      # @param brotli [Boolean, nil] Accept and automatically decompress Brotli
      #   content encoding. When true, adds "Accept-Encoding: br" header.
      #   Provides better compression than gzip.
      #
      # @param deflate [Boolean, nil] Accept and automatically decompress deflate
      #   content encoding. When true, adds "Accept-Encoding: deflate" header.
      #
      # @param zstd [Boolean, nil] Accept and automatically decompress Zstandard
      #   content encoding. When true, adds "Accept-Encoding: zstd" header.
      #   Modern compression algorithm with excellent performance.
      #
      # @return [Wreq::Client] A configured HTTP client instance ready to make requests.
      #
      # @raise [ArgumentError] if an option is unknown, conflicting,
      #   ineffective, or unavailable on the current platform.
      # @raise [TypeError] if the option argument is not a Hash.
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted or validated.
      # @raise [Wreq::BuilderError, Wreq::TlsError] if the native client cannot
      #   be initialized.
      # @raise [Wreq::ForkError] if :cookie_provider belongs to a parent process.
      # @example Minimal client
      #   client = Wreq::Client.new
      #
      # @example Client with custom headers
      #   client = Wreq::Client.new(
      #     user_agent: "MyApp/2.0 (https://example.com)",
      #     headers: {
      #       "Accept" => "application/json",
      #       "X-API-Key" => "secret-key-here"
      #     }
      #   )
      #
      # @example Client with timeouts
      #   client = Wreq::Client.new(
      #     timeout: 30,           # 30 seconds total
      #     connect_timeout: 5,    # 5 seconds to connect
      #     read_timeout: 10       # 10 seconds between reads
      #   )
      #
      # @example Client with redirect handling
      #   client = Wreq::Client.new(
      #     allow_redirects: true,
      #     max_redirects: 5,
      #     referer: true
      #   )
      #
      # @example Client with compression
      #   client = Wreq::Client.new(
      #     gzip: true,
      #     brotli: true,
      #     zstd: true
      #   )
      #
      # @example Client with proxy
      #   client = Wreq::Client.new(
      #     proxy: "http://proxy.corp.com:8080"
      #   )
      #
      # @example Client with SOCKS5 proxy
      #   client = Wreq::Client.new(
      #     proxy: "socks5://localhost:1080"
      #   )
      #
      # @example HTTPS-only client with strict verification
      #   client = Wreq::Client.new(
      #     https_only: true,
      #     verify: true
      #   )
      #
      # @example HTTP/2 optimized client
      #   client = Wreq::Client.new(
      #     http2_only: true,
      #     tcp_nodelay: true
      #   )
      #
      # @example Connection pool tuning
      #   client = Wreq::Client.new(
      #     pool_max_idle_per_host: 32,
      #     pool_idle_timeout: 90,
      #     pool_max_size: 128
      #   )
      #
      # @example TCP keepalive configuration
      #   client = Wreq::Client.new(
      #     tcp_keepalive: 60,
      #     tcp_keepalive_interval: 10,
      #     tcp_keepalive_retries: 3
      #   )
      #
      # @example Development/testing client (insecure)
      #   client = Wreq::Client.new(
      #     verify: false,  # WARNING: Do not use in production!
      #     timeout: 5
      #   )
      # @example Client with custom CA (replace system roots)
      #   client = Wreq::Client.new(
      #     ca_file: "/etc/ssl/private/internal-ca.pem"
      #   )
      #
      # @example Client with additional CA (augment system roots)
      #   client = Wreq::Client.new(
      #     additional_ca_pem: File.binread("/etc/ssl/certs/extra-ca.pem")
      #   )
      def self.new(**options)
      end

      # Send an HTTP request.
      #
      # Only the options listed here are accepted. Body options (`body`, `form`,
      # `json`) and authentication options (`auth`, `bearer_auth`, `basic_auth`)
      # are mutually exclusive. `max_redirects` requires `allow_redirects: true`.
      #
      # @param method [Wreq::Method] HTTP method to use
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [ArgumentError] if options are unknown, conflicting, ineffective,
      #   or unavailable on the current platform
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def request(method, url, **options)
      end

      # Send an HTTP GET request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def get(url, **options)
      end

      # Send an HTTP HEAD request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def head(url, **options)
      end

      # Send an HTTP POST request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def post(url, **options)
      end

      # Send an HTTP PUT request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def put(url, **options)
      end

      # Send an HTTP DELETE request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def delete(url, **options)
      end

      # Send an HTTP OPTIONS request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def options(url, **options)
      end

      # Send an HTTP TRACE request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def trace(url, **options)
      end

      # Send an HTTP PATCH request.
      #
      # @param url [String] Target URL
      # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
      # @param default_headers [Boolean, nil] Whether to apply native default headers
      # @param query [Hash, nil] URL query parameters
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Hash{String=>String}, String, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum redirects; requires allow_redirects: true
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param local_address [String, nil] Bind the request's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
      # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
      # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
      # @return [Wreq::Response] HTTP response
      # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
      #   value cannot be converted, validated, or built
      # @raise [Wreq::ForkError] if the client or runtime belongs to the parent process
      def patch(url, **options)
      end
    end
  end
end

module Wreq
  class Client
    def inspect
      "#<Wreq::Client>"
    end
  end
end
