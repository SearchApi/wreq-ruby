# frozen_string_literal: true

unless defined?(:Wreq)
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
      # @param user_agent [String, nil] Custom User-Agent header value.
      #   If not specified, a default user agent will be used.
      #
      # @param headers [Hash{String=>String}, nil] Default headers to include
      #   in every request. Header names are case-insensitive. These headers
      #   can be overridden on a per-request basis.
      #
      # @param referer [Boolean, nil] Whether to automatically send Referer
      #   headers when following redirects. When true, the previous URL will
      #   be sent as the Referer header.
      #
      # @param history [Boolean, nil] Whether to track the full redirect chain
      #   for each request. Useful for debugging redirect issues.
      #
      # @param allow_redirects [Boolean, nil] Enable automatic following of
      #   HTTP redirects (3xx status codes). When false, redirect responses
      #   will be returned directly to the caller.
      #
      # @param max_redirects [Integer, nil] Maximum number of redirects to
      #   follow before returning an error. Only applies when allow_redirects
      #   is true. Default is typically 10 if not specified.
      #
      # @param cookie_store [Boolean, nil] Enable an in-memory cookie jar
      #   that automatically handles Set-Cookie headers and sends appropriate
      #   Cookie headers on subsequent requests.
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
      #   forcibly closed. Linux-specific option (Android, Fuchsia, Linux only).
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
      # @param no_proxy [Boolean, nil] Disable use of any configured proxy
      #   for this client, even if proxy settings are detected from the
      #   environment.
      #
      # @param proxy [String, nil] Proxy server URI to use for all requests.
      #   Supports HTTP, HTTPS, and SOCKS5 proxies. Format: "protocol://host:port"
      #   Example: "http://proxy.example.com:8080"
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
      # @raise [ArgumentError] if incompatible options are specified together
      #   (e.g., http1_only and http2_only both true).
      # @raise [RuntimeError] if the underlying client cannot be initialized
      #   due to system resource constraints or invalid configuration.
      #
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
      #     referer: true,
      #     history: true
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
      def self.new(**options); end

      # @param method [Wreq::Method] HTTP method to use
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def request(method, url, **options); end

      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def get(url, **options); end

      # Send an HTTP HEAD request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def head(url, **options); end

      # Send an HTTP POST request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def post(url, **options); end

      # Send an HTTP PUT request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def put(url, **options); end

      # Send an HTTP DELETE request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def delete(url, **options); end

      # Send an HTTP OPTIONS request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def options(url, **options); end

      # Send an HTTP TRACE request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def trace(url, **options); end

      # Send an HTTP PATCH request.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data (application/x-www-form-urlencoded)
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token for Authorization header
      # @option options [Array<String>] :basic_auth Username and password for basic auth
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def patch(url, **options); end
    end
  end
end
