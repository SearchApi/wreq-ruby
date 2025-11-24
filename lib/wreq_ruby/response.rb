# frozen_string_literal: true

unless defined?(:Wreq)
  module Wreq
    # HTTP response object containing status, headers, and body.
    #
    # This class wraps a native Rust implementation providing efficient
    # access to HTTP response data including status codes, headers, body
    # content, and streaming capabilities.
    #
    # @example Basic response handling
    #   response = client.get("https://api.example.com")
    #   puts response.status.as_int  # => 200
    #   puts response.text
    #
    # @example JSON response
    #   response = client.get("https://api.example.com/data")
    #   data = response.json
    #
    # @example Streaming response
    #   response = client.get("https://example.com/large-file")
    #   response.stream.each do |chunk|
    #     # Process chunk
    #   end
    class Response
      # Get the HTTP status code as an integer.
      #
      # @return [Integer] Status code (e.g., 200, 404, 500)
      # @example
      #   response.code  # => 200
      def code; end

      # Get the HTTP status code object.
      #
      # @return [Wreq::StatusCode] Status code wrapper with helper methods
      # @example
      #   status = response.status
      #   status.success?  # => true
      def status; end

      # Get the HTTP protocol version used.
      #
      # @return [Wreq::Version] HTTP version (HTTP/1.1, HTTP/2, etc.)
      # @example
      #   response.version  # => Wreq::Version::HTTP_11
      def version; end

      # Get the final URL after redirects.
      #
      # @return [String] The final URL
      # @example
      #   response.url  # => "https://example.com/final-page"
      def url; end

      # Get the content length if known.
      #
      # @return [Integer, nil] Content length in bytes, or nil if unknown
      # @example
      #   response.content_length  # => 1024
      def content_length; end

      # Get the local socket address.
      #
      # @return [String, nil] Local address (e.g., "127.0.0.1:54321"), or nil
      # @example
      #   response.local_addr  # => "192.168.1.100:54321"
      def local_addr; end

      # Get the remote socket address.
      #
      # @return [String, nil] Remote address (e.g., "93.184.216.34:443"), or nil
      # @example
      #   response.remote_addr  # => "93.184.216.34:443"
      def remote_addr; end

      # Get the response body as text.
      #
      # @return [String] Response body decoded as UTF-8 text
      # @example
      #   html = response.text
      #   puts html
      def text; end

      # Parse the response body as JSON.
      #
      # @return [Object] Parsed JSON (Hash, Array, String, Integer, Float, Boolean, nil)
      # @raise [Wreq::DecodingError] if body is not valid JSON
      # @example
      #   data = response.json
      #   puts data["key"]
      def json; end

      # Get a streaming iterator for the response body, yielding each chunk.
      #
      # This method allows you to process large HTTP responses efficiently,
      # by yielding each chunk of the body as it arrives, without loading
      # the entire response into memory.
      #
      # @return [Wreq::Receiver] An iterator over response body chunks (binary String)
      # @yield [chunk] Each chunk of the response body as a binary String
      # @example Save response to file
      #   File.open("output.bin", "wb") do |f|
      #     response.chunks { |chunk| f.write(chunk) }
      #   end
      # @example Count total bytes streamed
      #   total = 0
      #   response.chunks { |chunk| total += chunk.bytesize }
      #   puts "Downloaded #{total} bytes"
      #
      # Note: The returned Receiver is only for reading response bodies, not for uploads.
      def chunks; end

      # Close the response and free associated resources.
      #
      # @return [void]
      # @example
      #   response.close
      def close; end
    end
  end
end

# ======================== Ruby API Extensions ========================

module Wreq
  class Response
    # Returns a compact string representation of the response.
    #
    # Format: #<Wreq::Response STATUS content-type="..." body=SIZE>
    #
    # @return [String] Compact formatted response information
    # @example
    #   puts response.to_s
    #   # => #<Wreq::Response 200 content-type="application/json" body=456B>
    def to_s
      parts = ["#<Wreq::Response"]

      # Status code
      parts << code.to_s

      # Content-Type header if present
      if headers.respond_to?(:get)
        content_type = headers.get("content-type")
        parts << "content-type=#{content_type.inspect}" if content_type
      end

      # Body size
      if content_length
        parts << "body=#{format_bytes(content_length)}"
      end

      parts.join(" ") + ">"
    end

    private

    def format_bytes(bytes)
      return "0B" if bytes.zero?

      units = ["B", "KB", "MB", "GB"]
      size = bytes.to_f
      unit_index = 0

      while size >= 1024 && unit_index < units.length - 1
        size /= 1024.0
        unit_index += 1
      end

      if unit_index == 0
        "#{size.to_i}#{units[unit_index]}"
      else
        "#{size.round(1)}#{units[unit_index]}"
      end
    end
  end
end
