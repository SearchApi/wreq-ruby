#!/usr/bin/env ruby

require_relative "../lib/wreq"

# Make a request
client = Wreq::Client.new
response = client.get("https://httpbin.io/headers")

puts "\n=== Iterating Over All Headers ==="
response.headers.each do |name, value|
  puts "#{name}: #{value}"
end

puts "\n=== Response Summary ==="
puts "Status: #{response.code}"
puts "Version: #{response.version}"
puts "URI: #{response.url}"
puts "Content Length: #{response.content_length || "Unknown"}"

if response.local_addr
  puts "Local Address: #{response.local_addr}"
end

if response.remote_addr
  puts "Remote Address: #{response.remote_addr}"
end
