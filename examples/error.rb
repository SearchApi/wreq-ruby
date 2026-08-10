#!/usr/bin/env ruby

require_relative "../lib/wreq"

begin
  Wreq.get("not-a-valid-url")
rescue Wreq::Error => error
  warn error.full_message(highlight: false)
end
