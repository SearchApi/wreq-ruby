# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # HTTP response object containing status, headers, and body.
    #
    # This class wraps a native Rust implementation providing efficient
    # access to HTTP response data including status codes, headers, body
    # content, and streaming capabilities.
    #
    # Body methods raise Wreq::ForkError if the child inherited wreq-ruby from
    # its parent.
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
      def code
      end

      # Get the HTTP status code object.
      #
      # @return [Wreq::StatusCode] Status code wrapper with helper methods
      # @example
      #   status = response.status
      #   status.success?  # => true
      def status
      end

      # Get the HTTP protocol version used.
      #
      # @return [Wreq::Version] HTTP version (HTTP/1.1, HTTP/2, etc.)
      # @example
      #   response.version  # => Wreq::Version::HTTP_11
      def version
      end

      # Get the final URL after redirects.
      #
      # @return [String] The final URL
      # @example
      #   response.url  # => "https://example.com/final-page"
      def url
      end

      # Get the content length if known.
      #
      # @return [Integer, nil] Content length in bytes, or nil if unknown
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
      # @example
      #   response.headers.get("content-type")  # => "application/json"
      def headers
      end

      # Get the local socket address.
      #
      # @return [String, nil] Local address (e.g., "127.0.0.1:54321"), or nil
      # @example
      #   response.local_addr  # => "192.168.1.100:54321"
      def local_addr
      end

      # Get the remote socket address.
      #
      # @return [String, nil] Remote address (e.g., "93.184.216.34:443"), or nil
      # @example
      #   response.remote_addr  # => "93.184.216.34:443"
      def remote_addr
      end

      # Get cookies parsed from the response's `Set-Cookie` headers.
      #
      # Invalid `Set-Cookie` values are skipped.
      #
      # @return [Array<Wreq::Cookie>] Parsed response cookies
      # @example
      #   response.cookies.each do |cookie|
      #     puts "#{cookie.name}=#{cookie.value}"
      #   end
      def cookies
      end

      # Get the response bytes as a binary string.
      # @return [String] Response body as binary data
      # @raise [Wreq::ForkError] if the child inherited wreq-ruby from its parent
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
      # @raise [Wreq::DecodingError] if body cannot be decoded with the specified encoding
      # @raise [Wreq::ForkError] if the child inherited wreq-ruby from its parent
      def text(default_encoding = "UTF-8")
      end

      # Parse the response body as JSON.
      #
      # Integral JSON numbers are returned as arbitrary-precision Ruby Integer
      # values. Fractional and exponent-form numbers are returned as Float values.
      #
      # @return [Object] Parsed JSON (Hash, Array, String, Integer, Float, Boolean, nil)
      # @raise [Wreq::DecodingError] if body is not valid JSON
      # @raise [Wreq::ForkError] if the child inherited wreq-ruby from its parent
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
      # @raise [Wreq::TimeoutError, Wreq::BodyError, Wreq::ConnectionResetError, Wreq::RequestError]
      #   if streaming fails while reading the response body
      # @raise [Wreq::ForkError] if the child inherited wreq-ruby from its parent
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
      # @raise [Wreq::ForkError] if the child inherited wreq-ruby from its parent
      # @example
      #   response.close
      def close
      end

      # Get the redirect history for this response.
      #
      # Returns an ordered, frozen array of {Wreq::RedirectHistoryEntry} objects
      # representing each hop followed during the request. When no redirects
      # were followed (including when redirects are disabled), returns an empty
      # frozen array.
      #
      # History is available regardless of whether the response body has been
      # consumed or closed.
      #
      # @return [Array<Wreq::RedirectHistoryEntry>] Ordered redirect hops (frozen)
      #
      # @example No redirects
      #   response = client.get("https://example.com/page")
      #   response.history  # => []
      #
      # @example Single redirect
      #   response = client.get("https://example.com/old",
      #     allow_redirects: true)
      #   response.history.length  # => 1
      #   hop = response.history[0]
      #   hop.status        # => 301
      #   hop.previous_url  # => "https://example.com/old"
      #   hop.url           # => "https://example.com/new"
      #
      # @example Iterating over multiple hops
      #   response.history.each do |hop|
      #     puts "#{hop.status}: #{hop.previous_url} -> #{hop.url}"
      #   end
      #
      # @example Converting to hashes
      #   response.history.map(&:to_h)
      def history
      end
    end

    # A single hop in the redirect history of a response.
    #
    # Each entry captures the status code, source and destination URLs,
    # and headers from one intermediate redirect response. Entries are
    # immutable value objects.
    #
    # Sensitive URL components (query strings, userinfo) are redacted
    # from {#inspect} and {#to_s} output.
    #
    # @see Response#history
    class RedirectHistoryEntry
      # The HTTP status code of the redirect response.
      #
      # @return [Integer] Status code (e.g., 301, 302, 307, 308)
      # @example
      #   hop.status  # => 301
      def status
      end

      # The resolved destination URL of the redirect.
      #
      # @return [String] The URL that was redirected to
      # @example
      #   hop.url  # => "https://example.com/new-page"
      def url
      end

      # The URL that was requested before this redirect occurred.
      #
      # @return [String] The source URL of the redirect
      # @example
      #   hop.previous_url  # => "https://example.com/old-page"
      def previous_url
      end

      # The headers from the redirect response.
      #
      # Returns a mutable snapshot of the intermediate response headers.
      # Duplicate header values are preserved.
      #
      # @return [Wreq::Headers] Headers from the redirect response
      # @example
      #   hop.headers.get("location")  # => "https://example.com/new"
      def headers
      end

      # Convert this entry to a Hash with symbol keys.
      #
      # @return [Hash{Symbol => Object}] Hash with +:status+, +:url+,
      #   +:previous_url+, and +:headers+ keys
      # @example
      #   hop.to_h
      #   # => { status: 301, url: "...", previous_url: "...",
      #   #      headers: #<Wreq::Headers ...> }
      def to_h
      end

      # Return a compact, safe string representation.
      #
      # Query strings and userinfo are redacted from URLs.
      #
      # @return [String] Formatted entry for debugging
      # @example
      #   hop.inspect
      #   # => "#<Wreq::RedirectHistoryEntry 301 https://example.com/old -> https://example.com/new>"
      def inspect
      end

      # Return the same representation as {#inspect}.
      #
      # @return [String]
      def to_s
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
