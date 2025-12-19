# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # System-level and runtime errors

    # Memory allocation failed.
    class MemoryError < StandardError; end

    # Network connection errors

    # Connection to the server failed.
    #
    # Raised when the client cannot establish a connection to the server.
    #
    # @example
    #   begin
    #     client.get("http://localhost:9999")
    #   rescue Wreq::ConnectionError => e
    #     puts "Connection failed: #{e.message}"
    #     retry_with_backoff
    #   end
    class ConnectionError < StandardError; end

    # Proxy Connection to the server failed.
    #
    # Raised when the client cannot establish a connection to the proxy server.
    # @example
    #   begin
    #     client.get("http://example.com", proxy: "http://invalid-proxy:8080")
    #   rescue Wreq::ProxyConnectionError => e
    #     puts "Proxy connection failed: #{e.message}"
    #     retry_with_different_proxy
    #   end
    class ProxyConnectionError < StandardError; end

    # Connection was reset by the server.
    #
    # Raised when the server closes the connection unexpectedly.
    #
    # @example
    #   rescue Wreq::ConnectionResetError => e
    #     puts "Connection reset: #{e.message}"
    #   end
    class ConnectionResetError < StandardError; end

    # TLS/SSL error occurred.
    #
    # Raised when there's an error with TLS/SSL, such as certificate
    # verification failure or protocol mismatch.
    #
    # @example
    #   begin
    #     client.get("https://self-signed.badssl.com")
    #   rescue Wreq::TlsError => e
    #     puts "TLS error: #{e.message}"
    #   end
    unless const_defined?(:TlsError)
      class TlsError < StandardError; end
    end

    # HTTP protocol and request/response errors

    # Request failed.
    #
    # Generic error for request failures that don't fit other categories.
    #
    # @example
    #   rescue Wreq::RequestError => e
    #     puts "Request failed: #{e.message}"
    #   end
    class RequestError < StandardError; end

    # HTTP status code indicates an error.
    #
    # Raised when the server returns an error status code (4xx or 5xx).
    #
    # @example
    #   begin
    #     response = client.get("https://httpbin.io/status/404")
    #   rescue Wreq::StatusError => e
    #     puts "HTTP error: #{e.message}"
    #     # e.response contains the full response
    #   end
    class StatusError < StandardError; end

    # Redirect handling failed.
    #
    # Raised when too many redirects occur or redirect logic fails.
    #
    # @example
    #   begin
    #     client = Wreq::Client.new(max_redirects: 3)
    #     client.get("https://httpbin.io/redirect/10")
    #   rescue Wreq::RedirectError => e
    #     puts "Too many redirects: #{e.message}"
    #   end
    class RedirectError < StandardError; end

    # Request timed out.
    #
    # Raised when the request exceeds the configured timeout.
    #
    # @example
    #   begin
    #     client = Wreq::Client.new(timeout: 5)
    #     client.get("https://httpbin.io/delay/10")
    #   rescue Wreq::TimeoutError => e
    #     puts "Request timed out: #{e.message}"
    #     retry_with_longer_timeout
    #   end
    class TimeoutError < StandardError; end

    # Data processing and encoding errors

    # Response body processing failed.
    #
    # Raised when there's an error reading or processing the response body.
    #
    # @example
    #   rescue Wreq::BodyError => e
    #     puts "Body error: #{e.message}"
    #   end
    class BodyError < StandardError; end

    # Decoding response failed.
    #
    # Raised when response content cannot be decoded (e.g., invalid UTF-8,
    # malformed JSON, corrupted compression).
    #
    # @example
    #   begin
    #     response = client.get("https://example.com/invalid-utf8")
    #     response.text  # May raise DecodingError
    #   rescue Wreq::DecodingError => e
    #     puts "Decoding error: #{e.message}"
    #     # Fall back to binary data
    #     data = response.body
    #   end
    class DecodingError < StandardError; end

    # Configuration and builder errors

    # Client configuration is invalid.
    #
    # Raised when the client is configured with invalid options.
    #
    # @example
    #   begin
    #     client = Wreq::Client.new(
    #       proxy: "invalid://proxy",
    #       timeout: -1
    #     )
    #   rescue Wreq::BuilderError => e
    #     puts "Invalid configuration: #{e.message}"
    #   end
    class BuilderError < StandardError; end
  end
end
