require "test_helper"
require "open3"
require "rbconfig"
require "socket"

class BodySenderTest < Minitest::Test
  def test_valid_capacity_creates_open_sender
    sender = Wreq::BodySender.new(2)

    refute_predicate sender, :closed?
  ensure
    sender&.close
  end

  def test_omitted_capacity_defaults_to_eight
    sender = Wreq::BodySender.new
    progress = Queue.new
    producer = Thread.new do
      9.times do |index|
        sender.push(index.to_s)
        progress << index
      end
      sender.close
    end
    producer.report_on_exception = false

    assert wait_until { progress.size >= 8 }, "the default channel should buffer eight chunks"
    assert_equal 8, progress.size
    assert_predicate producer, :alive?, "the ninth push should wait for channel capacity"

    with_request_body_server do |url, request_thread|
      response = Wreq.post(url, body: sender)

      assert_equal 200, response.code
      assert producer.join(5), "the ninth push should finish once the request drains"
      assert_equal "012345678", request_thread.value
    end
  ensure
    producer&.kill if producer&.alive?
    producer&.join(5)
    sender&.close
  end

  def test_non_positive_and_excessive_capacities_raise_argument_error
    [0, -1, 2**256].each do |capacity|
      error = assert_raises(ArgumentError) { Wreq::BodySender.new(capacity) }
      assert_match(/capacity/, error.message)
    end
  end

  def test_non_integer_capacities_raise_type_error
    [1.0, "not an integer"].each do |capacity|
      error = assert_raises(TypeError) { Wreq::BodySender.new(capacity) }
      assert_match(/capacity/, error.message)
    end
  end

  def test_too_many_constructor_arguments_raise_argument_error
    assert_raises(ArgumentError) { Wreq::BodySender.new(1, 2) }
  end

  def test_close_is_idempotent_and_updates_closed_state
    sender = Wreq::BodySender.new

    refute_predicate sender, :closed?
    assert_nil sender.close
    assert_predicate sender, :closed?
    assert_nil sender.close
  end

  def test_push_after_close_raises_io_error
    sender = Wreq::BodySender.new
    sender.close

    error = assert_raises(IOError) { sender.push("data") }
    assert_equal "closed body sender", error.message
  end

  def test_queued_chunks_survive_close_before_request_attachment
    sender = Wreq::BodySender.new(2)
    sender.push("queued-")
    sender.push("body")
    sender.close

    with_request_body_server do |url, request_thread|
      response = Wreq.post(url, body: sender)

      assert_equal 200, response.code
      assert_equal "queued-body", request_thread.value
    end
  end

  def test_valid_capacity_keeps_backpressure_until_request_drains
    sender = Wreq::BodySender.new(1)
    sender.push("first-")
    producer = Thread.new do
      sender.push("second")
      sender.close
    end
    producer.report_on_exception = false

    sleep 0.05
    assert_predicate producer, :alive?, "the second push should wait for channel capacity"

    with_request_body_server do |url, request_thread|
      response = Wreq.post(url, body: sender)

      assert_equal 200, response.code
      assert producer.join(5), "the blocked producer should finish once the request drains"
      assert_equal "first-second", request_thread.value
    end
  ensure
    producer&.kill if producer&.alive?
    producer&.join(5)
    sender&.close
  end

  def test_receiver_termination_closes_sender
    sender = Wreq::BodySender.new(1)
    sender.push("data")

    with_reset_server do |url|
      assert_raises(StandardError) { Wreq.post(url, body: sender, timeout: 2) }
    end

    assert wait_until { sender.closed? }, "sender should close after the request drops its receiver"
    assert_raises(IOError) { sender.push("more") }
  ensure
    sender&.close
  end

  def test_zero_capacity_regression_exits_subprocess_normally
    lib_dir = File.expand_path("../lib", __dir__)
    script = <<~RUBY
      require "wreq"

      begin
        Wreq::BodySender.new(0)
      rescue ArgumentError => error
        warn "\#{error.class}: \#{error.message}"
        exit 0
      end

      warn "zero capacity did not raise"
      exit 2
    RUBY

    _stdout, stderr, status = Open3.capture3(RbConfig.ruby, "-I", lib_dir, "-e", script)

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_match(/ArgumentError:.*capacity/, stderr)
    refute_match(/panicked|mpsc bounded channel/i, stderr)
  end

  private

  def wait_until(timeout: 2)
    deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + timeout
    loop do
      return true if yield
      return false if Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline

      sleep 0.01
    end
  end

  def with_request_body_server
    server = TCPServer.new("127.0.0.1", 0)
    port = server.addr[1]
    request_thread = Thread.new do
      socket = server.accept
      begin
        socket.gets
        headers = read_headers(socket)
        body = read_request_body(socket, headers)
        socket.write "HTTP/1.1 200 OK\r\n"
        socket.write "Content-Length: 0\r\n"
        socket.write "Connection: close\r\n\r\n"
        body
      ensure
        socket.close unless socket.closed?
      end
    ensure
      server.close unless server.closed?
    end
    request_thread.report_on_exception = false

    yield "http://127.0.0.1:#{port}/", request_thread
  ensure
    server&.close unless server&.closed?
    request_thread&.join(5)
  end

  def with_reset_server
    server = TCPServer.new("127.0.0.1", 0)
    port = server.addr[1]
    thread = Thread.new do
      socket = server.accept
      socket.close
    ensure
      server.close unless server.closed?
    end
    thread.report_on_exception = false

    yield "http://127.0.0.1:#{port}/"
  ensure
    server&.close unless server&.closed?
    thread&.join(5)
  end

  def read_headers(socket)
    headers = {}
    while (line = socket.gets)
      break if line == "\r\n"

      name, value = line.split(":", 2)
      headers[name.downcase] = value.strip
    end
    headers
  end

  def read_request_body(socket, headers)
    if headers.fetch("transfer-encoding", "").downcase.include?("chunked")
      read_chunked_body(socket)
    else
      socket.read(headers.fetch("content-length", "0").to_i)
    end
  end

  def read_chunked_body(socket)
    body = "".b
    loop do
      size_line = socket.gets or raise EOFError, "missing chunk size"
      size = Integer(size_line.split(";", 2).first, 16)
      break if size.zero?

      body << socket.read(size)
      raise IOError, "missing chunk terminator" unless socket.read(2) == "\r\n"
    end

    while (line = socket.gets)
      break if line == "\r\n"
    end
    body
  end
end
