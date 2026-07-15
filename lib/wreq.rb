# frozen_string_literal: true

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "wreq_ruby/#{$1}/wreq_ruby"
rescue LoadError
  require_relative "wreq_ruby/wreq_ruby"
end

# Load type hint definitions
require_relative "wreq_ruby/http"
require_relative "wreq_ruby/client"
require_relative "wreq_ruby/response"
require_relative "wreq_ruby/body"
require_relative "wreq_ruby/header"
require_relative "wreq_ruby/error"
require_relative "wreq_ruby/cookie"

unless defined?(Wreq)
  module Wreq
    # Current wreq gem version.
    # @return [String]
    VERSION = nil

    # Module request methods accept only the options documented for each
    # method. Unknown, ambiguous, ineffective, and unavailable platform options
    # raise ArgumentError. Known values retain the error class from their Ruby
    # or native conversion, such as TypeError or Wreq::BuilderError. Validation
    # finishes before network I/O.
    #
    # Requests made in a child process forked after wreq-ruby was loaded raise
    # Wreq::ForkError. Load wreq-ruby inside each worker after it has been
    # forked.

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
    def self.patch(url, **options)
    end
  end
end
