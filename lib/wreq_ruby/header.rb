# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # A mutable, case-insensitive collection of HTTP headers.
    #
    # Header names are normalized by the native header map and lookups are
    # case-insensitive. Use the `orig_headers` request option when exact wire
    # casing or header order is required. Duplicate values are stored as
    # separate header occurrences.
    #
    # @example Build and query headers
    #   headers = Wreq::Headers.new(
    #     "Accept" => ["application/json", "text/plain"],
    #     content_type: "application/json"
    #   )
    #   headers["accept"]       # => ["application/json", "text/plain"]
    #   headers[:content_type]  # => "application/json"
    #
    # @example Iterate over every occurrence
    #   headers.each do |name, value|
    #     puts "#{name}: #{value}"
    #   end
    class Headers
      include Enumerable

      # Create an empty collection or copy header pairs from a source.
      #
      # @param source [Hash, Wreq::Headers, Enumerable] A hash, another
      #   headers collection, or an enumerable that yields name-value pairs.
      #   Omit this argument to create an empty collection.
      # @return [Wreq::Headers]
      # @raise [Wreq::BuilderError] if the source does not contain valid pairs
      # @example
      #   Wreq::Headers.new
      #   Wreq::Headers.new("Accept" => "application/json")
      #   Wreq::Headers.new([[:content_type, "application/json"]])
      def self.new(*args)
      end

      # Return the first value for a header.
      #
      # @param name [String, Symbol] Header name
      # @return [String, nil] The first value, or nil when the name is missing
      # @example
      #   headers.get("content-type")  # => "application/json"
      #   headers.get(:missing)        # => nil
      def get(name)
      end

      # Return a header using collection-style value semantics.
      #
      # A missing name returns nil, one occurrence returns a String, and
      # multiple occurrences return an Array<String>.
      #
      # @param name [String, Symbol] Header name
      # @return [String, Array<String>, nil]
      # @example
      #   headers["accept"]      # => "application/json"
      #   headers["set-cookie"]  # => ["a=1", "b=2"]
      def [](name)
      end

      # Return every value for a header.
      #
      # @param name [String, Symbol] Header name
      # @return [Array<String>] Values in insertion order, or an empty array
      # @example
      #   headers.get_all("set-cookie")  # => ["a=1", "b=2"]
      #   headers.get_all(:missing)       # => []
      def get_all(name)
      end

      # Set one or more values, replacing every existing occurrence.
      #
      # Array values are stored as separate occurrences and are not joined. An
      # empty Array removes the header.
      #
      # @param name [String, Symbol] Header name
      # @param value [String, Array<String>] Header value or values
      # @return [void]
      # @raise [Wreq::BuilderError] if a name or value is invalid
      # @example
      #   headers.set("Accept", ["application/json", "text/plain"])
      def set(name, value)
      end

      # Set one or more values and return the assigned value.
      #
      # @param name [String, Symbol] Header name
      # @param value [String, Array<String>] Header value or values
      # @return [String, Array<String>] The assigned value
      # @example
      #   headers[:content_type] = "application/json"
      def []=(name, value)
      end

      # Append one or more values without replacing existing occurrences.
      #
      # @param name [String, Symbol] Header name
      # @param value [String, Array<String>] Header value or values
      # @return [void]
      # @raise [Wreq::BuilderError] if a name or value is invalid
      # @example
      #   headers.append("Set-Cookie", ["a=1", "b=2"])
      def append(name, value)
      end

      # Return a header value, a fallback, or the result of a block.
      #
      # @param name [String, Symbol] Header name
      # @param default [Object] Optional fallback for a missing name
      # @yieldparam name [String, Symbol] The missing name
      # @return [String, Array<String>, Object]
      # @raise [KeyError] if the name is missing and no fallback is provided
      # @example
      #   headers.fetch("accept", "*/*")
      #   headers.fetch(:missing) { |name| "missing: #{name}" }
      def fetch(name, default = nil)
      end

      # Remove every occurrence for a header.
      #
      # @param name [String, Symbol] Header name
      # @return [String, nil] The first removed value, or nil when missing
      # @example
      #   headers.remove("authorization")  # => "Bearer token"
      def remove(name)
      end

      # Remove every occurrence for a header. Alias for {#remove}.
      #
      # @param name [String, Symbol] Header name
      # @return [String, nil] The first removed value, or nil when missing
      def delete(name)
      end

      # Check whether a header exists.
      #
      # @param name [String, Symbol] Header name
      # @return [Boolean]
      def contains?(name)
      end

      # Check whether a header exists. Alias for {#contains?}.
      #
      # @param name [String, Symbol] Header name
      # @return [Boolean]
      def key?(name)
      end

      # Return the number of header occurrences.
      #
      # This can be greater than `keys.length` when a name has multiple values.
      #
      # @return [Integer]
      def length
      end

      # Return the number of header occurrences. Alias for {#length}.
      #
      # @return [Integer]
      def size
      end

      # Check whether the collection has no header occurrences.
      #
      # @return [Boolean]
      def empty?
      end

      # Remove every header occurrence.
      #
      # @return [Wreq::Headers] self
      def clear
      end

      # Return each unique header name.
      #
      # @return [Array<String>]
      def keys
      end

      # Return every header value.
      #
      # @return [Array<String>]
      def values
      end

      # Iterate over every header occurrence.
      #
      # @yieldparam name [String] Normalized lowercase header name
      # @yieldparam value [String] Header value
      # @return [Enumerator, Wreq::Headers] An Enumerator without a block,
      #   otherwise self
      # @example
      #   headers.each.to_a
      def each
      end

      # Convert every occurrence to name-value pairs.
      #
      # @return [Array<Array(String, String)>]
      # @example
      #   headers.to_a  # => [["accept", "application/json"], ...]
      def to_a
      end

      # Convert unique names to a Hash.
      #
      # Hash values use the same nil, String, or Array shape as {#[]}.
      #
      # @return [Hash{String => String, Array<String>}]
      # @example
      #   headers.to_h  # => {"accept" => "application/json"}
      def to_h
      end

      # Convert unique names to a Hash. Alias for {#to_h}.
      #
      # @return [Hash{String => String, Array<String>}]
      def to_hash
      end

      # Convert headers to a string representation.
      #
      # @return [String]
      def to_s
      end

      # Return a compact representation for debugging.
      #
      # @return [String]
      # @example
      #   headers.inspect  # => "#<Wreq::Headers [2 headers]>"
      def inspect
      end
    end
  end
end

# ======================== Ruby API Extensions ========================

module Wreq
  class Headers
    FETCH_UNDEFINED = Object.new.freeze
    private_constant :FETCH_UNDEFINED

    alias delete remove
    alias size length

    # Return a header value, a fallback, or the result of a block.
    #
    # The block takes precedence when both a default and block are provided.
    #
    # @param name [String, Symbol] Header name
    # @param default [Object] Optional fallback for a missing name
    # @yieldparam name [String, Symbol] The missing name
    # @return [String, Array<String>, Object]
    # @raise [KeyError] if the name is missing and no fallback is provided
    def fetch(name, default = FETCH_UNDEFINED)
      value = self[name]
      return value unless value.nil?
      return yield(name) if block_given?
      return default unless default.equal?(FETCH_UNDEFINED)

      raise KeyError, "key not found: #{name.inspect}"
    end

    # Convert every header occurrence to a name-value pair.
    #
    # @return [Array<Array(String, String)>]
    def to_a
      each.to_a
    end

    # Convert unique normalized names to a Hash.
    #
    # A name with one occurrence maps to a String, while multiple occurrences
    # map to an Array<String>.
    #
    # @return [Hash{String => String, Array<String>}]
    def to_h
      keys.to_h { |name| [name, self[name]] }
    end

    alias to_hash to_h

    # Return a compact representation for debugging.
    #
    # @return [String]
    def inspect
      "#<Wreq::Headers [#{length} headers]>"
    end
  end
end
