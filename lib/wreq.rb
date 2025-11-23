# frozen_string_literal: true

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "wreq_ruby/#{$1}/wreq_ruby"
rescue LoadError
  require_relative "wreq_ruby/wreq_ruby"
end

# Load type hint definitions
require_relative "wreq_ruby/http"
require_relative "wreq_ruby/client"
require_relative "wreq_ruby/response"
require_relative "wreq_ruby/streamer"
require_relative "wreq_ruby/upload_stream"
require_relative "wreq_ruby/header"
require_relative "wreq_ruby/error"
require_relative "wreq_ruby/cookie"

unless defined?(:Wreq)
  module Wreq
    class << self
      # Send an HTTP request using the default client.
      #
      # @param method [Wreq::Method] HTTP method to use
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @option options [Hash{String=>String}] :headers Custom headers
      # @option options [Hash] :query URL query parameters
      # @option options [Hash{String=>String}] :form Form data
      # @option options [Object] :json JSON body (will be serialized)
      # @option options [String] :body Raw request body
      # @option options [String] :auth Authorization header value
      # @option options [String] :bearer_auth Bearer token
      # @option options [Array<String>] :basic_auth Username and password
      # @option options [Integer] :timeout Request timeout in seconds
      # @option options [Boolean] :allow_redirects Whether to follow redirects
      # @option options [Boolean] :gzip Enable gzip compression
      # @return [Wreq::Response] HTTP response
      def request(method, url, **options); end

      # Send an HTTP GET request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def get(url, **options); end

      # Send an HTTP POST request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def post(url, **options); end

      # Send an HTTP PUT request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def put(url, **options); end

      # Send an HTTP DELETE request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def delete(url, **options); end

      # Send an HTTP HEAD request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def head(url, **options); end

      # Send an HTTP OPTIONS request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def options(url, **options); end

      # Send an HTTP TRACE request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def trace(url, **options); end

      # Send an HTTP PATCH request using the default client.
      #
      # @param url [String] Target URL
      # @param options [Hash] Optional request parameters
      # @return [Wreq::Response] HTTP response
      def patch(url, **options); end
    end
  end
end
