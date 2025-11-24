# test/request_stream_test.rb
require "test_helper"
require "wreq"

class RequestStreamTest < Minitest::Test
  ENDPOINT = ENV["WREQ_ECHO_URL"] || "https://httpbin.io/post"

  def test_simple_push_stream
    client = Wreq::Client.new
    sender = Wreq::BodySender.new(4)
    producer = Thread.new do
      3.times { |i| sender.push("chunk-#{i}\n") }
      sender.close
    end
    resp = client.post(ENDPOINT, body: sender, headers: { "Content-Type" => "text/plain" })
    assert_equal 200, resp.code
    echoed = resp.json["data"]
    assert_includes echoed, "chunk-0"
    assert_includes echoed, "chunk-1"
    assert_includes echoed, "chunk-2"
    producer.join
  end
end

class ResponseStreamTest < Minitest::Test
  ENDPOINT = ENV["WREQ_ECHO_URL"] || "https://httpbin.io/stream/5"

  def test_response_body_chunks_stream
    client = Wreq::Client.new
    resp = client.get(ENDPOINT)
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
end
