# frozen_string_literal: true

unless defined?(:Wreq)
  module Wreq
    # Streaming iterator for HTTP response bodies.
    #
    # This class provides efficient streaming of large HTTP responses
    # by yielding data in chunks, avoiding the need to load the entire
    # response body into memory at once.
    #
    # The actual implementation is provided by Rust for performance.
    #
    # @example Stream a large file
    #   response = client.get("https://example.com/large-file.zip")
    #   File.open("output.zip", "wb") do |f|
    #     response.stream.each do |chunk|
    #       f.write(chunk)
    #     end
    #   end
    #
    # @example Count bytes streamed
    #   total = 0
    #   response.stream.each do |chunk|
    #     total += chunk.bytesize
    #   end
    #   puts "Downloaded #{total} bytes"
    class Streamer
      include Enumerable

      # Iterate over response chunks.
      #
      # @yieldparam chunk [String] Binary data chunk
      # @return [Enumerator, self] Returns enumerator if no block given
      # @example With block
      #   streamer.each do |chunk|
      #     puts "Received #{chunk.bytesize} bytes"
      #   end
      # @example As enumerator
      #   chunks = streamer.each.to_a
      def each; end
    end
  end
end
