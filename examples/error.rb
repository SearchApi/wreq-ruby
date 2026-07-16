#!/usr/bin/env ruby

require_relative "../lib/wreq"

begin
  Wreq.get("not-a-valid-url")
rescue Wreq::Error => error
  puts "URI: #{error.uri.inspect}"
  puts "Status: #{error.status.inspect}"
  puts "#{error.class}: #{error.message}"
  puts "Native builder error: #{error.is_builder}"
end
