#!/usr/bin/env ruby

require_relative "../lib/wreq"

begin
  Wreq.get("not-a-valid-url")
rescue Wreq::Error => error
  puts "#{error.class}: #{error.message}"
  puts "builder: #{error.builder?}"
  puts "uri: #{error.uri.inspect}"
  puts "status: #{error.status.inspect}"
end
