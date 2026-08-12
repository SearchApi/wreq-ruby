# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # Base class for wreq-ruby runtime errors.
    #
    # Error remains a RuntimeError so existing rescue handlers keep working.
    # A native `wreq::Error` has one top-level kind and may contain a chain of
    # lower-level causes. The builder, body, TLS, decoding, redirect, status,
    # upgrade, and request predicates report that top-level kind. Timeout,
    # connection reset, proxy connection, and destination connection predicates
    # inspect the cause chain and may overlap with the request kind.
    #
    # The subclass records one primary category. For example, a destination
    # connection timeout normally raises TimeoutError while `timeout?`,
    # `connect?`, and `request?` can all return true. Top-level native kinds take
    # precedence over cause-chain details. Other failures use connection reset,
    # timeout, proxy connection, destination connection, then RequestError.
    #
    # Use the predicates when code needs every native fact. Errors created by
    # the binding return false for all of them. New facts may be exposed as
    # predicates without changing the exception class for existing failures.
    # The standard error message includes the native cause chain, so `warn`,
    # `message`, `to_s`, and uncaught exception output show the underlying
    # network error. `detailed_message` also includes the active facts, and
    # `full_message` adds the backtrace and Ruby exception causes. None of these
    # outputs includes `uri`.
    #
    # @example Rescue any wreq-ruby runtime error
    #   begin
    #     Wreq.get("not-a-valid-url")
    #   rescue Wreq::Error => error
    #     warn error
    #   end
    #
    # @see https://github.com/SearchApi/wreq-ruby/blob/main/docs/errors.md
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

      # This checks the top-level native error kind. Binding-side validation can
      # also raise BuilderError without setting this predicate.
      #
      # @return [Boolean] Whether the top-level native kind is Builder
      def builder?
      end

      # @return [Boolean] Whether the top-level native kind is Redirect
      def redirect?
      end

      # @return [Boolean] Whether the top-level native kind is Status
      def status?
      end

      # This scans the native cause chain for wreq's timeout marker, a protocol
      # timeout, or an operating-system timed-out error. A timeout can therefore
      # also be a connection, proxy connection, request, or body failure.
      #
      # @return [Boolean] Whether the native cause chain contains a timeout
      def timeout?
      end

      # This checks the top-level native kind, not every operation performed for
      # a request. It is commonly true on connection and timeout subclasses
      # because wreq wraps client-layer failures in a Request error.
      #
      # @return [Boolean] Whether the top-level native kind is Request
      def request?
      end

      # This scans the cause chain for wreq's destination Connect stage. The
      # root cause can be DNS, TCP, TLS handshake, or connection-pool failure.
      #
      # @return [Boolean] Whether the native chain contains a destination
      #   connection failure
      def connect?
      end

      # This scans the cause chain for proxy TCP, HTTP CONNECT tunnel, or SOCKS
      # negotiation failures. Errors after a tunnel is established may instead
      # belong to the destination Connect stage.
      #
      # @return [Boolean] Whether the native chain contains a proxy connection
      #   failure
      def proxy_connect?
      end

      # A clean EOF or an HTTP "connection closed before message completed"
      # error is not sufficient. The chain must retain an operating-system
      # ConnectionReset error.
      #
      # @return [Boolean] Whether the native chain contains a connection reset
      def connection_reset?
      end

      # This is narrower than any failure encountered while reading or writing a
      # body. It checks wreq's top-level Body kind.
      #
      # @return [Boolean] Whether the top-level native kind is Body
      def body?
      end

      # This reports TLS client setup, such as connector, trust store, identity,
      # or TLS option configuration. Certificate verification and TLS alerts
      # during a remote handshake normally appear under the Connect stage.
      #
      # @return [Boolean] Whether the top-level native kind is TLS
      def tls?
      end

      # The native Decode kind covers value parsing and response-body errors
      # that wreq maps through its decoder. It does not mean JSON parsing only.
      #
      # @return [Boolean] Whether the top-level native kind is Decode
      def decoding?
      end

      # @return [Boolean] Whether the top-level native kind is Upgrade
      def upgrade?
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

    # Raised when the native cause chain contains a destination Connect stage
    # and no higher-priority category applies.
    #
    # The root cause can be DNS resolution, TCP connection, connection-pool
    # acquisition, TLS negotiation, or certificate verification. Inspect the
    # complete error message for that root cause. A timeout raises TimeoutError
    # and keeps `connect?` true; a retained operating-system reset raises
    # ConnectionResetError instead.
    #
    # @example Handle a destination connection failure
    #   client = Wreq::Client.new(no_proxy: true)
    #   begin
    #     client.get("http://127.0.0.1:1")
    #   rescue Wreq::ConnectError => error
    #     warn "connection failed: #{error.message}"
    #   end
    class ConnectError < Error; end

    # Raised when the cause chain identifies a proxy connection stage and no
    # higher-priority category applies.
    #
    # This includes connecting to the proxy, negotiating an HTTP CONNECT tunnel,
    # and SOCKS negotiation. Once a tunnel is established, destination TLS or
    # connection failures can raise ConnectError instead. A timeout raises
    # TimeoutError while `proxy_connect?` remains true when wreq preserves the
    # proxy stage in its cause chain.
    #
    # @example Handle a proxy connection failure
    #   begin
    #     Wreq.get(
    #       "https://example.com",
    #       proxy: "http://127.0.0.1:1"
    #     )
    #   rescue Wreq::ProxyConnectError => error
    #     warn "proxy connection failed: #{error.message}"
    #   end
    class ProxyConnectError < Error; end

    # Raised when the native cause chain retains an operating-system connection
    # reset and no top-level native kind takes precedence.
    #
    # An EOF, graceful close, or incomplete HTTP message is not necessarily a
    # connection reset. Those failures can raise RequestError or DecodingError
    # when no `io::ErrorKind::ConnectionReset` remains in the chain.
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

    # Raised when wreq records a top-level TLS setup error.
    #
    # This covers creating the TLS connector and configuring trust stores,
    # identities, certificate compression, key logging, or TLS options. The
    # current Ruby API does not expose certificate or identity inputs that can
    # deliberately trigger every setup path. Remote handshake alerts and
    # certificate verification failures normally raise ConnectError because
    # they occur after setup while acquiring a connection.
    #
    # @example Distinguish TLS setup errors from connection errors
    #   client = Wreq::Client.new(no_proxy: true, verify: true)
    #   begin
    #     client.get("https://expired.badssl.com/")
    #   rescue Wreq::TlsError => error
    #     warn "TLS setup failed: #{error.message}"
    #   rescue Wreq::ConnectError => error
    #     warn "TLS connection failed: #{error.message}"
    #   end
    class TlsError < Error; end

    # Raised for a top-level native Request error when its cause chain has no
    # more specific transport category.
    #
    # Typical causes include a request rejected by the HTTP client, a send
    # failure, an incomplete response, or a closed connection represented as a
    # protocol error rather than an operating-system reset. Connection reset,
    # timeout, proxy connection, and destination connection causes use their
    # corresponding subclasses before this fallback.
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
    # The native Status kind has no lower-level source. The inherited `status`
    # reader returns the integer HTTP status.
    #
    # @example
    #   client = Wreq::Client.new(no_proxy: true)
    #   begin
    #     client.get("https://example.testserver.host/status/404").raise_for_status!
    #   rescue Wreq::StatusError => error
    #     warn "HTTP #{error.status}: #{error.message}"
    #   end
    class StatusError < Error; end

    # Raised when wreq records a top-level redirect policy error, such as after
    # too many redirects. `uri` contains the last redirect target when wreq
    # recorded one.
    #
    # @example Limit the number of redirects
    #   client = Wreq::Client.new(allow_redirects: true, max_redirects: 3)
    #   begin
    #     client.get("https://httpbin.io/redirect/10")
    #   rescue Wreq::RedirectError => error
    #     warn "redirect failed: #{error.message}"
    #   end
    class RedirectError < Error; end

    # Raised when the cause chain contains a timeout and no top-level native kind
    # or connection reset takes precedence.
    #
    # wreq recognizes its own timeout marker, protocol timeouts, and
    # `io::ErrorKind::TimedOut`. This includes overall request, response read,
    # destination connection, and proxy connection timeouts. Check `connect?`,
    # `proxy_connect?`, `request?`, and `body?` for the retained stage. A native
    # Body timeout raises BodyError because the top-level Body kind wins.
    #
    # @example Handle a request timeout
    #   client = Wreq::Client.new(no_proxy: true, timeout: 1)
    #   begin
    #     client.get("https://example.testserver.host/delay/5")
    #   rescue Wreq::TimeoutError => error
    #     warn "request timed out: #{error.message}"
    #   end
    class TimeoutError < Error; end

    # Raised for wreq's top-level Body kind or a binding-side body sender state
    # error.
    #
    # This is not the class for every failure encountered while transferring a
    # body. In the current wreq version, a total timeout while consuming a
    # response body uses the Body kind, while read timeouts and protocol errors
    # can use TimeoutError, RequestError, or DecodingError. Binding-side errors
    # do not set `body?` because they have no native wreq error.
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

    # Raised when wreq cannot decode or parse a value, or when it wraps a
    # response-body transport or protocol failure in its Decode kind.
    #
    # The complete message identifies whether the root cause is JSON, character
    # or cookie parsing, decompression, an HTTP body failure, or another decoder.
    # A timeout or connection reset can remain visible through its predicate
    # even when the primary class is DecodingError.
    #
    # @example Fall back to bytes when a response is not valid JSON
    #   response = Wreq.get("https://example.com")
    #   begin
    #     data = response.json
    #   rescue Wreq::DecodingError
    #     data = response.bytes
    #   end
    class DecodingError < Error; end

    # Raised for wreq's top-level Builder kind or binding-side validation that
    # cannot initialize the native runtime or construct a client, request, URL,
    # header, or JSON body.
    #
    # Binding-generated BuilderError instances have no native source chain, so
    # `builder?` returns false for them. The exception message still describes
    # the rejected value.
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

# ======================== Ruby API Extensions ========================

module Wreq
  class Error
    NATIVE_DETAILS = {
      builder?: :builder,
      body?: :body,
      tls?: :tls,
      decoding?: :decoding,
      redirect?: :redirect,
      status?: :status,
      upgrade?: :upgrade,
      connection_reset?: :connection_reset,
      timeout?: :timeout,
      proxy_connect?: :proxy_connect,
      connect?: :connect,
      request?: :request
    }.freeze
    private_constant :NATIVE_DETAILS

    # Return Ruby's detailed exception message with active native error facts.
    #
    # More specific facts appear before the connection and request stages.
    # `full_message` calls this method and adds the backtrace and exception
    # causes. The recorded URI is deliberately omitted because it may contain
    # credentials or other sensitive values.
    #
    # @param highlight [Boolean] Whether Ruby should add terminal highlighting
    # @param options [Hash] Additional options accepted by Exception
    # @return [String] Detailed message suitable for diagnostic output
    def detailed_message(highlight: false, **options)
      message = super
      details = NATIVE_DETAILS.filter_map do |predicate, label|
        label if public_send(predicate)
      end

      details.empty? ? message : "#{message}\n    wreq: #{details.inspect}"
    end
  end
end
