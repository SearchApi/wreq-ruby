# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

HTTPBIN_URL = ENV.fetch("HTTPBIN_URL", "https://httpbin.io").delete_suffix("/")

require "minitest/autorun"
