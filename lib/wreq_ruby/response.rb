# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # An HTTP response returned by wreq-ruby.
    #
    # Response metadata can be read repeatedly. Body helpers either buffer the
    # body for reuse or stream it once. A response belongs to the process that
    # received it. Accessing inherited metadata or body state raises
    # Wreq::ForkError. Independent values copied before fork, such as its status
    # or headers, remain usable in the child.
    #
    # @note Fork safety Keep each response in the process that received it.
    #   Issue a new request in the worker instead of carrying a response through
    #   `fork`.
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
    #   File.open("download.bin", "wb") do |file|
    #     response.chunks { |chunk| file.write(chunk) }
    #   end
    class Response
      # Get the HTTP status code as an integer.
      #
      # @return [Integer] Status code (e.g., 200, 404, 500)
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.code  # => 200
      def code
      end

      # Get the HTTP status code object.
      #
      # @return [Wreq::StatusCode] Status code wrapper with helper methods
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   status = response.status
      #   status.success?  # => true
      def status
      end

      # Return this response or raise for a 4xx or 5xx status.
      #
      # Requests do not raise for HTTP status codes by default. This opt-in
      # check leaves the response body available.
      #
      # @return [Wreq::Response] The same response for a non-error status
      # @raise [Wreq::StatusError] If the status is in the 4xx or 5xx range
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response = client.get("https://example.com/missing")
      #   response.raise_for_status!
      def raise_for_status!
      end

      # Get the HTTP protocol version used.
      #
      # @return [Wreq::Version] HTTP version (HTTP/1.1, HTTP/2, etc.)
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.version  # => Wreq::Version::HTTP_11
      def version
      end

      # Get the final URL after redirects.
      #
      # @return [String] The final URL
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.url  # => "https://example.com/final-page"
      def url
      end

      # Get the content length if known.
      #
      # @return [Integer, nil] Content length in bytes, or nil if unknown
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.content_length  # => 1024
      def content_length
      end

      # Get the response headers.
      #
      # Header names are case-insensitive. Use {Wreq::Headers#get_all} to get
      # every value when a header appears more than once. Each call returns a
      # fresh, mutable snapshot. Changing that snapshot does not change the
      # response or a later snapshot, and object identity is not guaranteed.
      #
      # @return [Wreq::Headers] Response headers
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.headers.get("content-type")  # => "application/json"
      def headers
      end

      # Get the local socket address.
      #
      # @return [String, nil] Local address (e.g., "127.0.0.1:54321"), or nil
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.local_addr  # => "192.168.1.100:54321"
      def local_addr
      end

      # Get the remote socket address.
      #
      # @return [String, nil] Remote address (e.g., "93.184.216.34:443"), or nil
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.remote_addr  # => "93.184.216.34:443"
      def remote_addr
      end

      # Get cookies parsed from the response's `Set-Cookie` headers.
      #
      # Invalid `Set-Cookie` values are skipped.
      #
      # @return [Array<Wreq::Cookie>] Parsed response cookies
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.cookies.each do |cookie|
      #     puts "#{cookie.name}=#{cookie.value}"
      #   end
      def cookies
      end

      # Get the response bytes as a binary string.
      # @return [String] Response body as binary data
      # @raise [Wreq::MemoryError] if another body operation is active, or the
      #   body was streamed or closed
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   binary_data = response.bytes
      #   puts binary_data.size  # => 1024
      def bytes
      end

      # Get the response body as text with a specific charset.
      # This method allows you to specify a default encoding
      # to use when decoding the response body.
      # @param default_encoding [String] Default encoding to use (e.g., "UTF-8")
      # @return [String] Response body decoded as text using the specified encoding
      # @example
      #   html = response.text("ISO-8859-1")
      #   puts html
      # @raise [Wreq::MemoryError] if another body operation is active, or the
      #   body was streamed or closed
      # @raise [Wreq::DecodingError] if body cannot be decoded with the specified encoding
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      def text(default_encoding = "UTF-8")
      end

      # Parse the response body as JSON.
      #
      # Integral JSON numbers are returned as arbitrary-precision Ruby Integer
      # values. Fractional and exponent-form numbers are returned as Float values.
      #
      # @return [Object] Parsed JSON (Hash, Array, String, Integer, Float, Boolean, nil)
      # @raise [Wreq::MemoryError] if another body operation is active, or the
      #   body was streamed or closed
      # @raise [Wreq::DecodingError] if body is not valid JSON
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   data = response.json
      #   puts data["key"]
      def json
      end

      # Stream the response body, yielding each chunk to the given block.
      #
      # This method allows you to process large HTTP responses efficiently,
      # by yielding each chunk of the body as it arrives, without loading
      # the entire response into memory.
      #
      # @return [nil]
      # @yield [chunk] Each chunk of the response body as a binary String
      # @raise [LocalJumpError] if called without a block
      # @raise [Wreq::MemoryError] if another body operation is active, or the
      #   body was already read, streamed, or closed
      # @raise [Wreq::TimeoutError, Wreq::BodyError, Wreq::ConnectionResetError, Wreq::RequestError]
      #   if streaming fails while reading the response body
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example Save response to file
      #   File.open("output.bin", "wb") do |f|
      #     response.chunks { |chunk| f.write(chunk) }
      #   end
      # @example Count total bytes streamed
      #   total = 0
      #   response.chunks { |chunk| total += chunk.bytesize }
      #   puts "Downloaded #{total} bytes"
      #
      # Exceptions raised inside the block are propagated to the caller.
      def chunks
      end

      # Close the response and free associated resources.
      #
      # @return [void]
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   response.close
      def close
      end

      # Return TLS information captured for this response.
      #
      # Returns +nil+ when +tls_info: true+ was not enabled, the response used
      # plain HTTP, or the transport supplied no TLS information. Reading or
      # closing the response body does not discard captured TLS data.
      #
      # @return [Wreq::TlsInfo, nil] TLS information for this response, or +nil+
      #   when unavailable
      # @raise [Wreq::ForkError] if the response belongs to the parent process
      # @example
      #   client = Wreq::Client.new(tls_info: true)
      #   response = client.get("https://example.com")
      #   tls = response.tls_info
      #
      #   if tls
      #     tls.peer_certificate       # => DER-encoded binary String
      #     tls.peer_certificate_chain # => frozen Array of DER binary Strings
      #   end
      def tls_info
      end
    end
  end
end

# ======================== Ruby API Extensions ========================

module Wreq
  class Response
    # Returns the response body as a string.
    #
    # @return [String] Response body text
    # @raise [Wreq::ForkError] if the response belongs to the parent process
    # @example
    #   puts response.to_s
    #   puts response
    #   File.write("page.html", response)
    def to_s
      text
    end

    # Returns a compact string representation for debugging.
    #
    # Format: #<Wreq::Response STATUS content-type="..." body=SIZE>
    #
    # @return [String] Compact formatted response information
    # @raise [Wreq::ForkError] if the response belongs to the parent process
    # @example
    #   p response
    #   # => #<Wreq::Response 200 content-type="application/json" body=456B>
    def inspect
      parts = ["#<Wreq::Response"]

      parts << code.to_s

      if headers.respond_to?(:get)
        content_type = headers.get("content-type")
        parts << "content-type=#{content_type.inspect}" if content_type
      end

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
