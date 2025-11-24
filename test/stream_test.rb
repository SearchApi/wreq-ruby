require "test_helper"

class StreamTest < Minitest::Test
  def test_simple_push_stream
    client = Wreq::Client.new
    sender = Wreq::BodySender.new(4)
    producer = Thread.new do
      3.times { |i| sender.push("chunk-#{i}\n") }
      sender.close
    end
    resp = client.post("http://localhost:8080/post", body: sender, headers: { "Content-Type" => "text/plain" })
    assert_equal 200, resp.code
    echoed = resp.json["data"]
    assert_includes echoed, "chunk-0"
    assert_includes echoed, "chunk-1"
    assert_includes echoed, "chunk-2"
    producer.join
  end

  def test_response_body_chunks_stream
    client = Wreq::Client.new
    resp = client.get("http://localhost:8080/stream/5")
    chunks = []
    resp.chunks do |chunk|
      chunks << chunk
      assert_kind_of String, chunk
      assert_match(/\{.*\}/, chunk)
    end
    assert_equal 5, chunks.size
    chunks.each do |c|
      parsed = JSON.parse(c) rescue nil
      assert parsed.is_a?(Hash)
    end
  end

  def test_thread_interrupt_connect
    url = "http://10.255.255.1:12345/"
    killed = false
    thread = Thread.new do
      begin
        Wreq.get(url)
      rescue => _
      end
    end
    sleep 2
    thread.kill
    killed = thread.join(5)
    assert killed, "Connect phase should be interruptible"
  end

  def test_thread_interrupt_connect_with_timeout
    url = "http://10.255.255.1:12345/"
    killed = false
    thread = Thread.new do
      begin
        Wreq.get(url, timeout: 60)
      rescue => _
      end
    end
    sleep 2
    thread.kill
    killed = thread.join(5)
    assert killed, "Connect+timeout phase should be interruptible"
  end

  def test_thread_interrupt_body_reading
    url = "https://httpbin.io/drip?duration=5&numbytes=5"
    killed = false
    thread = Thread.new do
      begin
        resp = Wreq.get(url)
        resp.text
      rescue => _
      end
    end
    sleep 2
    thread.kill
    killed = thread.join(5)
    assert killed, "Body reading should be interruptible"
  end

  def test_thread_interrupt_body_streaming
    url = "https://httpbin.io/drip?duration=5&numbytes=5"
    killed = false
    thread = Thread.new do
      begin
        resp = Wreq.get(url)
        resp.chunks { |chunk| chunk }
      rescue => _
      end
    end
    sleep 2
    thread.kill
    killed = thread.join(5)
    assert killed, "Body streaming should be interruptible"
  end
end
