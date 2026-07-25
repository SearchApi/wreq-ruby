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
    #     warn "invalid request" if error.is_builder
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
    #
    # @example Handle an interrupted request separately
    #   begin
    #     Wreq.get("https://example.com", timeout: 30)
    #   rescue Wreq::InterruptError
    #     warn "request interrupted"
    #   rescue Wreq::Error => error
    #     warn error.message
    #   end
    # Keep interruption outside StandardError so a broad transport rescue
    # never swallows a Ruby interrupt.
    class InterruptError < Interrupt; end

    # Raised when single-use native state was already consumed or is borrowed.
    #
    # @example A closed response no longer has a readable body
    #   response = Wreq.get("https://example.com")
    #   response.close
    #   response.bytes # Raises Wreq::MemoryError
    class MemoryError < Error; end

    # Raised when a forked child tries to use inherited native state.
    #
    # Tokio worker threads and pooled connections cannot be reused after fork.
    #
    # @example Native operations are rejected in an inherited child
    #   pid = Process.fork do
    #     begin
    #       Wreq::Client.new
    #     rescue Wreq::ForkError => error
    #       warn error.message
    #     end
    #   end
    #   Process.wait(pid)
    #
    # @see https://github.com/SearchApi/wreq-ruby/blob/main/docs/fork-safety.md
    class ForkError < Error; end

    # Raised when the client cannot connect to the destination server.
    #
    # The error reflects the layer that actually fails. If a system proxy or
    # VPN accepts the connection but does not return a response, the request
    # raises Wreq::TimeoutError instead.
    #
    # @example Handle a destination connection failure
    #   client = Wreq::Client.new(no_proxy: true)
    #   begin
    #     client.get("http://127.0.0.1:1")
    #   rescue Wreq::ConnectionError => error
    #     warn "connection failed: #{error.message}"
    #   end
    class ConnectionError < Error; end

    # Raised when the client cannot connect to the configured proxy.
    #
    # @example Handle a proxy connection failure
    #   begin
    #     Wreq.get(
    #       "https://example.com",
    #       proxy: "http://127.0.0.1:1"
    #     )
    #   rescue Wreq::ProxyConnectionError => error
    #     warn "proxy connection failed: #{error.message}"
    #   end
    class ProxyConnectionError < Error; end

    # Raised when a peer resets the connection.
    #
    # @example Handle a reset while streaming a response
    #   response = Wreq.get("https://example.com")
    #   begin
    #     File.open("response.bin", "wb") do |file|
    #       response.chunks { |chunk| file.write(chunk) }
    #     end
    #   rescue Wreq::ConnectionResetError => error
    #     warn "connection reset: #{error.message}"
    #   end
    class ConnectionResetError < Error; end

    # Raised when native TLS setup fails while constructing a client.
    #
    # The current Ruby API does not expose certificate or identity inputs that
    # can deliberately trigger this error. TLS handshake and certificate
    # verification failures happen while connecting and normally raise
    # Wreq::ConnectionError instead.
    #
    # @example Distinguish TLS setup errors from connection errors
    #   begin
    #     Wreq::Client.new(verify: true).get("https://example.com")
    #   rescue Wreq::TlsError => error
    #     warn "TLS setup failed: #{error.message}"
    #   rescue Wreq::ConnectionError => error
    #     warn "TLS connection failed: #{error.message}"
    #   end
    class TlsError < Error; end

    # Raised for a request failure without a more specific error subclass.
    #
    # @example Rescue the native fallback request category
    #   client = Wreq::Client.new
    #   begin
    #     client.get("https://example.com")
    #   rescue Wreq::RequestError => error
    #     warn "request failed: #{error.message}"
    #   end
    class RequestError < Error; end

    # Raised when Response#raise_for_status! sees a 4xx or 5xx response.
    #
    # Requests return error responses normally until this opt-in check is made.
    # The inherited `status` reader returns the integer HTTP status.
    #
    # @example
    #   client = Wreq::Client.new
    #   begin
    #     client.get("https://httpbin.io/status/404").raise_for_status!
    #   rescue Wreq::StatusError => error
    #     warn "HTTP #{error.status}: #{error.message}"
    #   end
    class StatusError < Error; end

    # Raised when redirect handling fails, such as after too many redirects.
    #
    # @example Limit the number of redirects
    #   client = Wreq::Client.new(allow_redirects: true, max_redirects: 3)
    #   begin
    #     client.get("https://httpbin.io/redirect/10")
    #   rescue Wreq::RedirectError => error
    #     warn "redirect failed: #{error.message}"
    #   end
    class RedirectError < Error; end

    # Raised when a request operation exceeds its timeout.
    #
    # @example Handle a request timeout
    #   client = Wreq::Client.new(timeout: 1)
    #   begin
    #     client.get("https://httpbin.io/delay/10")
    #   rescue Wreq::TimeoutError => error
    #     warn "request timed out: #{error.message}"
    #   end
    class TimeoutError < Error; end

    # Raised while sending, reading, or streaming an HTTP body.
    #
    # @example Handle a body error while streaming
    #   response = Wreq.get("https://example.com")
    #   begin
    #     File.open("response.bin", "wb") do |file|
    #       response.chunks { |chunk| file.write(chunk) }
    #     end
    #   rescue Wreq::BodyError => error
    #     warn "body failed: #{error.message}"
    #   end
    class BodyError < Error; end

    # Raised when a response body cannot be decoded or parsed.
    #
    # @example Fall back to bytes when a response is not valid JSON
    #   response = Wreq.get("https://example.com")
    #   begin
    #     data = response.json
    #   rescue Wreq::DecodingError
    #     data = response.bytes
    #   end
    class DecodingError < Error; end

    # Raised when client, request, header, or body configuration is invalid.
    #
    # @example Handle an invalid request URL
    #   begin
    #     Wreq.get("not-a-valid-url")
    #   rescue Wreq::BuilderError => error
    #     warn "invalid request: #{error.message}"
    #   end
    class BuilderError < Error; end
  end
end
