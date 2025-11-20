# frozen_string_literal: true

module Wreq
  # Iterator for HTTP headers.
  #
  # This class provides efficient iteration over HTTP response headers,
  # yielding name-value pairs. The implementation is backed by Rust for
  # performance.
  #
  # @example Iterate headers
  #   response.each_header do |name, value|
  #     puts "#{name}: #{value}"
  #   end
  #
  # @example Collect all headers
  #   headers = response.each_header.to_h
  unless const_defined?(:HeaderIterator)
    class HeaderIterator
      include Enumerable

      # Iterate over header name-value pairs.
      #
      # @yieldparam name [String] Header name (lowercase)
      # @yieldparam value [String] Header value
      # @return [Enumerator, self] Returns enumerator if no block given
      # @example With block
      #   iterator.each do |name, value|
      #     puts "#{name}: #{value}"
      #   end
      # @example As enumerator
      #   headers_hash = iterator.each.to_h
      def each; end
    end
  end
end
