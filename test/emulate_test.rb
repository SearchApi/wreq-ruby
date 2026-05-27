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

  def test_all_emulation_os_constants_are_non_nil
    Wreq::Platform.constants.each do |name|
      const = Wreq::Platform.const_get(name)
      assert_instance_of Wreq::Platform, const,
        "#{name} should be Platform, got #{const.inspect}"
    end
  end

  def test_chrome_parser
    profiles = ["test/results/chrome_147.json", "test/results/chrome_148.json"]

    profiles.each do |profile|
      str = File.read(profile)
      json = JSON.parse(str)
      emulation = Wreq::Emulation.parse(JSON.dump(json), permute_extensions: true)
      client = Wreq::Client.new(emulation: emulation)
      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(no psk)
      assert_includes resp.bytes, "t13d1516h2_8daaf6152771_d8a2da3f94cd"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"

      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(psk)
      assert_includes resp.bytes, "t13d1517h2_8daaf6152771_b6f405a00624"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"
    end
  end

  def test_edge_parser
    profiles = ["test/results/edge_148.json"]

    profiles.each do |profile|
      str = File.read(profile)
      json = JSON.parse(str)
      emulation = Wreq::Emulation.parse(JSON.dump(json), permute_extensions: true)
      client = Wreq::Client.new(emulation: emulation)
      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(no psk)
      assert_includes resp.bytes, "t13d1516h2_8daaf6152771_d8a2da3f94cd"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"

      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(psk)
      assert_includes resp.bytes, "t13d1517h2_8daaf6152771_b6f405a00624"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"
    end
  end

  def test_opera_parser
    profiles = ["test/results/opera_131.json"]

    profiles.each do |profile|
      str = File.read(profile)
      json = JSON.parse(str)
      emulation = Wreq::Emulation.parse(JSON.dump(json), permute_extensions: true)
      client = Wreq::Client.new(emulation: emulation)
      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(no psk)
      assert_includes resp.bytes, "t13d1516h2_8daaf6152771_d8a2da3f94cd"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"

      resp = client.get("https://tls.peet.ws/api/all")
      # ja4(psk)
      assert_includes resp.bytes, "t13d1517h2_8daaf6152771_b6f405a00624"
      # akamai
      assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"
    end
  end
end
