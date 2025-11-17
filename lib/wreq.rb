# frozen_string_literal: true

require_relative "wreq_ruby/wreq_ruby"

# Wreq is a high-performance HTTP client library for Ruby, powered by Rust.
#
# @example Simple GET request
#   response = Wreq.get("https://api.example.com")
module Wreq
  # Returns the version of the Wreq gem
  VERSION = "0.1.0"

  # HTTP method enumeration (defined by native extension)
  # Class is intentionally empty here; constants are set from Rust.
  class Method; end

  # HTTP version enumeration (defined by native extension)
  # Class is intentionally empty here; constants are set from Rust.
  class Version; end
end
