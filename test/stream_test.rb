require "test_helper"

class StreamTest < Minitest::Test
  def test_simple_push_stream
    client = Wreq::Client.new
    sender = Wreq::BodySender.new(4)
    producer = Thread.new do
      3.times { |i| sender.push("chunk-#{i}\n") }
      sender.close
    end

    resp = client.post("#{HTTPBIN_URL}/post", body: sender, headers: {"Content-Type" => "text/plain"})

    assert_equal 200, resp.code

    echoed = resp.json["data"]
    assert_includes echoed, "chunk-0"
    assert_includes echoed, "chunk-1"
    assert_includes echoed, "chunk-2"

    producer.join
  end

  def test_response_body_chunks_stream
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/5")
    chunks = []

    resp.chunks do |chunk|
      chunks << chunk
      assert_kind_of String, chunk, "Each yielded chunk must be a String"
      assert_match(/\{.*\}/, chunk)
    end

    assert_equal 5, chunks.size, "Should yield exactly 5 chunks from /stream/5"
  end

  def test_chunks_yields_binary_encoding
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")

    resp.chunks do |chunk|
      assert chunk.encoding == Encoding::BINARY || chunk.encoding == Encoding::ASCII_8BIT,
        "Chunk should have binary encoding, got #{chunk.encoding}"
    end
  end

  def test_chunks_with_single_chunk_body
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/bytes/1024")
    chunk_count = 0
    total_bytes = 0

    resp.chunks do |chunk|
      chunk_count += 1
      total_bytes += chunk.bytesize
    end

    assert chunk_count >= 1, "Should yield at least one chunk"
    assert_equal 1024, total_bytes, "Total bytes should match content length"
  end

  def test_chunks_returns_nil
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")

    result = resp.chunks { |_chunk| :processing }

    assert_nil result, "chunks should return nil after completion"
  end

  def test_chunks_with_empty_body
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/status/204")
    chunk_count = 0

    resp.chunks do |_chunk|
      chunk_count += 1
    end

    assert_equal 0, chunk_count, "No chunks should be yielded for empty 204 response"
  end

  def test_chunks_without_block_raises_error
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")

    assert_raises(LocalJumpError) do
      resp.chunks
    end
  end

  def test_other_threads_run_during_streaming
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/drip?duration=3&numbytes=3&delay=1")

    counter = 0
    tick_thread = Thread.new do
      30.times do
        counter += 1
        sleep 0.1
      end
    end

    chunks_received = 0
    resp.chunks do |_chunk|
      chunks_received += 1
    end

    tick_thread.join(10)

    assert counter > 5,
      "Counter only reached #{counter} - other threads may not be running during streaming. " \
      "GVL should be released during I/O waits."
    assert chunks_received >= 1, "Should have received at least one chunk"
  end

  def test_multiple_concurrent_streams
    client = Wreq::Client.new
    results = {}
    done = {}

    t1 = Thread.new do
      resp = client.get("#{HTTPBIN_URL}/stream/3")
      chunks = []
      resp.chunks { |chunk| chunks << chunk }
      results[:t1] = chunks.size
      done[:t1] = true
    end

    t2 = Thread.new do
      resp = client.get("#{HTTPBIN_URL}/stream/3")
      chunks = []
      resp.chunks { |chunk| chunks << chunk }
      results[:t2] = chunks.size
      done[:t2] = true
    end

    t1.join(15)
    t2.join(15)

    assert done[:t1], "Thread 1 should complete"
    assert done[:t2], "Thread 2 should complete"
    assert_equal 3, results[:t1], "Thread 1 should receive 3 chunks"
    assert_equal 3, results[:t2], "Thread 2 should receive 3 chunks"
  end

  def test_thread_interrupt_connect
    url = "http://10.255.255.1:12345/"
    thread = Thread.new do
      Wreq.get(url)
    rescue => _
    end

    sleep 2
    thread.kill
    killed = thread.join(5)

    assert killed, "Connect phase should be interruptible"
  end

  def test_thread_interrupt_connect_with_timeout
    url = "http://10.255.255.1:12345/"
    thread = Thread.new do
      Wreq.get(url, timeout: 60)
    rescue => _
    end

    sleep 2
    thread.kill
    killed = thread.join(5)

    assert killed, "Connect+timeout phase should be interruptible"
  end

  def test_thread_interrupt_body_reading
    url = "#{HTTPBIN_URL}/drip?duration=5&numbytes=5"
    thread = Thread.new do
      resp = Wreq.get(url)
      resp.text
    rescue => _
    end

    sleep 2
    thread.kill
    killed = thread.join(5)

    assert killed, "Body reading should be interruptible"
  end

  def test_thread_interrupt_body_streaming
    url = "#{HTTPBIN_URL}/drip?duration=5&numbytes=5"
    thread = Thread.new do
      resp = Wreq.get(url)
      resp.chunks { |chunk| chunk }
    rescue => _
    end

    sleep 2
    thread.kill
    killed = thread.join(5)

    assert killed, "Body streaming should be interruptible"
  end

  def test_thread_interrupt_during_slow_stream_with_block_processing
    url = "#{HTTPBIN_URL}/drip?duration=5&numbytes=5&delay=1"
    thread = Thread.new do
      resp = Wreq.get(url)
      resp.chunks do |_chunk|
        sleep 0.5
      end
    rescue => _
    end

    sleep 2
    thread.kill
    killed = thread.join(5)

    assert killed, "Streaming with slow block processing should be interruptible"
  end

  def test_chunks_propagates_streaming_errors
    client = Wreq::Client.new
    error_raised = false

    begin
      resp = client.get("#{HTTPBIN_URL}/drip?duration=10&numbytes=10", timeout: 1)
      resp.chunks do |_chunk|
      end
    rescue => e
      error_raised = true
      assert(
        e.is_a?(Wreq::TimeoutError) || e.is_a?(Wreq::BodyError) || e.is_a?(Wreq::ConnectionResetError),
        "Expected a streaming error (TimeoutError/BodyError/ConnectionResetError), got #{e.class}: #{e.message}"
      )
    end

    assert error_raised, "A streaming error should have been raised for a timed-out drip response"
  end

  def test_exception_in_block_propagates
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/5")
    error_raised = false
    chunks_before_error = 0

    begin
      resp.chunks do |_chunk|
        chunks_before_error += 1
        raise "intentional error in block" if chunks_before_error == 2
      end
    rescue RuntimeError => e
      error_raised = true
      assert_equal "intentional error in block", e.message
    end

    assert error_raised, "Exception raised inside the block should propagate out"
    assert_equal 2, chunks_before_error, "Should have processed 2 chunks before the error"
  end

  def test_chunks_called_twice_raises_error
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")
    resp.chunks { |_chunk| }
    error_raised = false

    begin
      resp.chunks { |_chunk| }
    rescue => e
      error_raised = true
      assert_instance_of Wreq::MemoryError, e,
        "Second chunks call should raise MemoryError, got #{e.class}: #{e.message}"
    end

    assert error_raised, "Second chunks call should raise an error"
  end

  def test_text_after_chunks_raises_error
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")
    resp.chunks { |_chunk| }
    error_raised = false

    begin
      resp.text
    rescue => e
      error_raised = true
      assert_instance_of Wreq::MemoryError, e,
        "Calling text after chunks should raise MemoryError, got #{e.class}: #{e.message}"
    end

    assert error_raised, "Calling text after chunks should raise an error"
  end

  def test_chunks_content_matches_full_body
    client = Wreq::Client.new
    resp_full = client.get("#{HTTPBIN_URL}/bytes/4096")
    full_bytes = resp_full.bytes

    resp_stream = client.get("#{HTTPBIN_URL}/bytes/4096")
    streamed_bytes = "".b
    resp_stream.chunks do |chunk|
      streamed_bytes << chunk
    end

    assert_equal full_bytes.bytesize, streamed_bytes.bytesize,
      "Streamed body size should match full body size"
  end

  def test_chunks_json_stream_content
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/5")
    chunks = []

    resp.chunks do |chunk|
      chunks << chunk
    end

    chunks.each_with_index do |chunk, i|
      assert_match(/\{.*\}/, chunk,
        "Chunk #{i} should contain a JSON object, got: #{chunk[0..80]}")
    end
  end

  def test_block_not_garbage_collected_during_streaming
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")
    chunks_received = 0

    resp.chunks do |_chunk|
      chunks_received += 1
      GC.start
      GC.start
    end

    assert_equal 3, chunks_received,
      "All 3 chunks should be received even with forced GC between yields"
  end

  def test_close_after_streaming
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")

    resp.chunks { |_chunk| }
    resp.close
  end

  def test_chunks_via_module_method
    resp = Wreq.get("#{HTTPBIN_URL}/stream/3")
    chunks = []

    resp.chunks do |chunk|
      chunks << chunk
    end

    assert_equal 3, chunks.size, "Module-level Wreq.get + chunks should work"
  end

  def test_chunks_via_client_instance
    client = Wreq::Client.new
    resp = client.get("#{HTTPBIN_URL}/stream/3")
    chunks = []

    resp.chunks do |chunk|
      chunks << chunk
    end

    assert_equal 3, chunks.size, "Client instance get + chunks should work"
  end
end
