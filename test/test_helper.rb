# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

HTTPBIN_URL = ENV.fetch("HTTPBIN_URL", "https://httpbin.io").delete_suffix("/")

require "minitest/autorun"

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
