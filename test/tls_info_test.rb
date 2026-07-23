# frozen_string_literal: true

require "test_helper"

class TlsInfoTest < Minitest::Test
  # ---- Opt-in behavior ----

  def test_tls_info_nil_when_not_enabled
    response = Wreq.get("#{HTTPBIN_URL}/get")
    assert_nil response.tls_info
  end

  def test_tls_info_nil_on_default_client
    client = Wreq::Client.new
    response = client.get("#{HTTPBIN_URL}/get")
    assert_nil response.tls_info
  end

  def test_tls_info_present_when_enabled
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    refute_nil response.tls_info
    assert_instance_of Wreq::TlsInfo, response.tls_info
  end

  # ---- Plain HTTP returns nil ----

  def test_tls_info_nil_for_plain_http
    client = Wreq::Client.new(tls_info: true)
    response = client.get("http://httpbin.io/get")
    assert_nil response.tls_info
  end

  # ---- Peer certificate ----

  def test_peer_certificate_is_binary_string
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    tls = response.tls_info

    cert = tls.peer_certificate
    refute_nil cert
    assert_instance_of String, cert
    assert_equal Encoding::BINARY, cert.encoding
    assert cert.bytesize > 0
  end

  # ---- Peer certificate chain ----

  def test_peer_certificate_chain_is_frozen_array
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    tls = response.tls_info

    chain = tls.peer_certificate_chain
    refute_nil chain
    assert_instance_of Array, chain
    assert chain.frozen?, "certificate chain array must be frozen"
    assert chain.length > 0
  end

  def test_peer_certificate_chain_contains_binary_strings
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    chain = response.tls_info.peer_certificate_chain

    chain.each do |cert|
      assert_instance_of String, cert
      assert_equal Encoding::BINARY, cert.encoding
      assert cert.bytesize > 0
    end
  end

  def test_peer_certificate_chain_immutable
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    chain = response.tls_info.peer_certificate_chain

    assert_raises(FrozenError) { chain.push("test") }
  end

  # ---- Data survives body consumption ----

  def test_tls_info_available_after_body_read
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")

    _body = response.text
    tls = response.tls_info

    refute_nil tls
    refute_nil tls.peer_certificate
    assert tls.peer_certificate.bytesize > 0
  end

  def test_tls_info_available_after_close
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")

    response.close
    tls = response.tls_info

    refute_nil tls
    refute_nil tls.peer_certificate
  end

  # ---- Inspect does not leak certificate bytes ----

  def test_inspect_shows_byte_counts_only
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    tls = response.tls_info

    inspection = tls.inspect
    assert_match(/peer_certificate=\(\d+ bytes\)/, inspection)
    assert_match(/peer_certificate_chain=\(\d+ certs\)/, inspection)
    assert_match(/\A#<Wreq::TlsInfo /, inspection)
  end

  def test_to_s_matches_inspect
    client = Wreq::Client.new(tls_info: true)
    response = client.get("#{HTTPBIN_URL}/get")
    tls = response.tls_info

    assert_equal tls.inspect, tls.to_s
  end

  # ---- Connection reuse ----

  def test_tls_info_on_reused_connection
    client = Wreq::Client.new(tls_info: true)

    resp1 = client.get("#{HTTPBIN_URL}/get")
    resp2 = client.get("#{HTTPBIN_URL}/get")

    tls1 = resp1.tls_info
    tls2 = resp2.tls_info

    refute_nil tls1
    refute_nil tls2
    assert tls1.peer_certificate.bytesize > 0
    assert tls2.peer_certificate.bytesize > 0
  end
end