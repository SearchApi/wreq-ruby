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

unless defined?(:Wreq)
  module Wreq
    class << self
      # Send an HTTP request.
      #
      # @param method [Wreq::Method] HTTP method to use
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def request(method, url, **options); end

      # Send an HTTP GET request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def get(url, **options); end

      # Send an HTTP HEAD request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def head(url, **options); end

      # Send an HTTP POST request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def post(url, **options); end

      # Send an HTTP PUT request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def put(url, **options); end

      # Send an HTTP DELETE request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def delete(url, **options); end

      # Send an HTTP OPTIONS request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def options(url, **options); end

      # Send an HTTP TRACE request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def trace(url, **options); end

      # Send an HTTP PATCH request.
      #
      # @param url [String] Target URL
      # @param headers [Hash{String=>String}, nil] Custom headers for this request
      # @param orig_headers [Hash{String=>String}, nil] Original headers (raw, unmodified)
      # @param default_headers [Hash{String=>String}, nil] Default headers to merge
      # @param query [Hash, nil] URL query parameters
      # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
      # @param json [Object, nil] JSON body (will be serialized)
      # @param body [String, IO, nil] Raw request body (string or stream)
      # @param auth [String, nil] Authorization header value
      # @param bearer_auth [String, nil] Bearer token for Authorization header
      # @param basic_auth [Array<String>, nil] Username and password for basic auth
      # @param cookies [Array<String>, nil] Cookies to send
      # @param allow_redirects [Boolean, nil] Whether to follow redirects
      # @param max_redirects [Integer, nil] Maximum number of redirects to follow
      # @param referer [Boolean, nil] Whether to send Referer header on redirects
      # @param history [Boolean, nil] Track full redirect chain
      # @param gzip [Boolean, nil] Enable gzip compression
      # @param brotli [Boolean, nil] Enable Brotli compression
      # @param deflate [Boolean, nil] Enable deflate compression
      # @param zstd [Boolean, nil] Enable Zstandard compression
      # @param timeout [Integer, nil] Total request timeout (seconds)
      # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
      # @param proxy [String, nil] Proxy server URI
      # @param no_proxy [Boolean, nil] Disable proxy for this request
      # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
      # @param version [Wreq::Version, nil] HTTP version to use
      # @return [Wreq::Response] HTTP response
      def patch(url, **options); end
    end
  end
end
