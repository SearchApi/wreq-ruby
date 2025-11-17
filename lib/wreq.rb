# frozen_string_literal: true

require_relative "wreq_ruby/wreq_ruby"

# Wreq is a high-performance HTTP client library for Ruby, powered by Rust.
#
# @example Simple GET request
#   response = Wreq.get("https://api.example.com")
module Wreq
  # Returns the version of the Wreq gem
  VERSION = "0.1.0"

    # HTTP method enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::Method}.
  #
  # Available methods:
  # - GET     - Safe, idempotent
  # - HEAD    - Safe, idempotent
  # - POST    - Not safe, not idempotent
  # - PUT     - Not safe, idempotent
  # - DELETE  - Not safe, idempotent
  # - OPTIONS - Safe, idempotent
  # - TRACE   - Safe, idempotent
  # - PATCH   - Not safe, not idempotent
  #
  # @example Using predefined constants
  #   method = Wreq::Method::GET
  #   method.class #=> Wreq::Method
  #
  # @example In request context
  #   Wreq.request(url: "https://api.example.com", method: Wreq::Method::POST)
  class Method
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:GET)
      GET     = nil # @return [Wreq::Method] HTTP GET method
      HEAD    = nil # @return [Wreq::Method] HTTP HEAD method
      POST    = nil # @return [Wreq::Method] HTTP POST method
      PUT     = nil # @return [Wreq::Method] HTTP PUT method
      DELETE  = nil # @return [Wreq::Method] HTTP DELETE method
      OPTIONS = nil # @return [Wreq::Method] HTTP OPTIONS method
      TRACE   = nil # @return [Wreq::Method] HTTP TRACE method
      PATCH   = nil # @return [Wreq::Method] HTTP PATCH method
    end
  end

  # HTTP version enumeration backed by Rust.
  #
  # Available versions:
  # - HTTP_09 - HTTP/0.9 (legacy)
  # - HTTP_10 - HTTP/1.0
  # - HTTP_11 - HTTP/1.1 (most common)
  # - HTTP_2  - HTTP/2
  # - HTTP_3  - HTTP/3 (QUIC-based)
  #
  # @example Using predefined constants
  #   version = Wreq::Version::HTTP_11
  #   version.class #=> Wreq::Version
  class Version
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:HTTP_11)
      HTTP_09 = nil # @return [Wreq::Version] HTTP/0.9
      HTTP_10 = nil # @return [Wreq::Version] HTTP/1.0
      HTTP_11 = nil # @return [Wreq::Version] HTTP/1.1
      HTTP_2  = nil # @return [Wreq::Version] HTTP/2
      HTTP_3  = nil # @return [Wreq::Version] HTTP/3
    end
  end
end
