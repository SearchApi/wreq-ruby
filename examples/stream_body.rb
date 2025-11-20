#!/usr/bin/env ruby

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

# Make a request
client = Wreq::Client.new()
response = client.get("https://httpbin.org/stream/20")
streamer = response.stream()

puts "\n=== Streaming Response Body ==="
streamer.each do |chunk|
  puts chunk
  sleep 0.1 # Simulate processing time
end
