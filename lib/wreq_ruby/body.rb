# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # BodySender: for streaming request bodies, allowing thread-safe chunked data push.
    # Backed by a Rust channel, avoids buffering the entire payload in memory.
    #
    # Supports multi-threaded chunk push: you can safely call `push` from multiple threads.
    #
    # Usage:
    #   sender = Wreq::BodySender.new(8)
    #   Thread.new do
    #     File.open('big.bin', 'rb') { |f| while (chunk = f.read(65536)); sender.push(chunk); end }
    #     sender.close
    #   end
    #   resp = client.post(url, body: sender)
    #
    # Note:
    #   - Sender is for request upload only, not for response reading.
    #   - Each BodySender instance can only be used once (single-use):
    #     after being consumed by a request, further push or reuse is not allowed.
    class BodySender
      # @param capacity [Integer] channel buffer size, default 8
      def self.new(capacity = 8)
      end

      # @param data [String] binary chunk
      def push(data)
      end

      # Close the sender, signaling end of data.
      def close
      end
    end
  end
end
