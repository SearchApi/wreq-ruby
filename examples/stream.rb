#!/usr/bin/env ruby

require_relative "../lib/wreq"


# Make a request
client = Wreq::Client.new
response = client.get("https://httpbin.io/stream/20")

# Create a cancellation token
token = Wreq::CancellationToken.new

count = 0

# Start a thread to process streaming chunks
stream_thread = Thread.new do
  begin
    response.chunks(token) do |chunk|
      count += 1
      puts chunk
      sleep 0.1 # Simulate processing time
    end
  rescue => e
    puts "[Streaming interrupted: #{e.class}: #{e.message}]"
  end
end

# Main thread: cancel after 1 second
sleep 1
token.cancel
puts "\n[Cancelled by main thread!]"

stream_thread.join
puts "\nChunks received before cancellation: #{count} (should be less than 20)"