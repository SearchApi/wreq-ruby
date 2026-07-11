# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # Streams a request body through a bounded, thread-safe channel.
    #
    # The channel applies backpressure instead of buffering the entire request body.
    # Multiple threads may safely push chunks while a request drains the receiving side.
    #
    # Usage:
    #   sender = Wreq::BodySender.new(8)
    #   Thread.new do
    #     File.open('big.bin', 'rb') { |f| while (chunk = f.read(65536)); sender.push(chunk); end }
    #     sender.close
    #   end
    #   resp = client.post(url, body: sender)
    #
    # A sender can be attached to one request. Closing it prevents further writes but
    # retains queued chunks so a request attached afterward can still drain them.
    class BodySender
      # Create a bounded request-body sender.
      #
      # @param capacity [Integer] positive number of chunks that may wait in the channel;
      #   defaults to 8 and must be greater than zero
      # @raise [ArgumentError] if capacity is zero, negative, or too large
      # @raise [TypeError] if capacity is not an Integer
      def self.new(capacity = 8)
      end

      # Push one binary chunk, waiting while the channel is full.
      #
      # @param data [String] binary chunk
      # @return [nil]
      # @raise [IOError] if the sender or receiving side is closed
      def push(data)
      end

      # Close the producer and signal EOF after all queued chunks are read.
      #
      # This operation is idempotent.
      #
      # @return [nil]
      def close
      end

      # Return whether the sender can no longer accept chunks.
      #
      # This becomes true after {#close} or when the request stops consuming
      # the receiving side.
      #
      # @return [Boolean]
      def closed?
      end
    end
  end
end
