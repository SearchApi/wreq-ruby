# frozen_string_literal: true

module Wreq
  # HTTP method enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::Method}.
  #
  # @example Using predefined constants
  #   method = Wreq::Method::GET
  #   method.class #=> Wreq::Method
  #
  # @example In request context
  #   Wreq.request(url: "https://api.example.com", method: Wreq::Method::POST)
  class Method
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:GET)
      GET = nil # @return [Wreq::Method] HTTP GET method
      HEAD = nil # @return [Wreq::Method] HTTP HEAD method
      POST = nil # @return [Wreq::Method] HTTP POST method
      PUT = nil # @return [Wreq::Method] HTTP PUT method
      DELETE = nil # @return [Wreq::Method] HTTP DELETE method
      OPTIONS = nil # @return [Wreq::Method] HTTP OPTIONS method
      TRACE = nil # @return [Wreq::Method] HTTP TRACE method
      PATCH = nil # @return [Wreq::Method] HTTP PATCH method
    end

    # Returns the HTTP method token (e.g. "GET", "POST").
    # @return [String]
    unless method_defined?(:to_s)
      def to_s
      end
    end

    # Returns the HTTP method as a lowercase symbol (e.g. :get, :post).
    # @return [Symbol]
    unless method_defined?(:to_sym)
      def to_sym
      end
    end

    # Value-based equality. Returns true when both represent the same HTTP method.
    # @param other [Object]
    # @return [Boolean]    
    unless method_defined?(:==)
      def ==(other)
      end
    end

    # Strict equality for Hash key and Set member semantics.
    # @param other [Object]
    # @return [Boolean]
    unless method_defined?(:eql?)
      def eql?(other)
      end
    end

    # Hash value consistent with {#eql?} for use as Hash keys.
    # @return [Integer]
    unless method_defined?(:hash)
      def hash
      end
    end
  end

  # HTTP version enumeration backed by Rust.
  #
  # @example Using predefined constants
  #   version = Wreq::Version::HTTP_11
  #   version.class #=> Wreq::Version
  class Version
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:HTTP_11)
      HTTP_09 = nil # @return [Wreq::Version] HTTP/0.9
      HTTP_10 = nil # @return [Wreq::Version] HTTP/1.0
      HTTP_11 = nil # @return [Wreq::Version] HTTP/1.1
      HTTP_2 = nil # @return [Wreq::Version] HTTP/2
      HTTP_3 = nil # @return [Wreq::Version] HTTP/3
    end

    # Returns a string representation of the HTTP version.
    # @return [String] HTTP version as string
    unless method_defined?(:to_s)
      def to_s
      end
    end

    # Compares HTTP versions by semantic value, not object identity.
    #
    # This method is implemented by the native extension.
    # When comparing with non-{Wreq::Version} objects, it returns false.
    #
    # @param other [Object] object to compare against
    # @return [Boolean] true when both represent the same HTTP version
    # @example
    #   Wreq::Version::HTTP_11 == response.version
    unless method_defined?(:==)
      def ==(other)
      end
    end

    # Strict equality for Hash key and Set member semantics.
    # @param other [Object]
    # @return [Boolean]
    unless method_defined?(:eql?)
      def eql?(other)
      end
    end

    # Hash value consistent with {#eql?} for use as Hash keys.
    # @return [Integer]
    unless method_defined?(:hash)
      def hash
      end
    end
  end

  # HTTP status code wrapper.
  #
  # This class wraps standard HTTP status codes and provides
  # convenient methods to check the response category.
  #
  # The actual implementation is provided by Rust for performance.
  #
  # @example Check if response is successful
  #   status = response.status
  #   if status.success?
  #     puts "Request succeeded with code: #{status.as_int}"
  #   end
  #
  # @example Check different status categories
  #   status.informational?  # 1xx
  #   status.success?        # 2xx
  #   status.redirection?    # 3xx
  #   status.client_error?   # 4xx
  #   status.server_error?   # 5xx
  unless const_defined?(:StatusCode)
    class StatusCode
      # Returns the status code as an integer.
      #
      # @return [Integer] the numeric HTTP status code (100-599)
      def as_int
      end

      # Checks if status code is informational (1xx).
      #
      # Informational responses indicate that the request was received
      # and the process is continuing.
      #
      # @return [Boolean] true if status is 100-199
      def informational?
      end

      # Checks if status code indicates success (2xx).
      #
      # Success responses indicate that the request was successfully
      # received, understood, and accepted.
      #
      # @return [Boolean] true if status is 200-299
      def success?
      end

      # Checks if status code indicates redirection (3xx).
      #
      # Redirection responses indicate that further action needs to be
      # taken to complete the request.
      #
      # @return [Boolean] true if status is 300-399
      def redirection?
      end

      # Checks if status code indicates client error (4xx).
      #
      # Client error responses indicate that the request contains bad
      # syntax or cannot be fulfilled.
      #
      # @return [Boolean] true if status is 400-499
      def client_error?
      end

      # Checks if status code indicates server error (5xx).
      #
      # Server error responses indicate that the server failed to
      # fulfill a valid request.
      #
      # @return [Boolean] true if status is 500-599
      def server_error?
      end

      # Returns a string representation of the status code.
      # @return [String] Status code as string
      def to_s
      end

      # Returns the status code as an integer.
      # @return [Integer] the numeric HTTP status code
      def to_i
      end

      # Value-based equality. Only compares with other StatusCode instances.
      # @param other [Object]
      # @return [Boolean]
      def ==(other)
      end

      # Strict equality for Hash key and Set member semantics.
      # @param other [Object]
      # @return [Boolean]
      def eql?(other)
      end

      # Hash value consistent with {#eql?} for use as Hash keys.
      # @return [Integer]
      def hash
      end
    end
  end
end

module Wreq
  class StatusCode
    def inspect
      "#<Wreq::StatusCode #{self}>"
    end
  end

  class Version
    def inspect
      "#<Wreq::Version #{self}>"
    end
  end
end
