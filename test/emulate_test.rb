# frozen_string_literal: true

require "test_helper"

class EmulationTest < Minitest::Test
  def test_all_emulation_device_constants_are_non_nil
    Wreq::EmulationDevice.constants.each do |name|
      const = Wreq::EmulationDevice.const_get(name)
      assert_instance_of Wreq::EmulationDevice, const,
        "#{name} should be EmulationDevice, got #{const.inspect}"
    end
  end

  def test_all_emulation_os_constants_are_non_nil
    Wreq::EmulationOS.constants.each do |name|
      const = Wreq::EmulationOS.const_get(name)
      assert_instance_of Wreq::EmulationOS, const,
        "#{name} should be EmulationOS, got #{const.inspect}"
    end
  end

  def test_http2_parser
    str = File.read("test/results/chrome_147.json")
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
