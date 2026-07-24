# frozen_string_literal: true

# Profile and platform constants mirror the native enum variant names.
# standard:disable Naming/ConstantName

module Wreq
  # Browser and client fingerprint profile enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::Profile} and can be passed to
  # {Wreq::Emulation.new} via the `profile:` keyword.
  #
  # @example Using a predefined profile
  #   profile = Wreq::Profile::Chrome117
  #   profile.class #=> Wreq::Profile
  #
  # @example Applying a profile to emulation
  #   emu = Wreq::Emulation.new(profile: Wreq::Profile::Chrome117)
  class Profile
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:Chrome100)
      Chrome100 = nil
      Chrome101 = nil
      Chrome104 = nil
      Chrome105 = nil
      Chrome106 = nil
      Chrome107 = nil
      Chrome108 = nil
      Chrome109 = nil
      Chrome110 = nil
      Chrome114 = nil
      Chrome116 = nil
      Chrome117 = nil
      Chrome118 = nil
      Chrome119 = nil
      Chrome120 = nil
      Chrome123 = nil
      Chrome124 = nil
      Chrome126 = nil
      Chrome127 = nil
      Chrome128 = nil
      Chrome129 = nil
      Chrome130 = nil
      Chrome131 = nil
      Chrome132 = nil
      Chrome133 = nil
      Chrome134 = nil
      Chrome135 = nil
      Chrome136 = nil
      Chrome137 = nil
      Chrome138 = nil
      Chrome139 = nil
      Chrome140 = nil
      Chrome141 = nil
      Chrome142 = nil
      Chrome143 = nil
      Chrome144 = nil
      Chrome145 = nil
      Chrome146 = nil
      Chrome147 = nil
      Chrome148 = nil
      Chrome149 = nil
      Chrome150 = nil

      Edge101 = nil
      Edge122 = nil
      Edge127 = nil
      Edge131 = nil
      Edge134 = nil
      Edge135 = nil
      Edge136 = nil
      Edge137 = nil
      Edge138 = nil
      Edge139 = nil
      Edge140 = nil
      Edge141 = nil
      Edge142 = nil
      Edge143 = nil
      Edge144 = nil
      Edge145 = nil
      Edge146 = nil
      Edge147 = nil
      Edge148 = nil

      Firefox109 = nil
      Firefox117 = nil
      Firefox128 = nil
      Firefox133 = nil
      Firefox135 = nil
      FirefoxPrivate135 = nil
      FirefoxAndroid135 = nil
      Firefox136 = nil
      FirefoxPrivate136 = nil
      Firefox139 = nil
      Firefox142 = nil
      Firefox143 = nil
      Firefox144 = nil
      Firefox145 = nil
      Firefox146 = nil
      Firefox147 = nil
      Firefox148 = nil
      Firefox149 = nil
      Firefox150 = nil
      Firefox151 = nil

      SafariIos17_2 = nil
      SafariIos17_4_1 = nil
      SafariIos16_5 = nil
      Safari15_3 = nil
      Safari15_5 = nil
      Safari15_6_1 = nil
      Safari16 = nil
      Safari16_5 = nil
      Safari17_0 = nil
      Safari17_2_1 = nil
      Safari17_4_1 = nil
      Safari17_5 = nil
      Safari17_6 = nil
      Safari18 = nil
      SafariIPad18 = nil
      Safari18_2 = nil
      Safari18_3 = nil
      Safari18_3_1 = nil
      SafariIos18_1_1 = nil
      Safari18_5 = nil
      Safari26 = nil
      Safari26_1 = nil
      Safari26_2 = nil
      Safari26_3 = nil
      Safari26_4 = nil
      SafariIos26 = nil
      SafariIos26_2 = nil
      SafariIPad26 = nil
      SafariIpad26_2 = nil

      OkHttp3_9 = nil
      OkHttp3_11 = nil
      OkHttp3_13 = nil
      OkHttp3_14 = nil
      OkHttp4_9 = nil
      OkHttp4_10 = nil
      OkHttp4_12 = nil
      OkHttp5 = nil

      Opera116 = nil
      Opera117 = nil
      Opera118 = nil
      Opera119 = nil
      Opera120 = nil
      Opera121 = nil
      Opera122 = nil
      Opera123 = nil
      Opera124 = nil
      Opera125 = nil
      Opera126 = nil
      Opera127 = nil
      Opera128 = nil
      Opera129 = nil
      Opera130 = nil
      Opera131 = nil
    end

    unless method_defined?(:to_s)
      # Returns the profile name.
      # @return [String] Profile name as a string
      def to_s
      end
    end

    unless method_defined?(:==)
      # Value-based equality.
      # @param other [Object]
      # @return [Boolean]
      def ==(other)
      end
    end

    unless method_defined?(:eql?)
      # Strict equality for Hash key and Set member semantics.
      # @param other [Object]
      # @return [Boolean]
      def eql?(other)
      end
    end

    unless method_defined?(:hash)
      # Hash value consistent with {#eql?} for use as Hash keys.
      # @return [Integer]
      def hash
      end
    end
  end

  # Operating system platform enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::Platform} and can be passed to
  # {Wreq::Emulation.new} via the `platform:` keyword.
  #
  # @example Using a predefined platform
  #   platform = Wreq::Platform::Windows
  #   platform.class #=> Wreq::Platform
  #
  # @example Applying a platform to emulation
  #   emu = Wreq::Emulation.new(platform: Wreq::Platform::Windows)
  class Platform
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:Windows)
      Windows = nil
      MacOS = nil
      Linux = nil
      Android = nil
      IOS = nil
    end

    unless method_defined?(:to_s)
      # Returns the platform name.
      # @return [String] Platform name as a string
      def to_s
      end
    end

    unless method_defined?(:to_sym)
      # Returns the platform as a lowercase symbol (e.g. :windows, :linux).
      # @return [Symbol]
      def to_sym
      end
    end

    unless method_defined?(:==)
      # Value-based equality.
      # @param other [Object]
      # @return [Boolean]
      def ==(other)
      end
    end

    unless method_defined?(:eql?)
      # Strict equality for Hash key and Set member semantics.
      # @param other [Object]
      # @return [Boolean]
      def eql?(other)
      end
    end

    unless method_defined?(:hash)
      # Hash value consistent with {#eql?} for use as Hash keys.
      # @return [Integer]
      def hash
      end
    end
  end

  # Emulation option wrapper.
  #
  # This class combines a fingerprint `profile`, an OS `platform`, and toggles
  # for HTTP/2 and automatic default headers. The actual implementation is
  # provided by Rust.
  #
  # `profile:` defaults to the library's default profile when omitted.
  # `platform:` defaults to the library's default platform when omitted.
  #
  # @example Create an emulation option
  #   emu = Wreq::Emulation.new(
  #     profile: Wreq::Profile::Chrome117,
  #     platform: Wreq::Platform::Windows,
  #     http2: true,
  #     headers: true
  #   )
  #
  # @param profile [Wreq::Profile, nil] Fingerprint profile to emulate
  # @param platform [Wreq::Platform, nil] Operating system platform to emulate
  # @param http2 [Boolean, nil] Whether HTTP/2 emulation is enabled; defaults
  #   to true when omitted or nil
  # @param headers [Boolean, nil] Whether default emulation headers are enabled;
  #   defaults to true when omitted or nil
  # @return [Wreq::Emulation] Configured emulation settings
  # @raise [ArgumentError] if an option is unknown or extra arguments are given
  # @raise [TypeError] if the option argument is not a Hash or a value has the
  #   wrong Ruby type
  class Emulation
    # Native fields and methods are set by the extension.
    # This stub is for documentation only.
    unless singleton_methods(false).include?(:new)
      # @param profile [Wreq::Profile, nil] Fingerprint profile to emulate
      # @param platform [Wreq::Platform, nil] Operating system platform to emulate
      # @param http2 [Boolean, nil] Whether HTTP/2 emulation is enabled; defaults
      #   to true when omitted or nil
      # @param headers [Boolean, nil] Whether default emulation headers are enabled;
      #   defaults to true when omitted or nil
      # @return [Wreq::Emulation] Configured emulation settings
      # @raise [ArgumentError] if an option is unknown or extra arguments are given
      # @raise [TypeError] if an option or value has the wrong Ruby type
      def self.new(**options)
      end
    end
  end
end

# standard:enable Naming/ConstantName
