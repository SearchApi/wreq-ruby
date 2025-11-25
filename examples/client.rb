#!/usr/bin/env ruby

require_relative "../lib/wreq"

puts "--- Example 1: Invalid Proxy URL ---"
begin
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    headers: {
      "User-Agent" => "WreqClient/1.0",
      "Accept" => "application/json",
      "App" => "WreqExample",
      "Cookie2" => "sessionid=abc123",
      "Cookie" => "preferences=darkmode",
    },
    timeout: 10,
    gzip: true,
    brotli: true,
    proxy: "http:://localhost:8080",
  )
  puts "Client created successfully: #{client.inspect}"
  s = client.get("https://httpbin.io/anything", headers: { "Custom-Header" => "CustomValue" }, basic_auth: ["user", "pass"])
  code = s.code()
  puts "Response code: #{code}"
rescue Wreq::BuilderError => e
  puts "❌ BuilderError caught!"
  puts "Error message: #{e.message}"
  puts "Error class: #{e.class}"
  puts "Backtrace (first 3 lines):"
  puts e.backtrace.first(3).map { |line| "  #{line}" }
rescue StandardError => e
  puts "❌ Unexpected error: #{e.class} - #{e.message}"
end
