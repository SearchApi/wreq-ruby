# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # Base class for regular errors raised by wreq-ruby.
    #
    # This remains a RuntimeError so existing broad rescue handlers continue
    # to work. Predicate methods describe a captured native wreq::Error rather
    # than the Ruby exception class, and more than one predicate may be true.
    # Errors created entirely by the binding return false for every predicate.
    class Error < RuntimeError
      # The URI associated with the native error.
      #
      # This explicit accessor may contain credentials or sensitive query
      # parameters. Error messages and inspection output omit it, so avoid
      # logging this value without redacting it first.
      #
      # @return [String, nil] frozen URI string, if one was recorded
      attr_reader :uri

      # The response status associated with the error.
      #
      # @return [Integer, nil] HTTP status code, if one was recorded
      attr_reader :status

      # @return [Boolean] whether the native error came from a builder
      def is_builder
      end

      # @return [Boolean] whether the native error came from redirect handling
      def is_redirect
      end

      # @return [Boolean] whether the native error represents an HTTP status
      def is_status
      end

      # @return [Boolean] whether the native error is related to a timeout
      def is_timeout
      end

      # @return [Boolean] whether the native error is related to a request
      def is_request
      end

      # @return [Boolean] whether the native error is related to connecting
      def is_connect
      end

      # @return [Boolean] whether the native error is related to proxy connection
      def is_proxy_connect
      end

      # @return [Boolean] whether the native error is a connection reset
      def is_connection_reset
      end

      # @return [Boolean] whether the native error is related to a body
      def is_body
      end

      # @return [Boolean] whether the native error is related to TLS
      def is_tls
      end

      # @return [Boolean] whether the native error is related to decoding
      def is_decode
      end

      # @return [Boolean] whether the native error is related to an upgrade
      def is_upgrade
      end
    end

    # Raised when a native request wait is interrupted.
    #
    # InterruptError stays outside StandardError so broad application rescues
    # do not swallow Ruby interrupts.
    class InterruptError < Interrupt; end

    # System-level and runtime errors

    # Memory allocation failed.
    class MemoryError < Error; end

    # The native extension was inherited from a parent process.
    #
    # Raised when wreq-ruby is used in a child process forked after the
    # extension was loaded. Its process-global native state cannot be reused
    # safely in the child.
    #
    # @example
    #   Process.fork do
    #     Wreq::Client.new # Raises when the parent loaded wreq-ruby.
    #   end
    class ForkError < Error; end

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
    class ConnectionError < Error; end

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
    class ProxyConnectionError < Error; end

    # Connection was reset by the server.
    #
    # Raised when the server closes the connection unexpectedly.
    #
    # @example
    #   rescue Wreq::ConnectionResetError => e
    #     puts "Connection reset: #{e.message}"
    #   end
    class ConnectionResetError < Error; end

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
    class TlsError < Error; end

    # HTTP protocol and request/response errors

    # Request failed.
    #
    # Generic error for request failures that don't fit other categories.
    #
    # @example
    #   rescue Wreq::RequestError => e
    #     puts "Request failed: #{e.message}"
    #   end
    class RequestError < Error; end

    # HTTP status code indicates an error.
    #
    # Raised by Response#raise_for_status! for a 4xx or 5xx response. Requests
    # continue to return these responses normally until that method is called.
    #
    # @example
    #   begin
    #     response = client.get("https://httpbin.io/status/404")
    #     response.raise_for_status!
    #   rescue Wreq::StatusError => e
    #     puts "HTTP #{e.status}: #{e.message}"
    #   end
    class StatusError < Error; end

    # Redirect handling failed.
    #
    # Raised when too many redirects occur or redirect logic fails.
    #
    # @example
    #   begin
    #     client = Wreq::Client.new(allow_redirects: true, max_redirects: 3)
    #     client.get("https://httpbin.io/redirect/10")
    #   rescue Wreq::RedirectError => e
    #     puts "Too many redirects: #{e.message}"
    #   end
    class RedirectError < Error; end

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
    class TimeoutError < Error; end

    # Data processing and encoding errors

    # Response body processing failed.
    #
    # Raised when there's an error reading or processing the response body.
    #
    # @example
    #   rescue Wreq::BodyError => e
    #     puts "Body error: #{e.message}"
    #   end
    class BodyError < Error; end

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
    class DecodingError < Error; end

    # Configuration and builder errors

    # A native client or request configuration could not be built.
    #
    # Raised when validated Ruby options cannot be represented by the native
    # builder or request body.
    #
    # @example
    #   begin
    #     client = Wreq::Client.new(proxy: "invalid://")
    #   rescue Wreq::BuilderError => e
    #     puts "Invalid configuration: #{e.message}"
    #   end
    class BuilderError < Error; end
  end
end
