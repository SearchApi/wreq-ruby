# frozen_string_literal: true

module Wreq
  # Base error class for all Wreq exceptions.
  # This is only defined if the native extension hasn't loaded yet.
  unless const_defined?(:Error)
    class Error < StandardError; end
  end

  # System-level and runtime errors

  # DNS resolution failed.
  #
  # Raised when the DNS resolver cannot resolve the hostname.
  #
  # @example
  #   begin
  #     client.get("http://nonexistent.domain.invalid")
  #   rescue Wreq::DNSResolverError => e
  #     puts "DNS resolution failed: #{e.message}"
  #   end
  unless const_defined?(:DNSResolverError)
    class DNSResolverError < Error; end
  end

  # Rust panic occurred.
  #
  # Raised when an unexpected panic occurs in the Rust code.
  # This usually indicates a bug in the library.
  #
  # @example
  #   rescue Wreq::RustPanic => e
  #     puts "Internal error: #{e.message}"
  #     # Report this as a bug
  #   end
  unless const_defined?(:RustPanic)
    class RustPanic < Error; end
  end

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
  unless const_defined?(:ConnectionError)
    class ConnectionError < Error; end
  end

  # Connection was reset by the server.
  #
  # Raised when the server closes the connection unexpectedly.
  #
  # @example
  #   rescue Wreq::ConnectionResetError => e
  #     puts "Connection reset: #{e.message}"
  #   end
  unless const_defined?(:ConnectionResetError)
    class ConnectionResetError < ConnectionError; end
  end

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
    class TlsError < Error; end
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
  unless const_defined?(:RequestError)
    class RequestError < Error; end
  end

  # HTTP status code indicates an error.
  #
  # Raised when the server returns an error status code (4xx or 5xx).
  #
  # @example
  #   begin
  #     response = client.get("https://httpbin.org/status/404")
  #   rescue Wreq::StatusError => e
  #     puts "HTTP error: #{e.message}"
  #     # e.response contains the full response
  #   end
  unless const_defined?(:StatusError)
    class StatusError < Error; end
  end

  # Redirect handling failed.
  #
  # Raised when too many redirects occur or redirect logic fails.
  #
  # @example
  #   begin
  #     client = Wreq::Client.new(max_redirects: 3)
  #     client.get("https://httpbin.org/redirect/10")
  #   rescue Wreq::RedirectError => e
  #     puts "Too many redirects: #{e.message}"
  #   end
  unless const_defined?(:RedirectError)
    class RedirectError < Error; end
  end

  # Request timed out.
  #
  # Raised when the request exceeds the configured timeout.
  #
  # @example
  #   begin
  #     client = Wreq::Client.new(timeout: 5)
  #     client.get("https://httpbin.org/delay/10")
  #   rescue Wreq::TimeoutError => e
  #     puts "Request timed out: #{e.message}"
  #     retry_with_longer_timeout
  #   end
  unless const_defined?(:TimeoutError)
    class TimeoutError < Error; end
  end

  # Data processing and encoding errors

  # Response body processing failed.
  #
  # Raised when there's an error reading or processing the response body.
  #
  # @example
  #   rescue Wreq::BodyError => e
  #     puts "Body error: #{e.message}"
  #   end
  unless const_defined?(:BodyError)
    class BodyError < Error; end
  end

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
  unless const_defined?(:DecodingError)
    class DecodingError < Error; end
  end

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
  unless const_defined?(:BuilderError)
    class BuilderError < Error; end
  end

  # Input validation and parsing errors

  # URL parsing failed.
  #
  # Raised when a provided URL is malformed or invalid.
  #
  # @example
  #   begin
  #     client.get("not a valid url")
  #   rescue Wreq::URLParseError => e
  #     puts "Invalid URL: #{e.message}"
  #   end
  unless const_defined?(:URLParseError)
    class URLParseError < Error; end
  end
end