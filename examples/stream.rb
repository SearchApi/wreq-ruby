#!/usr/bin/env ruby

require_relative "../lib/wreq"

# Make a request
client = Wreq::Client.new()
response = client.get("https://httpbin.io/stream/20")

# Get the streaming body receiver
puts "\n=== Streaming Response Body ==="
response.chunks do |chunk|
  puts chunk
  sleep 0.1 # Simulate processing time
end
