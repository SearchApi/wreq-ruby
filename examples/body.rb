#!/usr/bin/env ruby

# Example: Using Wreq::Response headers methods
# Demonstrates the different ways to access HTTP response headers

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

# Make a request
client = Wreq::Client.new
response = client.get("https://httpbin.org/anything",
                      json: { foo: "bar", baz: "qux" },
                      headers: { "Content-Type" => "application/json" },
                      cookies: { "session_id" => "abc123",
                                 "user" => "test_user" })

puts "\n=== Iterating Over All Headers ==="
response.each_header do |name, value|
  puts "#{name}: #{value}"
end

puts "\n=== Response Summary ==="
puts "Status: #{response.code}"
puts "Version: #{response.version}"
puts "URI: #{response.uri}"
puts "Content Length: #{response.content_length || "Unknown"}"

if response.local_addr
  puts "Local Address: #{response.local_addr}"
end

if response.remote_addr
  puts "Remote Address: #{response.remote_addr}"
end

puts "\n=== Response Body as Text ==="
puts response.text

puts "\n=== Response Body as JSON ==="
json = response.json
puts json
puts json["json"]
puts json["headers"]["Content-Type"]
