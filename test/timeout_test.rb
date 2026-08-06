# frozen_string_literal: true

require "test_helper"
require "socket"

class TimeoutTest < Minitest::Test
  SUBSECOND_TIMEOUT = 0.25
  SERVER_DELAY = 1.2

  def test_client_duration_options_accept_numeric_seconds
    [1, 0.125, Rational(1, 8), 0, nil].each do |value|
      options = client_duration_options.to_h { |name| [name, value] }

      assert_instance_of Wreq::Client, Wreq::Client.new(**options)
    end
  end

  def test_each_client_duration_option_rejects_negative_seconds
    client_duration_options.each do |name|
      error = assert_raises(ArgumentError) do
        Wreq::Client.new(**{name => -0.25})
      end

      assert_includes error.message, ":#{name}"
    end
  end

  def test_duration_rejects_invalid_numeric_seconds
    invalid_values = [
      -1,
      Float::NAN,
      Float::INFINITY,
      -Float::INFINITY,
      Float::MAX,
      2**256
    ]

    invalid_values.each do |value|
      error = assert_raises(ArgumentError) do
        Wreq::Client.new(timeout: value)
      end

      assert_includes error.message, ":timeout"
    end
  end

  def test_duration_rejects_non_numeric_values
    error = assert_raises(TypeError) do
      Wreq::Client.new(timeout: "0.25")
    end

    assert_includes error.message, ":timeout"
  end

  def test_request_duration_options_reject_invalid_values_before_network_io
    invalid_values = [
      -1,
      -0.25,
      Float::NAN,
      Float::INFINITY,
      -Float::INFINITY,
      Float::MAX,
      2**256,
      "0.25"
    ]

    %i[timeout read_timeout].each do |name|
      invalid_values.each do |value|
        error_class = value.is_a?(String) ? TypeError : ArgumentError
        error = assert_raises(error_class) do
          Wreq.get("not a url", **{name => value})
        end

        assert_includes error.message, ":#{name}"
      end
    end
  end

  def test_request_timeouts_accept_integer_and_nil_values
    with_http_server do |url|
      response = Wreq.get(url, timeout: 1, read_timeout: 1)

      assert_equal 200, response.code
      assert_equal "ok", response.text
    end

    with_http_server do |url|
      response = Wreq.get(url, timeout: nil, read_timeout: nil)

      assert_equal 200, response.code
      assert_equal "ok", response.text
    end
  end

  def test_client_fractional_timeout_preserves_subsecond_value
    client = Wreq::Client.new(timeout: SUBSECOND_TIMEOUT)

    with_http_server(response_delay: SERVER_DELAY) do |url|
      assert_fractional_timeout { client.get(url) }
    end
  end

  def test_request_timeout_override_preserves_subsecond_value
    client = Wreq::Client.new(timeout: 2)

    with_http_server(response_delay: SERVER_DELAY) do |url|
      assert_fractional_timeout do
        client.get(url, timeout: SUBSECOND_TIMEOUT)
      end
    end
  end

  def test_request_read_timeout_override_preserves_subsecond_value
    client = Wreq::Client.new(read_timeout: 2)

    with_http_server(body_delay: SERVER_DELAY) do |url|
      assert_fractional_timeout do
        client.get(url, read_timeout: SUBSECOND_TIMEOUT).text
      end
    end
  end

  def test_zero_request_timeouts_expire_immediately
    client = Wreq::Client.new

    with_http_server(response_delay: SERVER_DELAY) do |url|
      assert_immediate_timeout { client.get(url, timeout: 0) }
    end

    with_http_server(body_delay: SERVER_DELAY) do |url|
      assert_immediate_timeout do
        client.get(url, read_timeout: 0).text
      end
    end
  end

  private

  def client_duration_options
    options = %i[
      timeout
      connect_timeout
      read_timeout
      tcp_keepalive
      tcp_keepalive_interval
      pool_idle_timeout
    ]
    if RUBY_PLATFORM.match?(/linux|android|fuchsia/)
      options << :tcp_user_timeout
    end
    options
  end

  def assert_fractional_timeout
    elapsed = measure_elapsed do
      assert_raises(Wreq::TimeoutError) { yield }
    end

    assert_operator elapsed, :>=, 0.1,
      "Fractional timeout fired too early after #{elapsed.round(3)} seconds"
    assert_operator elapsed, :<, 0.8,
      "Fractional timeout fired too late after #{elapsed.round(3)} seconds"
  end

  def assert_immediate_timeout
    elapsed = measure_elapsed do
      assert_raises(Wreq::TimeoutError) { yield }
    end

    assert_operator elapsed, :<, 0.5,
      "Zero timeout did not expire immediately (#{elapsed.round(3)} seconds)"
  end

  def measure_elapsed
    started_at = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    yield
    Process.clock_gettime(Process::CLOCK_MONOTONIC) - started_at
  end

  def with_http_server(response_delay: 0, body_delay: 0)
    server = TCPServer.new("127.0.0.1", 0)
    thread = Thread.new do
      socket = server.accept
      read_request(socket)
      sleep response_delay if response_delay.positive?

      socket.write(
        "HTTP/1.1 200 OK\r\n" \
        "Content-Length: 2\r\n" \
        "Connection: close\r\n\r\n"
      )
      sleep body_delay if body_delay.positive?
      socket.write("ok")
    rescue IOError, SystemCallError
      nil
    ensure
      socket&.close
      server.close unless server.closed?
    end
    thread.report_on_exception = false

    yield "http://127.0.0.1:#{server.addr[1]}/"
  ensure
    server&.close unless server&.closed?
    thread&.kill
    thread&.join(1)
  end

  def read_request(socket)
    while (line = socket.gets)
      break if line == "\r\n"
    end
  end
end
