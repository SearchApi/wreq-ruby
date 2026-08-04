# frozen_string_literal: true

require "test_helper"

class EmulationTest < Minitest::Test
  def test_all_emulation_device_constants_are_non_nil
    Wreq::Profile.constants.each do |name|
      const = Wreq::Profile.const_get(name)
      assert_instance_of Wreq::Profile, const,
        "#{name} should be Profile, got #{const.inspect}"
    end
  end

  def test_chrome151_profile_is_available
    profile = Wreq::Profile::Chrome151

    assert_equal "Chrome151", profile.to_s
    assert_instance_of Wreq::Emulation, Wreq::Emulation.new(profile: profile)
  end

  def test_all_emulation_os_constants_are_non_nil
    Wreq::Platform.constants.each do |name|
      const = Wreq::Platform.const_get(name)
      assert_instance_of Wreq::Platform, const,
        "#{name} should be Platform, got #{const.inspect}"
    end
  end
end
