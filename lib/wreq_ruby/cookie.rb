unless defined?(:Wreq)
  module Wreq
    # Cookie SameSite attribute.
    #
    # Values follow the Rust enum exposed by the native extension.
    class SameSite
      # Lax same-site policy.
      Strict = nil
      # Strict same-site policy.
      Lax = nil
      # Empty/None same-site policy.
      Empty = nil
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
      # @option options [Integer] :max_age Max-Age in seconds
      # @option options [Float] :expires Unix timestamp (seconds, float)
      # @option options [Boolean] :http_only HttpOnly flag
      # @option options [Boolean] :secure Secure flag
      # @option options [Wreq::SameSite] :same_site SameSite attribute
      # @return [Wreq::Cookie]
      # @example
      #   c = Wreq::Cookie.new(
      #     "sid", "abc",
      #     domain: "example.com",
      #     path: "/",
      #     max_age: 3600,
      #     http_only: true,
      #     secure: true,
      #     same_site: Wreq::SameSite::Lax
      #   )
      def new(name, value, **options); end

      # @return [String] Cookie name
      def name; end

      # @return [String] Cookie value
      def value; end

      # Returns true if the HttpOnly directive is enabled.
      # @return [Boolean]
      def http_only; end

      # Predicate version of http_only.
      # @return [Boolean]
      def http_only?; end

      # Returns true if the Secure directive is enabled.
      # @return [Boolean]
      def secure; end

      # Predicate version of secure.
      # @return [Boolean]
      def secure?; end

      # Returns true if SameSite is Lax.
      # @return [Boolean]
      def same_site_lax?; end

      # Returns true if SameSite is Strict.
      # @return [Boolean]
      def same_site_strict?; end

      # @return [String, nil] Path attribute
      def path; end

      # @return [String, nil] Domain attribute
      def domain; end

      # @return [Integer, nil] Max-Age in seconds
      def max_age; end

      # @return [Float, nil] Expires as Unix timestamp (seconds)
      def expires; end
    end

    # A cookie store (jar) used by the client to manage cookies across requests.
    class Jar
      # Create a new, empty cookie jar.
      # @return [Wreq::Jar]
      def new; end

      # Get all cookies currently stored.
      # @return [Array<Wreq::Cookie>]
      def get_all; end

      # Add a cookie object for the given URL.
      # @param cookie [Wreq::Cookie]
      # @param url [String]
      # @return [void]
      def add_cookie(cookie, url); end

      # Add a cookie from a Set-Cookie string for the given URL.
      # @param cookie [String] A Set-Cookie string
      # @param url [String]
      # @return [void]
      def add_cookie_str(cookie, url); end

      # Remove a cookie by name for the given URL.
      # @param name [String]
      # @param url [String]
      # @return [void]
      def remove(name, url); end

      # Clear all cookies from the jar.
      # @return [void]
      def clear; end
    end
  end
end
