# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # Base class for wreq-ruby runtime errors.
    #
    # Error remains a RuntimeError so existing rescue handlers keep working.
    # The `is_*` methods mirror predicates on the captured native `wreq::Error`.
    # One error can match more than one predicate. Errors created by the binding
    # itself return false for all of them.
    #
    # @example Rescue any wreq-ruby runtime error
    #   begin
    #     Wreq.get("not-a-valid-url")
    #   rescue Wreq::Error => error
    #     warn "#{error.class}: #{error.message}"
    #   end
    class Error < RuntimeError
      # Get the URI recorded by the native error.
      #
      # This value may contain credentials, query parameters, or fragments.
      # Error messages and `inspect` omit the URI. Redact it before logging it
      # explicitly.
      #
      # @return [String, nil] Frozen URI string, if one was recorded
      attr_reader :uri

      # Get the HTTP status recorded by the native error.
      #
      # @return [Integer, nil] HTTP status code, if one was recorded
      attr_reader :status

      # @return [Boolean] Whether the native error came from a builder
      def is_builder
      end

      # @return [Boolean] Whether the native error came from redirect handling
      def is_redirect
      end

      # @return [Boolean] Whether the native error represents an HTTP status
      def is_status
      end

      # @return [Boolean] Whether the native error is related to a timeout
      def is_timeout
      end

      # @return [Boolean] Whether the native error is related to a request
      def is_request
      end

      # @return [Boolean] Whether the native error is related to connecting
      def is_connect
      end

      # @return [Boolean] Whether the native error is related to a proxy connection
      def is_proxy_connect
      end

      # @return [Boolean] Whether the native error is a connection reset
      def is_connection_reset
      end

      # @return [Boolean] Whether the native error is related to a body
      def is_body
      end

      # @return [Boolean] Whether the native error is related to TLS
      def is_tls
      end

      # @return [Boolean] Whether the native error is related to decoding
      def is_decode
      end

      # @return [Boolean] Whether the native error is related to an upgrade
      def is_upgrade
      end
    end

    # Raised when Ruby interrupts a native request wait.
    #
    # This inherits from Interrupt instead of Error, so `rescue StandardError`
    # does not swallow the interrupt.
    class InterruptError < Interrupt; end

    # Raised when single-use native state was already consumed or is borrowed.
    class MemoryError < Error; end

    # Raised when a forked child tries to use inherited native state.
    #
    # Tokio worker threads and pooled connections cannot be reused after fork.
    # @see https://github.com/SearchApi/wreq-ruby/blob/main/docs/fork-safety.md
    class ForkError < Error; end

    # Raised when the client cannot connect to the destination server.
    class ConnectionError < Error; end

    # Raised when the client cannot connect to the configured proxy.
    class ProxyConnectionError < Error; end

    # Raised when a peer resets the connection.
    class ConnectionResetError < Error; end

    # Raised when TLS negotiation or certificate verification fails.
    class TlsError < Error; end

    # Raised for a request failure without a more specific error subclass.
    class RequestError < Error; end

    # Raised when Response#raise_for_status! sees a 4xx or 5xx response.
    #
    # Requests return error responses normally until this opt-in check is made.
    # The inherited `status` reader returns the integer HTTP status.
    #
    # @example
    #   begin
    #     client.get("https://example.com/missing").raise_for_status!
    #   rescue Wreq::StatusError => error
    #     warn "HTTP #{error.status}: #{error.message}"
    #   end
    class StatusError < Error; end

    # Raised when redirect handling fails, such as after too many redirects.
    class RedirectError < Error; end

    # Raised when a request operation exceeds its timeout.
    class TimeoutError < Error; end

    # Raised while sending, reading, or streaming an HTTP body.
    class BodyError < Error; end

    # Raised when a response body cannot be decoded or parsed.
    class DecodingError < Error; end

    # Raised when client, request, header, or body configuration is invalid.
    class BuilderError < Error; end
  end
end
