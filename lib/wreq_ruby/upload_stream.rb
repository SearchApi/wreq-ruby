# frozen_string_literal: true

unless defined?(:Wreq)
  module Wreq
    # UploadStream allows pushing bytes from Ruby into a request body stream.
    # Backed by a Rust channel, it supports true streaming without buffering
    # the entire payload in memory.
    #
    # Usage:
    #   us = Wreq::UploadStream.new(16) # capacity (optional)
    #   Thread.new do
    #     File.open("large.bin", "rb") do |f|
    #       while (chunk = f.read(64 * 1024))
    #         us.push(chunk)
    #       end
    #     end
    #     us.close
    #   end
    #   resp = client.post("https://httpbin.io/post", body: us)
    #
    # Methods are implemented in Rust via the native extension.
    class UploadStream
      # Create a new UploadStream with optional channel capacity.
      # @param capacity [Integer] optional buffer size (default: 8)
      # @return [Wreq::UploadStream]
      def self.new(capacity = 8); end

      # Push a chunk into the stream.
      # @param data [String] binary-safe string
      # @return [void]
      def push(data); end

      # Close the stream successfully. The receiver will see EOF.
      # @return [void]
      def close; end

      # Abort the stream with an error. The request will fail.
      # @param message [String, nil]
      # @return [void]
      def abort(message = nil); end
    end
  end
end
