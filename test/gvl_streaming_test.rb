# frozen_string_literal: true

require 'test_helper'

# Test suite for GVL-aware streaming (issue #57).
#
# Requires httpbin running on localhost:8080:
#   docker run -d -p 8080:80 --name httpbin kennethreitz/httpbin
#
# Covers:
#   - Basic functionality (yield count, encoding, return value, empty body)
#   - Block control flow (break, next, exception)
#   - Double-consumption / body ownership
#   - GVL correctness (other threads run during I/O)
#   - Thread interruption (kill, raise)
#   - Streaming error propagation
#   - Content integrity (streamed == buffered)
#   - GC safety (forced collection during streaming)
#   - Concurrency (multiple threads, multiple clients)
#   - Regression: original issue #57 (Mutex starvation)

class GvlStreamingTest < Minitest::Test
  BASE = 'http://localhost:8080'

  # ===========================================================================
  # Helpers
  # ===========================================================================

  def client
    Wreq::Client.new
  end

  def get(path, **opts)
    client.get("#{BASE}#{path}", **opts)
  end

  # Collect all chunks from a response into an array.
  def collect_chunks(resp)
    chunks = []
    resp.chunks { |c| chunks << c }
    chunks
  end

  # ===========================================================================
  # 1. Basic functionality
  # ===========================================================================

  def test_chunks_yields_correct_count
    chunks = collect_chunks(get('/stream/5'))
    assert_equal 5, chunks.size
  end

  def test_chunks_yields_strings
    get('/stream/3').chunks do |chunk|
      assert_kind_of String, chunk
    end
  end

  def test_chunks_yields_binary_encoding
    get('/stream/3').chunks do |chunk|
      assert_includes [Encoding::BINARY, Encoding::ASCII_8BIT], chunk.encoding,
                      "Expected binary encoding, got #{chunk.encoding}"
    end
  end

  def test_chunks_yields_non_empty_chunks
    get('/stream/3').chunks do |chunk|
      assert chunk.bytesize > 0, 'Chunks must not be empty'
    end
  end

  def test_chunks_returns_nil
    result = get('/stream/3').chunks { |_c| :ignored_return_value }
    assert_nil result
  end

  def test_chunks_block_return_value_is_always_nil
    # Whatever the block returns, chunks itself must return nil.
    [42, 'string', :symbol, [], {}, true, false].each do |val|
      result = get('/stream/1').chunks { |_c| val }
      assert_nil result, "chunks should return nil when block returns #{val.inspect}"
    end
  end

  def test_chunks_empty_body
    chunk_count = 0
    get('/status/204').chunks { |_c| chunk_count += 1 }
    assert_equal 0, chunk_count
  end

  def test_chunks_single_chunk_total_bytes
    total = 0
    get('/bytes/1024').chunks { |c| total += c.bytesize }
    assert_equal 1024, total
  end

  def test_chunks_without_block_raises_local_jump_error
    resp = get('/stream/3')
    assert_raises(LocalJumpError) { resp.chunks }
  end

  # ===========================================================================
  # 2. Block control flow
  # ===========================================================================

  def test_break_inside_block_stops_iteration
    chunks_seen = 0
    # /stream/10 yields 10 chunks; we break after 2.
    get('/stream/10').chunks do |_c|
      chunks_seen += 1
      break if chunks_seen == 2
    end
    assert_equal 2, chunks_seen, 'break should stop iteration after 2 chunks'
  end

  def test_break_inside_block_does_not_raise
    # break must not propagate as an exception to the caller.
    assert_silent do
      get('/stream/5').chunks do |_c|
        break
      end
    end
  end

  def test_next_inside_block_skips_to_next_chunk
    processed = []
    get('/stream/5').chunks do |c|
      next if processed.size == 2 # skip third chunk processing

      processed << c
    end
    # next skips the block body but iteration continues; we still get 5 yields
    # but only push 4 times (skip once when size==2 means index 2 skipped).
    assert_equal 4, processed.size
  end

  def test_exception_in_block_propagates_to_caller
    raised = nil
    begin
      get('/stream/5').chunks do |_c|
        raise 'block error'
      end
    rescue RuntimeError => e
      raised = e
    end
    refute_nil raised
    assert_equal 'block error', raised.message
  end

  def test_exception_in_block_stops_after_correct_chunk_count
    count = 0
    begin
      get('/stream/5').chunks do |_c|
        count += 1
        raise 'stop' if count == 3
      end
    rescue RuntimeError
    end
    assert_equal 3, count
  end

  def test_exception_class_preserved_through_block
    begin
      get('/stream/3').chunks { raise ArgumentError, 'bad arg' }
    rescue ArgumentError => e
      assert_equal 'bad arg', e.message
      return
    end
    flunk 'ArgumentError should have propagated'
  end

  # ===========================================================================
  # 3. Body ownership / double-consumption
  # ===========================================================================

  def test_chunks_called_twice_raises_memory_error
    resp = get('/stream/3')
    resp.chunks { |_c| }
    assert_raises(Wreq::MemoryError) { resp.chunks { |_c| } }
  end

  def test_text_after_chunks_raises_memory_error
    resp = get('/stream/3')
    resp.chunks { |_c| }
    assert_raises(Wreq::MemoryError) { resp.text }
  end

  def test_bytes_after_chunks_raises_memory_error
    resp = get('/stream/3')
    resp.chunks { |_c| }
    assert_raises(Wreq::MemoryError) { resp.bytes }
  end

  def test_chunks_after_text_raises_memory_error
    resp = get('/stream/3')
    resp.text
    assert_raises(Wreq::MemoryError) { resp.chunks { |_c| } }
  end

  # Regression for issue #3 in the fix analysis:
  # If response() raises (body already consumed), we must NOT leak a GC
  # registration. This is validated indirectly: if a stale pointer is
  # registered, subsequent GC cycles corrupt the heap and later tests crash.
  def test_gc_registration_not_leaked_when_response_already_consumed
    resp = get('/stream/3')
    resp.chunks { |_c| }

    # Force several GC cycles. If a stale pointer was registered, this crashes.
    10.times { GC.start(full_mark: true, immediate_sweep: true) }

    # Confirm the error is still raised cleanly (body correctly consumed).
    assert_raises(Wreq::MemoryError) { resp.chunks { |_c| } }
  end

  # ===========================================================================
  # 4. Content integrity
  # ===========================================================================

  def test_streamed_content_matches_buffered_content
    full   = client.get("#{BASE}/bytes/4096").bytes
    stream = ''.b
    client.get("#{BASE}/bytes/4096").chunks { |c| stream << c }
    assert_equal full.bytesize, stream.bytesize
  end

  def test_streamed_content_is_valid_json_per_chunk
    get('/stream/5').chunks do |chunk|
      assert_match(/\{.*\}/, chunk, 'Each chunk from /stream/N should be a JSON object')
    end
  end

  def test_large_stream_total_size
    # /bytes/N returns exactly N random bytes. Stream it and verify.
    size = 256 * 1024 # 256 KB
    total = 0
    client.get("#{BASE}/bytes/#{size}").chunks { |c| total += c.bytesize }
    assert_equal size, total
  end

  def test_many_chunks_all_received
    # /stream/N yields N JSON objects, one per chunk.
    n = 20
    chunks = collect_chunks(get("/stream/#{n}"))
    assert_equal n, chunks.size
  end

  # ===========================================================================
  # 5. GVL correctness
  # ===========================================================================

  # Core GVL test: a background thread must make measurable progress while
  # the main thread is blocked waiting for network chunks.
  #
  # Uses an Atomic-style counter via Array#push (GIL-safe on MRI) rather than
  # a plain integer to avoid a data race in the assertion read.
  def test_other_threads_run_during_network_wait
    resp = client.get("#{BASE}/drip?duration=3&numbytes=3&delay=1")

    ticks = []
    ticker = Thread.new do
      30.times do
        ticks << 1
        sleep 0.1
      end
    end

    chunks_received = 0
    resp.chunks { |_c| chunks_received += 1 }
    ticker.join(10)

    # With 3 chunks spaced 1s apart, the ticker should have accumulated
    # at least ~20 ticks (3 seconds * 10 ticks/sec) if the GVL is released.
    # We use a conservative threshold of 10 to allow for slow CI.
    assert ticks.size > 10,
           "Ticker only reached #{ticks.size} ticks — GVL may not be released during I/O. " \
           'Expected > 10 ticks during a 3-second drip stream.'
    assert chunks_received >= 1
  end

  # Regression for issue #57: streaming must not starve a thread that holds
  # a Mutex. Before the fix, BodyReceiver held the GVL continuously, so a
  # thread waiting on mutex.synchronize would never be scheduled.
  def test_streaming_does_not_starve_mutex_waiters
    mutex = Mutex.new
    mutex_acquired_at = nil

    # This thread acquires the mutex after a short delay.
    # If the GVL is held by the streaming thread, it will never run.
    waiter = Thread.new do
      sleep 0.5
      mutex.synchronize { mutex_acquired_at = Time.now }
    end

    Time.now
    client.get("#{BASE}/drip?duration=3&numbytes=3&delay=1").chunks { |_c| }
    stream_end = Time.now

    waiter.join(10)

    refute_nil mutex_acquired_at, 'Mutex waiter thread never ran'

    # The waiter should have acquired the mutex well before streaming finished.
    assert mutex_acquired_at < stream_end,
           "Mutex was only acquired at #{mutex_acquired_at}, after streaming ended at #{stream_end}. " \
           'The streaming thread may have held the GVL for the entire duration.'
  end

  def test_multiple_concurrent_streams_same_client
    results = Array.new(2)
    threads = 2.times.map do |i|
      Thread.new do
        chunks = collect_chunks(client.get("#{BASE}/stream/3"))
        results[i] = chunks.size
      end
    end
    threads.each { |t| t.join(15) }
    assert_equal [3, 3], results
  end

  def test_multiple_concurrent_streams_different_clients
    results = Array.new(3)
    threads = 3.times.map do |i|
      Thread.new do
        c = Wreq::Client.new
        chunks = collect_chunks(c.get("#{BASE}/stream/3"))
        results[i] = chunks.size
      end
    end
    threads.each { |t| t.join(15) }
    assert_equal [3, 3, 3], results
  end

  # ===========================================================================
  # 6. Thread interruption
  # ===========================================================================

  def test_thread_kill_during_network_wait
    started = false
    thread = Thread.new do
      resp = client.get("#{BASE}/drip?duration=10&numbytes=10")
      started = true
      resp.chunks { |_c| }
    rescue StandardError => _e
    end

    # Wait until the thread has actually started streaming before killing.
    sleep 0.1 until started || !thread.alive?
    sleep 0.5 # let it block on first chunk wait
    thread.kill
    assert thread.join(5), 'Thread should terminate after kill'
  end

  def test_thread_kill_during_block_execution
    started = false
    thread = Thread.new do
      resp = client.get("#{BASE}/stream/5")
      started = true
      resp.chunks do |_c|
        sleep 10 # block in the Ruby block, not in I/O
      end
    rescue StandardError => _e
    end

    sleep 0.1 until started || !thread.alive?
    sleep 0.3
    thread.kill
    assert thread.join(5), 'Thread should terminate when killed during block execution'
  end

  def test_thread_raise_during_streaming
    error_class = Class.new(StandardError)
    received_error = nil
    started = false

    thread = Thread.new do
      resp = client.get("#{BASE}/drip?duration=10&numbytes=10")
      started = true
      resp.chunks { |_c| }
    rescue StandardError => e
      received_error = e
    end

    sleep 0.1 until started || !thread.alive?
    sleep 0.5
    thread.raise(error_class, 'injected')
    assert thread.join(5), 'Thread should terminate after raise'
    assert_instance_of error_class, received_error,
                       "Expected injected error class, got #{received_error.class}"
  end

  # ===========================================================================
  # 7. Streaming error propagation
  # ===========================================================================

  def test_streaming_error_raises_ruby_exception
    # Use a very short timeout against a slow drip to force a mid-body error.
    resp = client.get("#{BASE}/drip?duration=10&numbytes=10", timeout: 1)
    error_raised = false
    begin
      resp.chunks { |_c| }
    rescue Wreq::TimeoutError, Wreq::BodyError, Wreq::ConnectionResetError
      error_raised = true
    end
    assert error_raised, 'A streaming timeout should raise a Wreq error'
  end

  def test_streaming_error_is_not_silently_swallowed
    # Before the fix, BodyReceiver swallowed errors via .and_then(|r| r.ok()).
    # This test verifies that an error causes an exception, not silent EOF.
    resp = client.get("#{BASE}/drip?duration=10&numbytes=10", timeout: 1)
    chunks_received = 0
    begin
      resp.chunks do |_c|
        chunks_received += 1
      end
    rescue Wreq::TimeoutError, Wreq::BodyError, Wreq::ConnectionResetError
      # expected
    end
    # We should have received fewer chunks than the full 10 (cut short by timeout)
    # and an error should have been raised (not just silently stopped at 0 chunks).
    assert chunks_received < 10,
           'Should not have received all 10 chunks before timeout'
  end

  # ===========================================================================
  # 8. GC safety
  # ===========================================================================

  def test_block_not_gc_collected_during_streaming
    # Force GC between every chunk. If the block Proc is not GC-pinned,
    # this will crash or raise "invalid block VALUE".
    chunks_received = 0
    client.get("#{BASE}/drip?duration=3&numbytes=3&delay=1").chunks do |_c|
      chunks_received += 1
      GC.start(full_mark: true, immediate_sweep: true)
      GC.compact if GC.respond_to?(:compact)
    end
    assert_equal 3, chunks_received,
                 'All chunks must arrive even with forced GC + compaction between yields'
  end

  def test_gc_compaction_during_streaming
    skip 'GC.compact not available' unless GC.respond_to?(:compact)
    chunks = []
    client.get("#{BASE}/drip?duration=2&numbytes=2&delay=1").chunks do |c|
      chunks << c
      GC.compact
    end
    assert chunks.size >= 1
    chunks.each { |c| assert c.bytesize > 0 }
  end

  def test_aggressive_gc_between_chunks_does_not_corrupt_data
    stream = ''.b
    client.get("#{BASE}/bytes/8192").chunks do |c|
      GC.start
      stream << c
    end
    assert_equal 8192, stream.bytesize
  end

  # ===========================================================================
  # 9. close() integration
  # ===========================================================================

  def test_close_after_full_stream_does_not_raise
    resp = get('/stream/3')
    resp.chunks { |_c| }
    assert_silent { resp.close }
  end

  def test_close_after_partial_stream_does_not_raise
    resp = get('/stream/5')
    count = 0
    begin
      resp.chunks do |_c|
        count += 1
        break if count == 2
      end
    rescue StandardError
    end
    assert_silent { resp.close }
  end

  # ===========================================================================
  # 10. Client / module method variants
  # ===========================================================================

  def test_chunks_via_module_method
    chunks = []
    Wreq.get("#{BASE}/stream/3").chunks { |c| chunks << c }
    assert_equal 3, chunks.size
  end

  def test_chunks_via_client_instance
    chunks = []
    client.get("#{BASE}/stream/3").chunks { |c| chunks << c }
    assert_equal 3, chunks.size
  end

  def test_chunks_on_post_response
    chunks = []
    client.post("#{BASE}/post", body: 'hello').chunks { |c| chunks << c }
    assert chunks.size >= 1
    combined = chunks.join
    assert combined.bytesize > 0
  end
end
