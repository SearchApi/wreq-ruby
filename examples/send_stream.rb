#!/usr/bin/env ruby

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"
require "json"
require "tmpdir"

# Helpers
ENDPOINT = ENV["WREQ_ECHO_URL"] || "https://httpbin.io/post"
CHUNK = 64 * 1024

def example1_simple_push
  puts "\n=== Example 1: Simple push ==="
  client = Wreq::Client.new
  us = Wreq::BodySender.new(8)

  producer = Thread.new do
    5.times do |i|
      us.push("chunk-#{i}\n")
      sleep 0.05
    end
    us.close
  end

  resp = client.post(ENDPOINT, body: us, headers: { "Content-Type" => "text/plain" })
  body = resp.json
  data = body["data"] || body["json"].to_json
  puts "Echoed bytes: #{data.bytesize}"
  producer.join
end

def example2_file_stream
  puts "\n=== Example 2: File streaming ==="
  client = Wreq::Client.new
  us = Wreq::BodySender.new(16)

  # Prepare a temp file (~250KB)
  path = File.join(Dir.tmpdir, "wreq_upload_sample.bin")
  File.open(path, "wb") { |f| f.write(Random.new.bytes(2_500_00)) }

  total = 0
  producer = Thread.new do
    File.open(path, "rb") do |f|
      while (chunk = f.read(CHUNK))
        total += chunk.bytesize
        us.push(chunk)
      end
    end
    us.close
  end

  resp = client.post(ENDPOINT, body: us, headers: { "Content-Type" => "application/octet-stream" })
  json = resp.json
  echoed = (json["headers"] && json["headers"]["Content-Length"]) || (json["data"] || "").bytesize
  puts "Sent bytes ~#{total}, echoed length: #{echoed}"
  producer.join
ensure
  File.delete(path) if path && File.exist?(path)
end

def example3_background_producer
  puts "\n=== Example 3: Background thread producer ==="
  client = Wreq::Client.new
  us = Wreq::BodySender.new(4)

  # Simulate streaming generation
  producer = Thread.new do
    (1..20).each do |i|
      us.push("data-#{i},")
      sleep 0.01
    end
    us.close
  end

  resp = client.post(ENDPOINT, body: us)
  json = resp.json
  data = json["data"] || ""
  puts "Received length: #{data.bytesize}"
  producer.join
end

if __FILE__ == $0
  example1_simple_push
  example2_file_stream
  example3_background_producer
end