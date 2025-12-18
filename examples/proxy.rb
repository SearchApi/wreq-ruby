#!/usr/bin/env ruby

require_relative "../lib/wreq"

# Proxy templates:
# HTTP/HTTPS proxy: http://127.0.0.1:6152
# SOCKS5 proxy: socks5://127.0.0.1:6153

puts "=" * 60
puts "Wreq Ruby Proxy Examples"
puts "=" * 60

# ==============================================================================
# Example 1: Basic HTTP Proxy
# ==============================================================================
puts "\n--- Example 1: Basic HTTP Proxy ---"
begin
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    proxy: "http://127.0.0.1:6152"
  )
  puts "✅ Client with HTTP proxy created"

  response = client.get("https://httpbin.io/ip")
  puts "Status: #{response.code}"
  puts "Response: #{response.text}"
rescue Wreq::RequestError => e
  puts "❌ Request failed: #{e.message}"
rescue => e
  puts "❌ Error: #{e.class} - #{e.message}"
end

# ==============================================================================
# Example 2: SOCKS5 Proxy
# ==============================================================================
puts "\n--- Example 2: SOCKS5 Proxy ---"
begin
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    proxy: "socks5://127.0.0.1:6153"
  )
  puts "✅ Client with SOCKS5 proxy created"

  response = client.get("https://httpbin.io/ip")
  puts "Status: #{response.code}"
  puts "Response: #{response.text}"
rescue Wreq::RequestError => e
  puts "❌ Request failed: #{e.message}"
rescue => e
  puts "❌ Error: #{e.class} - #{e.message}"
end

# ==============================================================================
# Example 3: SOCKS5h Proxy (DNS resolution through proxy)
# ==============================================================================
puts "\n--- Example 3: SOCKS5h Proxy (Remote DNS) ---"
begin
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    proxy: "socks5h://127.0.0.1:6153"
  )
  puts "✅ Client with SOCKS5h proxy created (DNS resolved by proxy)"

  response = client.get("https://httpbin.io/ip")
  puts "Status: #{response.code}"
  puts "Response: #{response.text}"
rescue Wreq::RequestError => e
  puts "❌ Request failed: #{e.message}"
rescue => e
  puts "❌ Error: #{e.class} - #{e.message}"
end

# ==============================================================================
# Example 4: HTTP Proxy with Authentication
# ==============================================================================
puts "\n--- Example 4: HTTP Proxy with Authentication ---"
begin
  # Format: http://username:password@host:port
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    proxy: "http://user:pass@127.0.0.1:6152"
  )
  puts "✅ Client with authenticated HTTP proxy created"

  response = client.get("https://httpbin.io/ip")
  puts "Status: #{response.code}"
  puts "Response: #{response.text}"
rescue Wreq::RequestError => e
  puts "❌ Request failed: #{e.message}"
rescue => e
  puts "❌ Error: #{e.class} - #{e.message}"
end

# ==============================================================================
# Example 5: SOCKS5 Proxy with Authentication
# ==============================================================================
puts "\n--- Example 5: SOCKS5 Proxy with Authentication ---"
begin
  # Format: socks5://username:password@host:port
  client = Wreq::Client.new(
    user_agent: "WreqClient/1.0",
    proxy: "socks5://user:pass@127.0.0.1:6153"
  )
  puts "✅ Client with authenticated SOCKS5 proxy created"

  response = client.get("https://httpbin.io/ip")
  puts "Status: #{response.code}"
  puts "Response: #{response.text}"
rescue Wreq::RequestError => e
  puts "❌ Request failed: #{e.message}"
rescue => e
  puts "❌ Error: #{e.class} - #{e.message}"
end
