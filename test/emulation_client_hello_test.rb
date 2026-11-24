# frozen_string_literal: true

require "test_helper"
require "socket"
require "stringio"
require "timeout"

class EmulationClientHelloTest < Minitest::Test
  SUPPORTED_GROUPS = 10
  SIGNATURE_ALGORITHMS = 13
  KEY_SHARE = 51
  TRUST_ANCHOR_IDS = 51_764
  ENCRYPTED_CLIENT_HELLO = 65_037

  def test_reusing_chrome_emulation_preserves_trust_anchors
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

  def test_edge158_sends_signature_grease_without_trust_anchors
    client = Wreq::Client.new(emulation: Wreq::Emulation.new(profile: Wreq::Profile::Edge158), no_proxy: true)
    _, extensions = capture_client_hello { |url| client.get(url, timeout: 5) }

    _, grease, *signatures = extensions.fetch(SIGNATURE_ALGORITHMS).unpack("n*")
    assert_equal 0x0a0a, grease & 0x0f0f
    assert_equal grease >> 8, grease & 0xff
    assert_equal [0x0904, 0x0905, 0x0906], signatures.first(3) # ML-DSA
    refute extensions.key?(TRUST_ANCHOR_IDS)
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
      assert_equal [4588, 29, 23], key_share_groups(extensions.fetch(KEY_SHARE)), "#{profile}: key share groups"
      assert_equal expected_suites, suites, "#{profile}: cipher suites"

      assert_equal padding_remainder, ech_payload_size(extensions.fetch(ENCRYPTED_CLIENT_HELLO)) % 32, "#{profile}: ECH GREASE ciphertext"
      assert_equal groups, extensions.fetch(SUPPORTED_GROUPS).unpack("n*").drop(1), "#{profile}: supported groups"
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

  def ech_payload_size(extension)
    io = StringIO.new(extension)
    read_tls_bytes(io, 6) # Outer tag, HPKE suite and config ID.
    read_tls_bytes(io, read_tls_bytes(io, 2).unpack1("n")) # Encapsulated key.
    read_tls_bytes(io, 2).unpack1("n")
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
