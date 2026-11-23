# frozen_string_literal: true

require "test_helper"
require "socket"
require "stringio"
require "timeout"

class EmulationTest < Minitest::Test
  TRUST_ANCHOR_IDS = 51_764

  def test_reused_emulation_retains_trust_anchors_across_clients_and_requests
    [Wreq::Profile::Chrome152, Wreq::Profile::Chrome153, Wreq::Profile::Chrome154, Wreq::Profile::Chrome155, Wreq::Profile::Chrome156, Wreq::Profile::Chrome157].each do |profile|
      emulation = Wreq::Emulation.new(profile: profile)
      client = Wreq::Client.new(emulation: emulation, no_proxy: true)
      _, extensions = capture_client_hello { |url| client.get(url, timeout: 5) }
      expected = extensions.fetch(TRUST_ANCHOR_IDS)
      refute_empty expected, profile.to_s

      request_client = Wreq::Client.new(no_proxy: true)
      2.times do
        _, actual_extensions = capture_client_hello do |url|
          request_client.get(url, emulation: emulation, timeout: 5)
        end
        assert_equal expected, actual_extensions.fetch(TRUST_ANCHOR_IDS), "#{profile}: reusing an object must retain its configuration"
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
      Wreq::Profile::Chrome155 => "Chrome155",
      Wreq::Profile::Chrome156 => "Chrome156",
      Wreq::Profile::Chrome157 => "Chrome157",
      Wreq::Profile::Chrome158 => "Chrome158",
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

  def test_all_firefox_profiles_are_available
    versions = [109, 117, 128, 133, 135, 136, 139, *(142..160)]
    names = versions.map { |version| "Firefox#{version}" } + %w[FirefoxPrivate135 FirefoxAndroid135 FirefoxPrivate136]
    names.each do |name|
      assert Wreq::Profile.const_defined?(name, false), "missing #{name}"
      profile = Wreq::Profile.const_get(name, false)
      assert_equal name, profile.to_s
      assert_instance_of Wreq::Emulation, Wreq::Emulation.new(profile: profile)
    end
  end

  def test_firefox_client_hello_matches_version
    suites_153 = [
      0x1301, 0x1303, 0x1302,
      0xc02b, 0xc02f, 0xcca9, 0xcca8, 0xc02c, 0xc030, 0xc00a, 0xc013, 0xc014, 0x009c, 0x009d, 0x002f, 0x0035
    ]
    # Firefox 154 disabled TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA.
    suites_154 = suites_153.reject { |suite| suite == 0xc00a }

    [
      [Wreq::Profile::Firefox153, suites_153, 15, [4588, 29, 23, 24, 25, 256, 257]],
      [Wreq::Profile::Firefox154, suites_154, 15, [4588, 29, 23, 24, 25, 256, 257]],
      [Wreq::Profile::Firefox155, suites_154, 16, [4588, 29, 23, 24, 25, 256, 257]],
      [Wreq::Profile::Firefox156, suites_154, 16, [4588, 29, 23, 24, 25]]
    ].each do |profile, expected_suites, padding_remainder, groups|
      client = Wreq::Client.new(emulation: Wreq::Emulation.new(profile: profile), no_proxy: true)
      suites, extensions = capture_client_hello { |url| client.get(url, timeout: 5) }
      assert_equal [4588, 29, 23], key_share_groups(extensions.fetch(51)), "#{profile}: key share groups"
      assert_equal expected_suites, suites, "#{profile}: cipher suites"

      ech = StringIO.new(extensions.fetch(65_037))
      read_tls_bytes(ech, 6) # Outer tag, HPKE suite and config ID.
      read_tls_bytes(ech, read_tls_bytes(ech, 2).unpack1("n")) # Encapsulated key.
      payload_size = read_tls_bytes(ech, 2).unpack1("n")
      assert_equal padding_remainder, payload_size % 32, "#{profile}: ECH GREASE ciphertext"
      assert_equal groups, extensions.fetch(10).unpack("n*").drop(1), "#{profile}: supported groups"
    end
  end

  private

  # Each listener forces a new connection. Stop after ClientHello so this test
  # inspects the wire bytes without depending on certificates or an external service.
  def capture_client_hello
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
        parse_client_hello(handshake)
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

  def parse_client_hello(handshake)
    io = StringIO.new(handshake)
    raise "expected ClientHello" unless read_tls_bytes(io, 1).unpack1("C") == 1
    read_tls_bytes(io, 3 + 2 + 32) # Handshake length, legacy version and random.
    read_tls_bytes(io, read_tls_bytes(io, 1).unpack1("C")) # Session ID.
    suites = read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")).unpack("n*")
    read_tls_bytes(io, read_tls_bytes(io, 1).unpack1("C")) # Compression methods.
    extensions = StringIO.new(read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")))
    parsed = {}
    until extensions.eof?
      type, length = read_tls_bytes(extensions, 4).unpack("nn")
      parsed[type] = read_tls_bytes(extensions, length)
    end
    [suites, parsed]
  end

  def key_share_groups(extension)
    io = StringIO.new(extension)
    entries = StringIO.new(read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")))
    groups = []
    until entries.eof?
      group, length = read_tls_bytes(entries, 4).unpack("nn")
      read_tls_bytes(entries, length) # Key exchange bytes.
      groups << group
    end
    groups
  end
end
