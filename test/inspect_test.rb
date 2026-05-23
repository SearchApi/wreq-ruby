# frozen_string_literal: true

require "test_helper"

class InspectTest < Minitest::Test
  # ---- Headers ----

  def test_headers_inspect_empty
    headers = Wreq::Headers.new
    assert_equal "#<Wreq::Headers [0 headers]>", headers.inspect
  end

  def test_headers_inspect_with_entries
    headers = Wreq::Headers.new
    headers.set("Content-Type", "text/html")
    headers.set("Accept", "application/json")
    assert_equal "#<Wreq::Headers [2 headers]>", headers.inspect
  end

  # ---- Cookie ----

  def test_cookie_inspect_minimal
    c = Wreq::Cookie.new("sid", "secret123")
    result = c.inspect
    assert_includes result, "#<Wreq::Cookie"
    assert_includes result, "sid"
    refute_includes result, "secret123"
    assert result.end_with?(">")
  end

  def test_cookie_inspect_with_domain_and_path
    c = Wreq::Cookie.new("sid", "val",
      domain: "example.com",
      path: "/app")
    result = c.inspect
    assert_includes result, "domain=example.com"
    assert_includes result, "path=/app"
  end

  def test_cookie_inspect_with_flags
    c = Wreq::Cookie.new("sid", "val",
      secure: true,
      http_only: true)
    result = c.inspect
    assert_includes result, "secure"
    assert_includes result, "http_only"
  end

  def test_cookie_inspect_omits_nil_attributes
    c = Wreq::Cookie.new("sid", "val")
    result = c.inspect
    refute_includes result, "domain="
    refute_includes result, "path="
    refute_includes result, "secure"
    refute_includes result, "http_only"
  end

  # ---- Jar ----

  def test_jar_inspect_empty
    jar = Wreq::Jar.new
    assert_equal "#<Wreq::Jar [0 cookies]>", jar.inspect
  end

  def test_jar_inspect_with_cookies
    jar = Wreq::Jar.new
    jar.add_cookie_str("a=1; Path=/", "https://example.com")
    jar.add_cookie_str("b=2; Path=/", "https://example.com")
    assert_equal "#<Wreq::Jar [2 cookies]>", jar.inspect
  end

  # ---- Client ----

  def test_client_inspect
    client = Wreq::Client.new
    assert_equal "#<Wreq::Client>", client.inspect
  end

  def test_client_inspect_with_options
    client = Wreq::Client.new(timeout: 30, gzip: true)
    assert_equal "#<Wreq::Client>", client.inspect
  end

  # ---- Response ----

  def test_response_to_s_returns_body
    response = Wreq.get("http://localhost:8080/json")
    assert_equal response.text, response.to_s
  end

  def test_response_inspect_format
    response = Wreq.get("http://localhost:8080/json")
    result = response.inspect
    assert result.start_with?("#<Wreq::Response")
    assert_includes result, "200"
    assert result.end_with?(">")
  end

  # ---- StatusCode ----

  def test_status_code_inspect
    response = Wreq.get("http://localhost:8080/status/200")
    result = response.status.inspect
    assert result.start_with?("#<Wreq::StatusCode")
    assert_includes result, response.status.to_s
    assert result.end_with?(">")
  end

  # ---- Version ----

  def test_version_inspect_from_constant
    v = Wreq::Version::HTTP_11
    result = v.inspect
    assert result.start_with?("#<Wreq::Version")
    assert_includes result, v.to_s
    assert result.end_with?(">")
  end

  def test_version_inspect_from_response
    response = Wreq.get("http://localhost:8080/get")
    result = response.version.inspect
    assert result.start_with?("#<Wreq::Version")
    assert result.end_with?(">")
  end
end
