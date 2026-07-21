unless defined?(Wreq)
  module Wreq
    # Cookie SameSite attribute.
    #
    # Values follow the Rust enum exposed by the native extension.
    class SameSite
      # Strict same-site policy.
      Strict = nil
      # Lax same-site policy.
      Lax = nil
      # None same-site policy.
      None = nil

      # Returns the SameSite attribute name (e.g. "Strict", "Lax", "None").
      # @return [String]
      def to_s
      end

      # Returns the SameSite attribute as a lowercase symbol (e.g. :strict, :lax, :none).
      # @return [Symbol]
      def to_sym
      end

      # Value-based equality.
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

    # A single HTTP cookie.
    #
    # Thread-safe: instances are backed by an immutable Rust value and can be
    # shared across threads safely. This mirrors the native `Wreq::Cookie`.
    # Constructor accepts `name`, `value`, plus optional keyword arguments for
    # other attributes.
    class Cookie
      # Create a new Cookie instance.
      #
      # Note: This matches the native binding which defines `new` (not `initialize`).
      #
      # @param name [String] Cookie name
      # @param value [String] Cookie value
      # @param options [Hash] Optional keyword arguments
      # @option options [String] :domain Domain attribute
      # @option options [String] :path Path attribute
      # @option options [Integer] :max_age Signed Max-Age in seconds; zero or negative expires immediately
      # @option options [Time, Numeric] :expires Expiration time or finite Unix timestamp in seconds
      # @option options [Boolean] :http_only HttpOnly flag
      # @option options [Boolean] :secure Secure flag
      # @option options [Wreq::SameSite] :same_site SameSite attribute
      # @return [Wreq::Cookie]
      # @raise [ArgumentError] if an option is unknown, duplicated, or otherwise invalid
      # @raise [RangeError] if Max-Age or the expiration is outside the supported range
      # @raise [TypeError] if an option has an incompatible value type
      # @example
      #   c = Wreq::Cookie.new(
      #     "sid", "abc",
      #     domain: "example.com",
      #     path: "/",
      #     max_age: 3600,
      #     expires: Time.utc(2030, 1, 1),
      #     http_only: true,
      #     secure: true,
      #     same_site: Wreq::SameSite::Lax
      #   )
      def self.new(name, value, **options)
      end

      # @return [String] Cookie name
      def name
      end

      # @return [String] Cookie value
      def value
      end

      # Returns true if the HttpOnly directive is enabled.
      # @return [Boolean]
      def http_only
      end

      # Predicate version of http_only.
      # @return [Boolean]
      def http_only?
      end

      # Returns true if the Secure directive is enabled.
      # @return [Boolean]
      def secure
      end

      # Predicate version of secure.
      # @return [Boolean]
      def secure?
      end

      # Returns true if SameSite is Lax.
      # @return [Boolean]
      def same_site_lax?
      end

      # Returns true if SameSite is Strict.
      # @return [Boolean]
      def same_site_strict?
      end

      # @return [String, nil] Path attribute
      def path
      end

      # @return [String, nil] Domain attribute
      def domain
      end

      # Returns the signed Max-Age in seconds.
      #
      # Zero and negative values indicate immediate expiration.
      # @return [Integer, nil]
      def max_age
      end

      # Returns the expiration as a UTC Ruby Time.
      # @return [Time, nil]
      def expires_at
      end

      # Returns the expiration as fractional Unix seconds.
      # Large timestamps may lose precision when represented as a Float.
      # @deprecated Use {#expires_at} for a Ruby-native time value.
      # @return [Float, nil]
      def expires
      end
    end

    # A cookie store (jar) used by the client to manage cookies across requests.
    class Jar
      # Create a new, empty cookie jar.
      # @return [Wreq::Jar]
      def self.new
      end

      # Get all cookies currently stored.
      # @return [Array<Wreq::Cookie>]
      def get_all
      end

      # Add a cookie from a Set-Cookie string for the given URL.
      # @param cookie [String, Wreq::Cookie] A Set-Cookie string
      # @param url [String]
      # @return [void]
      def add(cookie, url)
      end

      # Remove a cookie by name for the given URL.
      # @param name [String]
      # @param url [String]
      # @return [void]
      def remove(name, url)
      end

      # Clear all cookies from the jar.
      # @return [void]
      def clear
      end
    end
  end
end

module Wreq
  class Cookie
    def inspect
      parts = ["#<Wreq::Cookie", name]
      parts << "domain=#{domain}" if domain
      parts << "path=#{path}" if path
      parts << "secure" if secure?
      parts << "http_only" if http_only?
      parts.join(" ") + ">"
    end
  end

  class Jar
    def inspect
      "#<Wreq::Jar [#{get_all.length} cookies]>"
    end
  end
end
