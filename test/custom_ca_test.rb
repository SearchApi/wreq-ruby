# frozen_string_literal: true

require "test_helper"
require "pathname"
require "openssl"
require "socket"
require "tempfile"

class CustomCaTest < Minitest::Test
  SKIP_LOCAL_TLS = Gem.win_platform?

  # Shared test CA and server certificate, generated once per suite.
  CA_KEY  = OpenSSL::PKey::RSA.new(2048)
  CA_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 1
    cert.subject    = OpenSSL::X509::Name.parse("/CN=Test CA")
    cert.issuer     = cert.subject
    cert.public_key = CA_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600

    ef = OpenSSL::X509::ExtensionFactory.new
    ef.subject_certificate = cert
    ef.issuer_certificate  = cert
    cert.add_extension(ef.create_extension("basicConstraints", "CA:TRUE", true))
    cert.add_extension(ef.create_extension("subjectKeyIdentifier", "hash"))

    cert.sign(CA_KEY, OpenSSL::Digest::SHA256.new)
  end

  SERVER_KEY  = OpenSSL::PKey::RSA.new(2048)
  SERVER_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 2
    cert.subject    = OpenSSL::X509::Name.parse("/CN=localhost")
    cert.issuer     = CA_CERT.subject
    cert.public_key = SERVER_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600

    ef = OpenSSL::X509::ExtensionFactory.new
    ef.subject_certificate = cert
    ef.issuer_certificate  = CA_CERT
    cert.add_extension(ef.create_extension("subjectAltName", "DNS:localhost,IP:127.0.0.1"))

    cert.sign(CA_KEY, OpenSSL::Digest::SHA256.new)
  end

  CA_PEM = CA_CERT.to_pem

  # A second unrelated CA that did NOT sign the server cert.
  OTHER_KEY  = OpenSSL::PKey::RSA.new(2048)
  OTHER_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 3
    cert.subject    = OpenSSL::X509::Name.parse("/CN=Other CA")
    cert.issuer     = cert.subject
    cert.public_key = OTHER_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600
    cert.sign(OTHER_KEY, OpenSSL::Digest::SHA256.new)
  end

  OTHER_PEM = OTHER_CERT.to_pem

  # Bundle containing both CAs.
  BUNDLE_PEM = CA_PEM + OTHER_PEM

  @server_started = false

  def self.start_server
    return if @server_started || SKIP_LOCAL_TLS
    @server_started = true

    ctx = OpenSSL::SSL::SSLContext.new
    ctx.cert = SERVER_CERT
    ctx.key  = SERVER_KEY

    tcp = TCPServer.new("127.0.0.1", 0)
    @port = tcp.addr[1]
    @ssl_server = OpenSSL::SSL::SSLServer.new(tcp, ctx)
    @server_url = "https://localhost:#{@port}"

    @thread = Thread.new do
      loop do
        begin
          client = @ssl_server.accept
        rescue OpenSSL::SSL::SSLError
          next
        rescue IOError
          break
        end
        Thread.new(client) do |c|
          begin
            c.gets
            c.print "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
          rescue
          ensure
            c.close rescue nil
          end
        end
      end
    end
    @thread.abort_on_exception = true

    Minitest.after_run do
      @ssl_server&.close
      @thread&.kill
      @thread&.join(2)
    end
  end

  def self.server_url
    @server_url
  end

  def setup
    self.class.start_server
  end

  def server_url
    self.class.server_url
  end

  # ---- Helper: write PEM to a tempfile and return its path ----

  def with_pem_file(content)
    file = Tempfile.new(["ca", ".pem"])
    file.write(content)
    file.flush
    yield file.path
  ensure
    file&.close!
  end

  # =================================================================
  # Replace semantics: ca_file / ca_pem
  # =================================================================

  def test_ca_pem_trusts_server_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    client = Wreq::Client.new(ca_pem: CA_PEM, timeout: 3)
    resp = client.get(server_url)
    assert_equal 200, resp.code
  end

  def test_ca_file_trusts_server_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code
    end
  end

  def test_ca_file_rejects_server_not_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(OTHER_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      assert_raises(Wreq::ConnectionError, Wreq::TimeoutError) { client.get(server_url) }
    end
  end

  def test_ca_pem_replaces_system_roots
    client = Wreq::Client.new(ca_pem: CA_PEM, timeout: 3)
    assert_raises(Wreq::ConnectionError) do
      client.get("https://www.google.com")
    end
  end

  def test_ca_file_does_not_set_verify_false
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(OTHER_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      assert_raises(Wreq::ConnectionError, Wreq::TimeoutError) { client.get(server_url) }
    end
  end

  # =================================================================
  # Augment semantics: additional_ca_file / additional_ca_pem
  # =================================================================

  def test_additional_ca_pem_trusts_custom_ca_and_system_roots
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    client = Wreq::Client.new(additional_ca_pem: CA_PEM, timeout: 3)
    resp = client.get(server_url)
    assert_equal 200, resp.code

    resp = client.get("https://www.google.com")
    assert_equal 200, resp.code
  end

  def test_additional_ca_file_trusts_custom_ca_and_system_roots
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(additional_ca_file: path, timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code

      resp = client.get("https://www.google.com")
      assert_equal 200, resp.code
    end
  end

  # =================================================================
  # Bundled PEM (multiple certificates in one file/string)
  # =================================================================

  def test_ca_pem_accepts_bundled_certificates
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    client = Wreq::Client.new(ca_pem: BUNDLE_PEM, timeout: 3)
    resp = client.get(server_url)
    assert_equal 200, resp.code
  end

  def test_ca_file_accepts_bundled_certificates
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(BUNDLE_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code
    end
  end

  # =================================================================
  # Path-like objects (to_path protocol)
  # =================================================================

  def test_ca_file_accepts_pathname
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: Pathname.new(path), timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code
    end
  end

  def test_additional_ca_file_accepts_pathname
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(additional_ca_file: Pathname.new(path), timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code
    end
  end

  def test_ca_file_accepts_custom_to_path_object
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    with_pem_file(CA_PEM) do |path|
      path_like = Object.new
      path_like.define_singleton_method(:to_path) { path }

      client = Wreq::Client.new(ca_file: path_like, timeout: 3)
      resp = client.get(server_url)
      assert_equal 200, resp.code
    end
  end

  # =================================================================
  # Invalid inputs fail during construction
  # =================================================================

  def test_missing_ca_file_raises_argument_error
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(ca_file: "/nonexistent/ca.pem")
    end
    assert_includes error.message, "ca_file"
    assert_includes error.message, "cannot read"
    refute_includes error.message, "BEGIN CERTIFICATE"
  end

  def test_missing_additional_ca_file_raises_argument_error
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(additional_ca_file: "/nonexistent/extra.pem")
    end
    assert_includes error.message, "additional_ca_file"
  end

  def test_empty_ca_pem_raises_argument_error
    assert_raises(ArgumentError) do
      Wreq::Client.new(ca_pem: "")
    end
  end

  def test_garbage_ca_pem_raises_argument_error
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(ca_pem: "not valid pem")
    end
    assert_includes error.message, "does not contain any certificates"
    refute_includes error.message, "not valid pem"
  end

  def test_malformed_base64_pem_raises_tls_error
    bad_pem = "-----BEGIN CERTIFICATE-----\nthis-is-not-valid-base64!!!\n-----END CERTIFICATE-----\n"
    assert_raises(Wreq::TlsError) do
      Wreq::Client.new(ca_pem: bad_pem)
    end
  end

  def test_malformed_ca_file_raises_tls_error
    bad_pem = "-----BEGIN CERTIFICATE-----\nthis-is-not-valid-base64!!!\n-----END CERTIFICATE-----\n"
    with_pem_file(bad_pem) do |path|
      assert_raises(Wreq::TlsError) do
        Wreq::Client.new(ca_file: path)
      end
    end
  end

  # =================================================================
  # Mutual exclusion
  # =================================================================

  def test_ca_file_and_ca_pem_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(ca_file: "/a", ca_pem: "b")
    end
    assert_includes error.message, ":ca_file"
    assert_includes error.message, ":ca_pem"
  end

  def test_ca_file_and_additional_ca_pem_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(ca_file: "/a", additional_ca_pem: "b")
    end
    assert_includes error.message, ":ca_file"
    assert_includes error.message, ":additional_ca_pem"
  end

  def test_all_four_ca_options_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(ca_file: "/a", ca_pem: "b", additional_ca_pem: "c")
    end
    assert_includes error.message, ":ca_file"
    assert_includes error.message, ":ca_pem"
    assert_includes error.message, ":additional_ca_pem"
  end

  # =================================================================
  # nil values are treated as absent
  # =================================================================

  def test_nil_ca_options_are_ignored
    client = Wreq::Client.new(ca_pem: nil, ca_file: nil, timeout: 3)
    assert_instance_of Wreq::Client, client
  end

  # =================================================================
  # verify: false with CA options
  # =================================================================

  def test_verify_false_with_valid_ca_still_builds
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(verify: false, ca_file: path)
      assert_instance_of Wreq::Client, client
    end
  end

  def test_verify_false_does_not_skip_pem_validation
    assert_raises(ArgumentError) do
      Wreq::Client.new(verify: false, ca_pem: "not valid pem")
    end
  end

  # =================================================================
  # Inspect does not leak CA configuration
  # =================================================================

  def test_inspect_does_not_leak_ca_file_path
    with_pem_file(CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: path)
      refute_includes client.inspect, path
      refute_includes client.inspect, "BEGIN CERTIFICATE"
    end
  end

  def test_inspect_does_not_leak_ca_pem_content
    client = Wreq::Client.new(ca_pem: CA_PEM)
    refute_includes client.inspect, "BEGIN CERTIFICATE"
    assert_equal "#<Wreq::Client>", client.inspect
  end
end