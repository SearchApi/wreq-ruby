# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # HTTP headers collection.
    #
    # Provides efficient access to HTTP headers with case-insensitive lookups.
    # Headers are created by the native extension and cannot be directly instantiated.
    #
    # All header names are case-insensitive. Multiple values for the same header
    # are supported through the `get_all` method.
    #
    # @example Accessing response headers
    #   response = Wreq.get("https://example.com")
    #   headers = response.headers
    #   content_type = headers["Content-Type"]
    #   content_type = headers.get("content-type")  # Same, case-insensitive
    #
    # @example Getting all values for a header
    #   accept_values = headers.get_all("Accept")
    #   # => ["application/json", "text/html"]
    #
    # @example Modifying headers
    #   headers.set("X-Custom-Header", "value")
    #   headers["Authorization"] = "Bearer token"
    #   headers.append("Accept", "application/xml")
    #
    # @example Iterating headers
    #   headers.each do |name, value|
    #     puts "#{name}: #{value}"
    #   end
    #
    # @example Converting to hash
    #   hash = headers.to_h
    #   hash["content-type"]  # => "text/html"
    class Headers
      # Create a new empty Headers collection.
      #
      # @return [Wreq::Headers] New headers instance
      # @example
      #   headers = Wreq::Headers.new
      #   headers.set("Content-Type", "application/json")
      def self.new
      end

      # Get a header value by name (case-insensitive).
      #
      # Returns the first value if multiple values exist for the same header.
      #
      # @param name [String] Header name (case-insensitive)
      # @return [String, nil] Header value, or nil if not found
      # @example
      #   headers.get("Content-Type")       # => "application/json"
      #   headers.get("content-type")       # => "application/json" (same)
      #   headers.get("X-Nonexistent")      # => nil
      def get(name)
      end

      # Get all values for a header name (case-insensitive).
      #
      # Useful when a header can have multiple values (e.g., Accept, Set-Cookie).
      #
      # @param name [String] Header name (case-insensitive)
      # @return [Array<String>] All values for this header (empty array if not found)
      # @example
      #   headers.get_all("Accept")
      #   # => ["application/json", "text/html", "application/xml"]
      #   headers.get_all("X-Nonexistent")  # => []
      def get_all(name)
      end

      # Set a header value, replacing any existing values.
      #
      # @param name [String] Header name
      # @param value [String] Header value
      # @return [void]
      # @raise [Wreq::BuilderError] if name or value contains invalid characters
      # @example
      #   headers.set("Content-Type", "application/json")
      #   headers.set("X-Custom-Header", "my-value")
      def set(name, value)
      end

      # Append a header value without replacing existing values.
      #
      # Adds a new value for the header, preserving any existing values.
      # Useful for headers that can have multiple values.
      #
      # @param name [String] Header name
      # @param value [String] Header value to append
      # @return [void]
      # @raise [Wreq::BuilderError] if name or value contains invalid characters
      # @example
      #   headers.set("Accept", "application/json")
      #   headers.append("Accept", "text/html")
      #   headers.get_all("Accept")  # => ["application/json", "text/html"]
      def append(name, value)
      end

      # Remove all values for a header name.
      #
      # @param name [String] Header name (case-insensitive)
      # @return [String, nil] The removed value (first one if multiple), or nil
      # @example
      #   headers.remove("Authorization")  # => "Bearer token"
      #   headers.remove("X-Nonexistent")  # => nil
      def remove(name)
      end

      # Check if a header exists (case-insensitive).
      #
      # @param name [String] Header name
      # @return [Boolean] true if the header exists
      # @example
      #   headers.contains?("Content-Type")  # => true
      #   headers.contains?("X-Missing")     # => false
      def contains?(name)
      end

      # Check if a header key exists (alias for {#contains?}).
      #
      # @param name [String] Header name
      # @return [Boolean] true if the header exists
      # @example
      #   headers.key?("Accept")  # => true
      def key?(name)
      end

      # Get the number of headers.
      #
      # @return [Integer] Total number of unique header names
      # @example
      #   headers.length  # => 12
      def length
      end

      # Check if there are no headers.
      #
      # @return [Boolean] true if no headers exist
      # @example
      #   headers.empty?  # => false
      def empty?
      end

      # Remove all headers.
      #
      # @return [void]
      # @example
      #   headers.clear
      #   headers.empty?  # => true
      def clear
      end

      # Get all header names.
      #
      # @return [Array<String>] Array of header names (lowercase)
      # @example
      #   headers.keys
      #   # => ["content-type", "accept", "user-agent", "authorization"]
      def keys
      end

      # Get all header values.
      #
      # Returns one value per header (the first if multiple values exist).
      #
      # @return [Array<String>] Array of header values
      # @example
      #   headers.values
      #   # => ["application/json", "text/html", "Mozilla/5.0", "Bearer token"]
      def values
      end

      # Iterate over headers.
      #
      # Yields each header name and value pair. If a header has multiple values,
      # only the first is yielded.
      #
      # @yieldparam name [String] Header name (lowercase)
      # @yieldparam value [String] Header value
      # @return [Enumerator, self] Returns enumerator if no block given, self otherwise
      # @example With block
      #   headers.each do |name, value|
      #     puts "#{name}: #{value}"
      #   end
      # @example Without block
      #   enum = headers.each
      #   enum.to_a  # => [["content-type", "text/html"], ...]
      def each
      end

      # Convert headers to a string representation.
      def to_s
      end
    end
  end
end
