# frozen_string_literal: true

require "test_helper"
require_relative "support/tls_server"

class TlsInfoTest < Minitest::Test
  HTTPBIN_HTTP_URL = ENV.fetch("HTTPBIN_HTTP_URL", HTTPBIN_URL.sub(/\Ahttps:/, "http:"))

  def test_tls_info_is_nil_when_disabled_or_request_is_plain_http
    default_response = Wreq::Client.new.get("#{HTTPBIN_URL}/get")
    plain_response = Wreq::Client.new(tls_info: true).get("#{HTTPBIN_HTTP_URL}/get")

    assert_nil default_response.tls_info
    assert_nil plain_response.tls_info
  end

  def test_certificate_data_survives_body_lifecycle_on_a_reused_connection
    fixture = TlsTestServer.with_connection(request_count: 2) do |base_url, certificate_der|
      client = Wreq::Client.new(
        tls_info: true,
        verify: false,
        http1_only: true,
        no_proxy: true,
        timeout: 5
      )

      read_response = client.get("#{base_url}read")
      assert_equal "ok", read_response.text
      read_tls = read_response.tls_info

      closed_response = client.get("#{base_url}close")
      closed_response.close
      closed_tls = closed_response.tls_info

      assert_instance_of Wreq::TlsInfo, read_tls
      certificate = read_tls.peer_certificate
      chain = read_tls.peer_certificate_chain
      assert_equal certificate_der, certificate
      assert_equal Encoding::BINARY, certificate.encoding
      assert_equal [certificate_der], chain
      assert_equal Encoding::BINARY, chain.first.encoding
      assert_predicate chain, :frozen?
      assert_equal(
        "#<Wreq::TlsInfo peer_certificate=#{certificate_der.bytesize}B peer_certificate_chain=1>",
        read_tls.inspect
      )
      assert_empty Wreq::TlsInfo.instance_methods(false) & %i[to_h to_s]

      certificate.clear
      assert_equal certificate_der, read_tls.peer_certificate
      assert_equal certificate_der, closed_tls.peer_certificate
    end

    assert_equal(
      {connections: 1, requests: ["GET /read HTTP/1.1\r\n", "GET /close HTTP/1.1\r\n"]},
      fixture
    )
  end
end
