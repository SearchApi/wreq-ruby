#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../lib/wreq"

# Run every scenario, or select a few by name:
#   bundle exec ruby examples/error.rb tls timeout reset

$stdout.sync = true
$stderr.sync = true

ERROR_PREDICATES = %i[
  builder?
  body?
  tls?
  decoding?
  redirect?
  status?
  upgrade?
  connection_reset?
  timeout?
  proxy_connect?
  connect?
  request?
].freeze

TEST_SERVER = "https://example.testserver.host"

def report_error(error, expected:)
  # The standard message contains the complete native error chain.
  warn "#{error.class}: #{error}"

  warn "native facts:"
  ERROR_PREDICATES.each do |predicate|
    warn "  #{predicate}: #{error.public_send(predicate)}"
  end
  warn "HTTP status: #{error.status}" if error.status

  # error.uri can contain credentials or private query parameters. Redact it
  # before logging it. Use error.full_message(highlight: false) when a
  # backtrace and Ruby exception causes are also useful.

  expected = Array(expected)
  return if expected.any? { |error_class| error.is_a?(error_class) }

  warn "expected: #{expected.map(&:name).join(" or ")}"
end

def run_example(name, expected:, note: nil)
  puts "\n--- #{name} ---"
  puts note if note

  response = yield
  expected_names = Array(expected).map(&:name).join(" or ")
  puts "no error was raised: HTTP #{response.code} (expected #{expected_names})"
rescue Wreq::Error => error
  report_error(error, expected: expected)
ensure
  response&.close
end

def run_scenario(name, client)
  case name
  when "builder"
    run_example("Invalid URL", expected: Wreq::BuilderError) do
      client.get("not-a-valid-url")
    end
  when "status"
    run_example("HTTP 404", expected: Wreq::StatusError) do
      client.get("#{TEST_SERVER}/status/404").raise_for_status!
    end
  when "tls"
    run_example(
      "Rejected TLS certificate",
      expected: Wreq::ConnectError,
      note: "Certificate verification happens while connecting, so this is a ConnectError."
    ) do
      client.get("https://expired.badssl.com/", timeout: 10)
    end
  when "connect"
    run_example("Refused destination connection", expected: Wreq::ConnectError) do
      client.get("http://127.0.0.1:1/", timeout: 5)
    end
  when "proxy"
    run_example("Refused proxy connection", expected: Wreq::ProxyConnectError) do
      client.get(
        "https://example.com/",
        proxy: "http://127.0.0.1:1",
        timeout: 5
      )
    end
  when "timeout"
    run_example("Slow response", expected: Wreq::TimeoutError) do
      client.get("#{TEST_SERVER}/delay/5", timeout: 1)
    end
  when "reset"
    run_example(
      "Server resets the connection",
      expected: [Wreq::RequestError, Wreq::ConnectionResetError],
      note: "The HTTP layer may report an incomplete response before exposing the TCP reset."
    ) do
      client.get("#{TEST_SERVER}/error/reset", timeout: 5)
    end
  end
end

scenarios = %w[
  builder
  status
  tls
  connect
  proxy
  timeout
  reset
].freeze
selected = ARGV.empty? ? scenarios : ARGV
unknown = selected - scenarios

unless unknown.empty?
  warn "unknown scenario: #{unknown.join(", ")}"
  warn "available scenarios: #{scenarios.join(", ")}"
  exit 1
end

client = Wreq::Client.new(no_proxy: true)
selected.each { |name| run_scenario(name, client) }
