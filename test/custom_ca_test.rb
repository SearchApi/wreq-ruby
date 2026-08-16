# frozen_string_literal: true

require "test_helper"
require "pathname"
require_relative "support/ca_tls_server"

class CustomCaTest < Minitest::Test
  SKIP_LOCAL_TLS = Gem.win_platform?

  def setup
    return if SKIP_LOCAL_TLS
    CaTlsServer.start_server!
  end

  # =================================================================
  # Replace semantics: ca_file / ca_pem
  # =================================================================

  def test_ca_pem_trusts_server_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    client = Wreq::Client.new(ca_pem: CaTlsServer::CA_PEM, timeout: 3)
    resp = client.get(CaTlsServer.server_url)
    assert_equal 200, resp.code
  end

  def test_ca_file_trusts_server_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      resp = client.get(CaTlsServer.server_url)
      assert_equal 200, resp.code
    end
  end

  def test_ca_file_rejects_server_not_signed_by_that_ca
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::OTHER_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      assert_raises(Wreq::ConnectionError, Wreq::TimeoutError) { client.get(CaTlsServer.server_url) }
    end
  end

  def test_ca_pem_replaces_system_roots
    client = Wreq::Client.new(ca_pem: CaTlsServer::CA_PEM, timeout: 3)
    assert_raises(Wreq::ConnectionError) do
      client.get("https://www.google.com")
    end
  end

  def test_ca_file_does_not_set_verify_false
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::OTHER_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      assert_raises(Wreq::ConnectionError, Wreq::TimeoutError) { client.get(CaTlsServer.server_url) }
    end
  end

  # =================================================================
  # Augment semantics: additional_ca_file / additional_ca_pem
  # =================================================================

  def test_additional_ca_pem_trusts_custom_ca_and_system_roots
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    client = Wreq::Client.new(additional_ca_pem: CaTlsServer::CA_PEM, timeout: 3)
    resp = client.get(CaTlsServer.server_url)
    assert_equal 200, resp.code

    resp = client.get("https://www.google.com")
    assert_equal 200, resp.code
  end

  def test_additional_ca_file_trusts_custom_ca_and_system_roots
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(additional_ca_file: path, timeout: 3)
      resp = client.get(CaTlsServer.server_url)
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
    client = Wreq::Client.new(ca_pem: CaTlsServer::BUNDLE_PEM, timeout: 3)
    resp = client.get(CaTlsServer.server_url)
    assert_equal 200, resp.code
  end

  def test_ca_file_accepts_bundled_certificates
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::BUNDLE_PEM) do |path|
      client = Wreq::Client.new(ca_file: path, timeout: 3)
      resp = client.get(CaTlsServer.server_url)
      assert_equal 200, resp.code
    end
  end

  # =================================================================
  # Path-like objects (to_path protocol)
  # =================================================================

  def test_ca_file_accepts_pathname
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: Pathname.new(path), timeout: 3)
      resp = client.get(CaTlsServer.server_url)
      assert_equal 200, resp.code
    end
  end

  def test_additional_ca_file_accepts_pathname
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(additional_ca_file: Pathname.new(path), timeout: 3)
      resp = client.get(CaTlsServer.server_url)
      assert_equal 200, resp.code
    end
  end

  def test_ca_file_accepts_custom_to_path_object
    skip "Local TLS server not supported on Windows" if SKIP_LOCAL_TLS
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      path_like = Object.new
      path_like.define_singleton_method(:to_path) { path }

      client = Wreq::Client.new(ca_file: path_like, timeout: 3)
      resp = client.get(CaTlsServer.server_url)
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

  def test_malformed_base64_pem_raises_tls_error
    bad_pem = "-----BEGIN CERTIFICATE-----\nthis-is-not-valid-base64!!!\n-----END CERTIFICATE-----\n"
    assert_raises(Wreq::TlsError) do
      Wreq::Client.new(ca_pem: bad_pem)
    end
  end

  def test_malformed_ca_file_raises_tls_error
    bad_pem = "-----BEGIN CERTIFICATE-----\nthis-is-not-valid-base64!!!\n-----END CERTIFICATE-----\n"
    CaTlsServer.with_pem_file(bad_pem) do |path|
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
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(verify: false, ca_file: path)
      assert_instance_of Wreq::Client, client
    end
  end

  # =================================================================
  # Inspect does not leak CA configuration
  # =================================================================

  def test_inspect_does_not_leak_ca_file_path
    CaTlsServer.with_pem_file(CaTlsServer::CA_PEM) do |path|
      client = Wreq::Client.new(ca_file: path)
      refute_includes client.inspect, path
      refute_includes client.inspect, "BEGIN CERTIFICATE"
    end
  end

  def test_inspect_does_not_leak_ca_pem_content
    client = Wreq::Client.new(ca_pem: CaTlsServer::CA_PEM)
    refute_includes client.inspect, "BEGIN CERTIFICATE"
    assert_equal "#<Wreq::Client>", client.inspect
  end
end