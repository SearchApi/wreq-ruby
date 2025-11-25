# frozen_string_literal: true

module Wreq
  # Device emulation enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::EmulationDevice}.
  #
  # @example Using predefined constants
  #   device = Wreq::EmulationDevice::Chrome117
  #   device.class #=> Wreq::EmulationDevice
  class EmulationDevice
    # Constants are set by the native extension at initialization.
    # These stubs are for documentation only.
    unless const_defined?(:Chrome117)
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
      Edge101 = nil
      Edge122 = nil
      Edge127 = nil
      Edge131 = nil
      Edge134 = nil
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
      Safari18 = nil
      SafariIPad18 = nil
      Safari18_2 = nil
      Safari18_3 = nil
      Safari18_3_1 = nil
      SafariIos18_1_1 = nil
      Safari18_5 = nil
      Safari26 = nil
      SafariIos26 = nil
      SafariIPad26 = nil
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
    end

    unless method_defined?(:to_s)
      # Returns a string representation of the emulation device.
      # @return [String] Emulation device as string
      def to_s; end
    end
  end

  # Operating system emulation enumeration backed by Rust.
  #
  # Variants are exposed as constants under this class.
  # Each constant is an instance of {Wreq::EmulationOS}.
  #
  # @example Using predefined constants
  #   os = Wreq::EmulationOS::Windows
  #   os.class #=> Wreq::EmulationOS
  class EmulationOS
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
      # Returns a string representation of the emulation OS.
      # @return [String] Emulation OS as string
      def to_s; end
    end
  end

  # Emulation option wrapper.
  #
  # This class wraps device and OS emulation options and provides
  # a unified interface for browser environment simulation.
  # The actual implementation is provided by Rust for performance.
  #
  # @example Create an emulation option
  #   emu = Wreq::Emulation.new(device: Wreq::EmulationDevice::Chrome117, os: Wreq::EmulationOS::Windows)
  #
  # @param device [Wreq::EmulationDevice] Device profile (optional)
  # @param os [Wreq::EmulationOS] Operating system profile (optional)
  # @param skip_http2 [Boolean] Whether to skip HTTP/2 (optional)
  # @param skip_headers [Boolean] Whether to skip default headers (optional)
  class Emulation
    # Native fields and methods are set by the extension.
    # This stub is for documentation only.
    unless method_defined?(:initialize)
      # @param device [Wreq::EmulationDevice] Device profile (optional)
      # @param os [Wreq::EmulationOS] Operating system profile (optional)
      # @param skip_http2 [Boolean] Whether to skip HTTP/2 (optional)
      # @param skip_headers [Boolean] Whether to skip default headers (optional)
      def new(device: nil, os: nil, skip_http2: false, skip_headers: false); end
    end
  end
end
