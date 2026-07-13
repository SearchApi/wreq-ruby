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

    # Send an HTTP request.
    #
    # @param method [Wreq::Method] HTTP method to use
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.request(method, url, **options)
    end

    # Send an HTTP GET request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.get(url, **options)
    end

    # Send an HTTP HEAD request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.head(url, **options)
    end

    # Send an HTTP POST request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.post(url, **options)
    end

    # Send an HTTP PUT request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.put(url, **options)
    end

    # Send an HTTP DELETE request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.delete(url, **options)
    end

    # Send an HTTP OPTIONS request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.options(url, **options)
    end

    # Send an HTTP TRACE request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.trace(url, **options)
    end

    # Send an HTTP PATCH request.
    #
    # @param url [String] Target URL
    # @param headers [Wreq::Headers, Hash{String=>String}, nil] Custom headers for this request
    # @param orig_headers [Array<String>, nil] Original header names used to preserve raw header order and HTTP/1 case-sensitive header handling
    # @param default_headers [Boolean, nil] Whether to apply default emulation headers
    # @param query [Hash, nil] URL query parameters
    # @param auth [String, nil] Authorization header value
    # @param bearer_auth [String, nil] Bearer token for Authorization header
    # @param basic_auth [Array<String>, nil] Username and password for basic auth
    # @param cookies [Hash{String=>String}, String, nil] Cookies to send
    # @param allow_redirects [Boolean, nil] Whether to follow redirects
    # @param max_redirects [Integer, nil] Maximum number of redirects to follow
    # @param gzip [Boolean, nil] Enable gzip compression
    # @param brotli [Boolean, nil] Enable Brotli compression
    # @param deflate [Boolean, nil] Enable deflate compression
    # @param zstd [Boolean, nil] Enable Zstandard compression
    # @param timeout [Integer, nil] Total request timeout (seconds)
    # @param read_timeout [Integer, nil] Per-chunk read timeout (seconds)
    # @param proxy [String, nil] Proxy server URI
    # @param local_address [String, nil] Bind the client's local source IP address (IPv4/IPv6). Useful on multi-homed hosts to originate connections from a specific address or enforce source routing. Examples: "192.168.1.10", "10.0.0.5", "2001:db8::1". The address must exist on the host and be routable or the connection may fail.
    # @param interface [String, nil] Bind the socket to a specific network interface via `SO_BINDTODEVICE` (e.g., "eth0", "wlan0", "tun0"). Effective only on systems that support the option (Linux/Android/Fuchsia) and typically requires privileges (root or CAP_NET_ADMIN).
    # @param emulation [Wreq::Emulation, nil] Device/OS emulation for this request
    # @param version [Wreq::Version, nil] HTTP version to use
    # @param form [Hash{String=>String}, nil] Form data (application/x-www-form-urlencoded)
    # @param json [Object, nil] JSON body serialized by the native encoder; Integer values retain arbitrary precision
    # @param body [String, Wreq::BodySender, nil] Request body bytes or streaming body sender
    # @return [Wreq::Response] HTTP response
    # @raise [Wreq::BuilderError] if json contains unsupported values; raised before network I/O
    def self.patch(url, **options)
    end
  end
end
