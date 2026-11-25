# frozen_string_literal: true

require "test_helper"

class EmulationTest < Minitest::Test
  def test_expected_browser_profiles_are_registered
    versions_by_browser = {
      "Chrome" => 151..158,
      "Edge" => [101, 122, 127, 131, *(134..158)],
      "Firefox" => [109, 117, 128, 133, 135, 136, 139, *(142..160)],
      "Opera" => 116..140
    }
    expected = versions_by_browser.flat_map do |browser, versions|
      versions.map { |version| "#{browser}#{version}" }
    end
    expected.concat(%w[FirefoxPrivate135 FirefoxAndroid135 FirefoxPrivate136])

    registered = Wreq::Profile.constants(false).map(&:to_s)
    assert_empty expected - registered, "missing browser profiles"
  end

  def test_registered_profiles_have_matching_names_and_build_emulation
    Wreq::Profile.constants(false).each do |name|
      profile = Wreq::Profile.const_get(name, false)
      assert_instance_of Wreq::Profile, profile, name.to_s
      assert_equal name.to_s, profile.to_s
      assert_instance_of Wreq::Emulation, Wreq::Emulation.new(profile: profile), name.to_s
    end
  end

  def test_registered_platforms_are_platform_values
    Wreq::Platform.constants(false).each do |name|
      assert_instance_of Wreq::Platform, Wreq::Platform.const_get(name, false), name.to_s
    end
  end
end
