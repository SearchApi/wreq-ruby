unless defined?(Wreq)
  module Wreq
    # SameSite values for HTTP cookies.
    #
    # The constant names match the native Rust variants.
    # standard:disable Naming/ConstantName
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
    # standard:enable Naming/ConstantName

    # A single HTTP cookie backed by an immutable native value.
    #
    # Cookie instances can be shared safely between threads. Pass optional
    # attributes as keywords to {.new}.
    class Cookie
      # Creates a Cookie instance.
      #
      # The native extension defines `.new` directly instead of `#initialize`.
      #
      # @param name [String] Cookie name
      # @param value [String] Cookie value
      # @param options [Hash] Optional keyword arguments
      # @option options [String] :domain Domain attribute
      # @option options [String] :path Path attribute
      # @option options [Integer] :max_age Signed Max-Age in seconds.
      #   Zero or negative values expire the cookie immediately.
      # @option options [Time, Numeric] :expires A Time or finite Unix timestamp in seconds
      # @option options [Boolean] :http_only HttpOnly flag
      # @option options [Boolean] :secure Secure flag
      # @option options [Wreq::SameSite] :same_site SameSite attribute
      # @return [Wreq::Cookie]
      # @raise [ArgumentError] if an option is unknown or duplicated, or if :expires is not finite
      # @raise [RangeError] if :max_age or :expires is outside the supported range
      # @raise [TypeError] if an option has the wrong type
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

      # Returns the SameSite setting, or nil if it was omitted.
      # @return [Wreq::SameSite, nil]
      def same_site
      end

      # @return [String, nil] Path attribute
      def path
      end

      # @return [String, nil] Domain attribute
      def domain
      end

      # Returns the signed Max-Age lifetime in seconds.
      #
      # Zero and negative values expire the cookie immediately.
      # @return [Integer, nil]
      def max_age
      end

      # Returns the expiration time in UTC.
      # @return [Time, nil]
      def expires_at
      end

      # Returns the expiration as a Unix timestamp with fractional seconds.
      # The Float return value may lose precision for large timestamps.
      # @deprecated Use {#expires_at}, which returns Time.
      # @return [Float, nil]
      def expires
      end

      # Returns the cookie formatted as a Set-Cookie value.
      # @return [String]
      def to_s
      end
    end

    # Stores cookies for reuse across requests.
    #
    # Pass a Jar to Wreq::Client as `cookie_provider` to share its cookies.
    class Jar
      # Creates an empty cookie jar.
      # @return [Wreq::Jar]
      def self.new
      end

      # Returns all stored cookies.
      # @return [Array<Wreq::Cookie>]
      def get_all
      end

      # Adds a Cookie object or Set-Cookie string for the given URL.
      # @param cookie [String, Wreq::Cookie] Cookie to store
      # @param url [String] URL that scopes the cookie
      # @return [void]
      # @raise [TypeError] if cookie is neither a String nor Wreq::Cookie
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
    # Returns a short representation for debugging.
    # @return [String]
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
    # Returns a short representation with the number of stored cookies.
    # @return [String]
    def inspect
      "#<Wreq::Jar [#{get_all.length} cookies]>"
    end
  end
end
