require "test_helper"
require "socket"
require "timeout"

class ErrorHierarchyTest < Minitest::Test
  REGULAR_ERROR_NAMES = %i[
    MemoryError
    ForkError
    ConnectionError
    ProxyConnectionError
    ConnectionResetError
    TlsError
    RequestError
    StatusError
    RedirectError
    TimeoutError
    BodyError
    DecodingError
    BuilderError
  ].freeze
  NATIVE_ERROR_PREDICATES = %i[
    is_builder
    is_redirect
    is_status
    is_timeout
    is_request
    is_connect
    is_proxy_connect
    is_connection_reset
    is_body
    is_tls
    is_decode
    is_upgrade
  ].freeze

  def test_regular_errors_share_stable_root
    assert_equal RuntimeError, Wreq::Error.superclass
    assert_operator Wreq::Error, :<, StandardError

    REGULAR_ERROR_NAMES.each do |name|
      assert_equal Wreq::Error, Wreq.const_get(name).superclass
    end

    error = Wreq::MemoryError.new
    assert_nil error.uri
    assert_nil error.status
    NATIVE_ERROR_PREDICATES.each do |predicate|
      assert_equal false, error.public_send(predicate)
    end
  end

  def test_root_and_specific_errors_can_be_rescued
    root_error = begin
      Wreq.get("not-a-valid-url")
    rescue Wreq::Error => error
      error
    end

    assert_instance_of Wreq::BuilderError, root_error
    assert root_error.is_builder
    assert_nil root_error.status
    assert_equal [:is_builder], active_native_predicates(root_error)
    assert_raises(Wreq::BuilderError) { Wreq.get("not-a-valid-url") }
  end

  def test_binding_generated_errors_have_no_native_predicates
    error = assert_raises(Wreq::BuilderError) { Wreq::Headers.new(Object.new) }

    assert_empty active_native_predicates(error)
  end

  def test_native_error_predicates_are_not_mutually_exclusive
    with_hanging_server do |url, _accepted|
      error = assert_raises(Wreq::TimeoutError) { Wreq.get(url, timeout: 1) }

      assert error.is_timeout
      assert error.is_request
    end
  end

  def test_interrupt_error_stays_outside_standard_error
    assert_equal Interrupt, Wreq::InterruptError.superclass
    refute_operator Wreq::InterruptError, :<, StandardError

    error = assert_raises(Interrupt) do
      raise Wreq::InterruptError, "request interrupted"
    end
    assert_instance_of Wreq::InterruptError, error
  end

  def test_request_interruption_raises_interrupt_error
    request_thread = nil
    with_hanging_server do |url, accepted|
      request_thread = Thread.new do
        Wreq.get(url, timeout: 60)
      rescue Interrupt, StandardError => error
        error
      end
      request_thread.report_on_exception = false

      Timeout.timeout(5) { accepted.pop }
      request_thread.wakeup

      assert request_thread.join(5), "Interrupted request thread should stop"

      error = request_thread.value
      assert_instance_of Wreq::InterruptError, error
      refute_kind_of StandardError, error
    end
  ensure
    request_thread&.kill
    request_thread&.join(1)
  end

  def test_raise_for_status_returns_same_non_error_response
    {200 => "ok", 302 => "redirect"}.each do |status, body|
      with_status_server(status, body:) do |url|
        response = Wreq.get(url)

        assert_same response, response.raise_for_status!
        assert_equal body, response.text
      end
    end
  end

  def test_raise_for_status_exposes_status_without_consuming_body
    {404 => "missing", 503 => "unavailable"}.each do |status, body|
      with_status_server(status, body:) do |url|
        response = Wreq.get("#{url}?token=response-secret#fragment-secret")
        error = assert_raises(Wreq::StatusError) { response.raise_for_status! }

        assert_kind_of Wreq::Error, error
        assert_instance_of Integer, error.status
        assert_equal status, error.status
        assert_equal response.url, error.uri
        assert_predicate error.uri, :frozen?
        assert error.is_status
        assert_equal [:is_status], active_native_predicates(error)
        refute_respond_to error, :kind
        refute_respond_to error, :retryable?
        refute_includes error.message, "response-secret"
        refute_includes error.inspect, "response-secret"
        assert_equal body, response.text
      end
    end
  end

  def test_native_error_messages_hide_sensitive_request_data
    port = closed_local_port
    error = assert_raises(Wreq::Error) do
      Wreq.get(
        "http://uri-user:uri-password@127.0.0.1:#{port}/private?token=query-secret#fragment-secret",
        proxy: "http://proxy-user:proxy-secret@127.0.0.1:#{port}",
        headers: {"Authorization" => "Bearer authorization-secret"},
        cookies: {"session" => "cookie-secret"},
        timeout: 1
      )
    end

    assert_instance_of String, error.uri
    assert_predicate error.uri, :frozen?
    assert_includes error.uri, "query-secret"
    assert NATIVE_ERROR_PREDICATES.any? { |predicate| error.public_send(predicate) }
    assert_nil error.status
    NATIVE_ERROR_PREDICATES.each do |predicate|
      assert_includes [true, false], error.public_send(predicate)
    end

    [
      "uri-user",
      "uri-password",
      "query-secret",
      "fragment-secret",
      "proxy-user",
      "proxy-secret",
      "authorization-secret",
      "cookie-secret"
    ].each do |secret|
      refute_includes error.message, secret
      refute_includes error.inspect, secret
    end
  end

  def test_option_conversion_preserves_exception_cause
    source = Object.new
    source.define_singleton_method(:to_a) do
      raise ArgumentError, "original conversion failure"
    rescue ArgumentError => cause
      raise IOError, "header conversion failed", cause: cause
    end

    error = assert_raises(IOError) { Wreq::Client.new(headers: source) }

    assert_includes error.message, ":headers"
    assert_instance_of ArgumentError, error.cause
    assert_equal "original conversion failure", error.cause.message
  end

  def test_option_context_preserves_native_error_metadata
    error = assert_raises(Wreq::BuilderError) { Wreq::Client.new(proxy: "://") }

    assert_includes error.message, ":proxy"
    assert_equal [:is_builder], active_native_predicates(error)
  end

  private

  def active_native_predicates(error)
    NATIVE_ERROR_PREDICATES.select { |predicate| error.public_send(predicate) }
  end

  def closed_local_port
    server = TCPServer.new("127.0.0.1", 0)
    server.addr[1]
  ensure
    server&.close
  end

  def with_hanging_server
    server = TCPServer.new("127.0.0.1", 0)
    accepted = Queue.new
    thread = Thread.new do
      socket = server.accept
      accepted << true
      sleep
    rescue IOError, SystemCallError
      nil
    ensure
      socket&.close unless socket&.closed?
    end
    thread.report_on_exception = false

    yield "http://127.0.0.1:#{server.addr[1]}/", accepted
  ensure
    server&.close unless server&.closed?
    thread&.kill
    thread&.join(1)
  end

  def with_status_server(status, body: "")
    reason = {
      200 => "OK",
      204 => "No Content",
      302 => "Found",
      404 => "Not Found",
      503 => "Service Unavailable"
    }.fetch(status)
    server = TCPServer.new("127.0.0.1", 0)
    port = server.addr[1]
    thread = Thread.new do
      socket = server.accept
      begin
        while (line = socket.gets)
          break if line == "\r\n"
        end
        socket.write "HTTP/1.1 #{status} #{reason}\r\n"
        socket.write "Content-Type: text/plain\r\n"
        socket.write "Content-Length: #{body.bytesize}\r\n"
        socket.write "Connection: close\r\n\r\n"
        socket.write body
      ensure
        socket.close unless socket.closed?
      end
    rescue IOError, SystemCallError
      nil
    ensure
      server.close unless server.closed?
    end
    thread.report_on_exception = false

    yield "http://127.0.0.1:#{port}/"
  ensure
    server&.close unless server&.closed?
    thread&.join(5)
  end
end
