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
    emulation = Wreq::Emulation.parse(JSON.dump(json))
    resp = Wreq::get("https://tls.peet.ws/api/all", emulation: emulation)
    assert_includes resp.bytes, "52d84b11737d980aef856699f885ca86"
  end
end
