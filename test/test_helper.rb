# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

HTTPBIN_URL = ENV.fetch("HTTPBIN_URL", "https://httpbin.io").delete_suffix("/")
MINITEST_RETRY_COUNT = Integer(ENV.fetch("MINITEST_RETRY_COUNT", "3"))
MINITEST_RETRY_DISPLAY_FAILURE_MESSAGES =
  ENV.fetch("MINITEST_RETRY_DISPLAY_FAILURE_MESSAGES", "true") != "false"
MINITEST_RETRY_SLEEP_INTERVAL = Float(ENV.fetch("MINITEST_RETRY_SLEEP_INTERVAL", "1"))

require "minitest/autorun"
require "minitest/retry"

# httpbin.io is a shared external service, so the integration tests can see
# occasional network failures even when the client behavior is correct.
Minitest::Retry.use!(
  retry_count: MINITEST_RETRY_COUNT,
  verbose: false,
  io: $stderr
)

Minitest::Retry.on_retry do |klass, test_name, attempt, result|
  $stderr.puts "[MinitestRetry] retry #{klass}##{test_name} (#{attempt}/#{MINITEST_RETRY_COUNT})"

  if MINITEST_RETRY_DISPLAY_FAILURE_MESSAGES
    result.failures.each do |failure|
      message = failure.message.to_s.strip
      $stderr.puts message unless message.empty?
    end
  end

  sleep MINITEST_RETRY_SLEEP_INTERVAL if MINITEST_RETRY_SLEEP_INTERVAL.positive?
end

module HttpbinHelpers
  def httpbin_value(value)
    (value.is_a?(Array) && value.length == 1) ? value.first : value
  end

  def httpbin_fetch(hash, key)
    httpbin_value(hash.fetch(key))
  end

  def httpbin_cookies(json)
    json.fetch("cookies", json)
  end
end

class Minitest::Test
  include HttpbinHelpers
end
