# frozen_string_literal: true

unless defined?(:Wreq)
  module Wreq
    # BodySender: for streaming request bodies, allowing thread-safe chunked data push.
    # Backed by a Rust channel, avoids buffering the entire payload in memory.
    #
    # Usage:
    #   sender = Wreq::BodySender.new(8)
    #   Thread.new do
    #     File.open("big.bin", "rb") { |f| while (chunk = f.read(65536)); sender.push(chunk); end }
    #   end
    #   resp = client.post(url, body: sender)
    #
    # Note: Sender is for request upload only, not for response reading.
    class BodySender
      # @param capacity [Integer] channel buffer size, default 8
      def self.new(capacity = 8); end

      # @param data [String] binary chunk
      def push(data); end
    end

    # BodySender: for streaming response bodies, supports enumerator/iterator chunk reading.
    # Backed by a Rust channel, automatically receives remote data in chunks.
    #
    # Usage:
    #   resp = client.get(url)
    #   receiver = resp.stream
    #   receiver.each { |chunk| ... }
    #
    # Note: BodySender is for response download only, not for request upload.
    class BodyReceiver
      include Enumerable

      # Enumerate all chunks
      def each; end
    end
  end
end
