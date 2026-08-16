# frozen_string_literal: true

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "wreq_ruby/#{$1}/wreq_ruby"
rescue LoadError
  require_relative "wreq_ruby/wreq_ruby"
end

# Load type hint definitions
require_relative "wreq_ruby/http"
require_relative "wreq_ruby/emulate"
require_relative "wreq_ruby/client"
require_relative "wreq_ruby/response"
require_relative "wreq_ruby/tls"
require_relative "wreq_ruby/body"
require_relative "wreq_ruby/header"
require_relative "wreq_ruby/error"
require_relative "wreq_ruby/cookie"

unless defined?(Wreq)
  # An HTTP client backed by a lazily initialized, process-wide Tokio runtime.
  #
  # Loading wreq-ruby before `fork` is supported. The parent must not send a
  # request or perform another operation that starts the runtime before workers
  # are forked. Create clients and begin HTTP work inside each worker so it gets
  # its own runtime and connection pool. Clients, responses, body senders, and
  # cookie jars belong to the process that created them and must be recreated
  # in the worker. wreq-ruby does not rebuild inherited objects.
  #
  # Accessing an inherited native-backed object raises Wreq::ForkError even if
  # the parent did not start the runtime. If the parent did start it, the child
  # also cannot perform new runtime-backed operations. Retrying does not replace
  # either kind of inherited state. Use `spawn` or `exec`, or move the parent's
  # HTTP work until after the workers have been forked.
  #
  # @example Preload the extension, then start HTTP work in the worker
  #   require "wreq"
  #
  #   Process.fork do
  #     client = Wreq::Client.new
  #     response = client.get("https://example.com")
  #     puts response.status
  #   end
  #
  # @note Fork safety Create clients, cookie jars, and body senders inside the
  #   worker that uses them. Do not carry responses across `fork`.
  # @see https://github.com/SearchApi/wreq-ruby/blob/main/docs/fork-safety.md
  module Wreq
    # Current wreq gem version.
    # @return [String]
    VERSION = nil

    # Module request methods accept only the options documented for each
    # method. Unknown, ambiguous, ineffective, and unavailable platform options
    # raise ArgumentError. Known values retain the error class from their Ruby
    # or native conversion, such as TypeError or Wreq::BuilderError. Validation
    # finishes before network I/O.

    # Send an HTTP request.
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.request(method, url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.get(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.head(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.post(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.put(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.delete(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.options(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.trace(url, **options)
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
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind to an interface on supported platforms; unsupported platforms raise ArgumentError.
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Raw or streaming request body
    # @return [Wreq::Response] HTTP response
    # @raise [TypeError, ArgumentError, Wreq::BuilderError] if a known option
    #   value cannot be converted, validated, or built
    # @raise [Wreq::ForkError] if the child inherited an initialized wreq-ruby runtime
    def self.patch(url, **options)
    end
  end
end
