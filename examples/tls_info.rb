#!/usr/bin/env ruby
# frozen_string_literal: true

require "openssl"
require_relative "../lib/wreq"

url = ARGV.fetch(0, "https://example.com")
client = Wreq::Client.new(tls_info: true)
response = client.get(url)
tls_info = response.tls_info
response.close

abort "TLS information is unavailable for #{url}" unless tls_info

p tls_info

if (der = tls_info.peer_certificate)
  certificate = OpenSSL::X509::Certificate.new(der)
  puts "Subject: #{certificate.subject}"
  puts "Issuer: #{certificate.issuer}"
  puts "Valid from: #{certificate.not_before}"
  puts "Valid until: #{certificate.not_after}"
end

chain = tls_info.peer_certificate_chain
chain_size = chain ? chain.length : "unavailable"
puts "Certificate chain: #{chain_size}"
