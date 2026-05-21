#!/usr/bin/env ruby

require_relative "../lib/wreq"

# Make a client
client = Wreq::Client.new

# Send a GET request with cookies provided as a Hash.
# This form is serialized as multiple Cookie header fields (common in HTTP/2).
resp = client.get(
  "https://tls.browserleaks.com",
  cookies: {"foo" => "bar", "baz" => "qux"}
)

puts resp.text

# Send a GET request with cookies provided as a single Cookie header string.
# This form is common in HTTP/1.1: one Cookie header with '; ' separated pairs.
resp = client.get(
  "https://tls.browserleaks.com",
  cookies: "foo=bar; baz=qux"
)

puts resp.text
