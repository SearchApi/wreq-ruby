# frozen_string_literal: true

module Wreq
  # HTTP method enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::Method}.
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
      GET = nil # @return [Wreq::Method] HTTP GET method
      HEAD = nil # @return [Wreq::Method] HTTP HEAD method
      POST = nil # @return [Wreq::Method] HTTP POST method
      PUT = nil # @return [Wreq::Method] HTTP PUT method
      DELETE = nil # @return [Wreq::Method] HTTP DELETE method
      OPTIONS = nil # @return [Wreq::Method] HTTP OPTIONS method
      TRACE = nil # @return [Wreq::Method] HTTP TRACE method
      PATCH = nil # @return [Wreq::Method] HTTP PATCH method
    end
  end

  # HTTP version enumeration backed by Rust.
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
      HTTP_2 = nil # @return [Wreq::Version] HTTP/2
      HTTP_3 = nil # @return [Wreq::Version] HTTP/3
    end
  end

  # HTTP status code wrapper.
  #
  # This class wraps standard HTTP status codes and provides
  # convenient methods to check the response category.
  #
  # The actual implementation is provided by Rust for performance.
  #
  # @example Check if response is successful
  #   status = response.status
  #   if status.success?
  #     puts "Request succeeded with code: #{status.as_int}"
  #   end
  #
  # @example Check different status categories
  #   status.informational?  # 1xx
  #   status.success?        # 2xx
  #   status.redirection?    # 3xx
  #   status.client_error?   # 4xx
  #   status.server_error?   # 5xx
 
end
