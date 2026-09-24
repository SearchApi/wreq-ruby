# frozen_string_literal: true

require "test_helper"
require "socket"
require "stringio"
require "timeout"

class EmulationTest < Minitest::Test
  def test_reused_emulation_retains_trust_anchors_across_clients_and_requests
    [Wreq::Profile::Chrome152, Wreq::Profile::Chrome153, Wreq::Profile::Chrome154].each do |profile|
      emulation = Wreq::Emulation.new(profile: profile)
      client = Wreq::Client.new(emulation: emulation, no_proxy: true)
      expected = capture_trust_anchors { |url| client.get(url, timeout: 5) }
      refute_empty expected, profile.to_s

      request_client = Wreq::Client.new(no_proxy: true)
      2.times do
        actual = capture_trust_anchors do |url|
          request_client.get(url, emulation: emulation, timeout: 5)
        end
        assert_equal expected, actual, "#{profile}: reusing an object must retain its configuration"
      end
    end
  end

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

  def test_new_browser_profiles_are_available
    {
      Wreq::Profile::Chrome152 => "Chrome152",
      Wreq::Profile::Chrome153 => "Chrome153",
      Wreq::Profile::Chrome154 => "Chrome154",
      Wreq::Profile::Firefox152 => "Firefox152"
    }.each do |profile, name|
      assert_equal name, profile.to_s
      assert_instance_of Wreq::Emulation, Wreq::Emulation.new(profile: profile)
    end
  end

  def test_all_emulation_os_constants_are_non_nil
    Wreq::Platform.constants.each do |name|
      const = Wreq::Platform.const_get(name)
      assert_instance_of Wreq::Platform, const,
        "#{name} should be Platform, got #{const.inspect}"
    end
  end

  private

  # Each listener forces a new connection. Stop after ClientHello so this test
  # inspects the extension bytes without depending on certificates or the network.
  def capture_trust_anchors
    server = TCPServer.new("127.0.0.1", 0)
    thread = Thread.new do
      socket = nil
      Timeout.timeout(5) do
        socket = server.accept
        handshake = +"".b
        loop do
          header = read_tls_bytes(socket, 5)
          raise "expected a TLS handshake record" unless header.getbyte(0) == 22
          handshake << read_tls_bytes(socket, header.byteslice(3, 2).unpack1("n"))
          next if handshake.bytesize < 4
          length = handshake.byteslice(1, 3).bytes.reduce(0) { |n, byte| (n << 8) | byte }
          break if handshake.bytesize >= length + 4
        end
        trust_anchors_from_client_hello(handshake)
      end
    ensure
      socket&.close
    end
    thread.report_on_exception = false

    assert_raises(Wreq::ConnectionError) { yield "https://127.0.0.1:#{server.addr[1]}/" }
    thread.value
  ensure
    server&.close
    thread&.kill if thread&.alive?
    thread&.join
  end

  def read_tls_bytes(io, length)
    bytes = io.read(length)
    raise EOFError, "truncated ClientHello" unless bytes&.bytesize == length
    bytes
  end

  def trust_anchors_from_client_hello(handshake)
    io = StringIO.new(handshake)
    raise "expected ClientHello" unless read_tls_bytes(io, 1).unpack1("C") == 1
    read_tls_bytes(io, 3 + 2 + 32) # Handshake length, legacy version and random.
    read_tls_bytes(io, read_tls_bytes(io, 1).unpack1("C")) # Session ID.
    read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")) # Cipher suites.
    read_tls_bytes(io, read_tls_bytes(io, 1).unpack1("C")) # Compression methods.
    extensions = StringIO.new(read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")))
    until extensions.eof?
      type, length = read_tls_bytes(extensions, 4).unpack("nn")
      data = read_tls_bytes(extensions, length)
      return data if type == 51764
    end
    raise "missing trust_anchors extension"
  end
end
