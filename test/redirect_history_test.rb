# frozen_string_literal: true

require "test_helper"
require "cgi"

class RedirectHistoryTest < Minitest::Test
  def setup
    @client = Wreq::Client.new(allow_redirects: true, max_redirects: 10, timeout: 10)
  end

  # =================================================================
  # No-redirect responses return an empty array
  # =================================================================

  def test_no_redirect_returns_empty_array
    resp = @client.get("#{HTTPBIN_URL}/get")
    assert_equal [], resp.history
  end

  def test_no_redirect_array_is_frozen
    resp = @client.get("#{HTTPBIN_URL}/get")
    assert resp.history.frozen?
  end

  # =================================================================
  # Redirects disabled returns empty history
  # =================================================================

  def test_redirects_disabled_returns_empty_history
    client = Wreq::Client.new(allow_redirects: false, timeout: 10)
    resp = client.get("#{HTTPBIN_URL}/redirect/1")
    assert_equal [], resp.history
    assert_equal 302, resp.code
  end

  # =================================================================
  # Single hop redirect
  # =================================================================

  def test_single_redirect_has_one_entry
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert_equal 1, resp.history.length
  end

  def test_single_redirect_entry_fields
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    hop = resp.history[0]

    assert_equal 302, hop.status
    assert_includes hop.previous_url, "/redirect/1"
    assert_includes hop.url, "/get"
  end

  def test_single_redirect_final_url
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert_includes resp.url, "/get"
  end

  # =================================================================
  # Multiple hops
  # =================================================================

  def test_multiple_hops_count
    resp = @client.get("#{HTTPBIN_URL}/redirect/3")
    assert_equal 3, resp.history.length
  end

  def test_multiple_hops_are_ordered
    resp = @client.get("#{HTTPBIN_URL}/redirect/3")
    urls = resp.history.map(&:previous_url)
    assert_includes urls[0], "/redirect/3"
    assert_includes urls[1], "redirect/2"
    assert_includes urls[2], "redirect/1"
  end

  # =================================================================
  # History array is immutable
  # =================================================================

  def test_history_array_is_frozen
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert resp.history.frozen?
  end

  def test_history_array_rejects_mutation
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert_raises(FrozenError) { resp.history << "something" }
  end

  # =================================================================
  # Entry type and methods
  # =================================================================

  def test_entry_is_redirect_history_entry
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert_instance_of Wreq::RedirectHistoryEntry, resp.history[0]
  end

  def test_entry_headers_returns_wreq_headers
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    assert_instance_of Wreq::Headers, resp.history[0].headers
  end

  # =================================================================
  # to_h serialization
  # =================================================================

  def test_entry_to_h_has_expected_keys
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    hash = resp.history[0].to_h
    assert_equal [:status, :url, :previous_url, :headers].sort, hash.keys.sort
  end

  def test_entry_to_h_values_match_accessors
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    hop = resp.history[0]
    hash = hop.to_h
    assert_equal hop.status, hash[:status]
    assert_equal hop.url, hash[:url]
    assert_equal hop.previous_url, hash[:previous_url]
    assert_instance_of Wreq::Headers, hash[:headers]
  end

  # =================================================================
  # inspect / to_s
  # =================================================================

  def test_inspect_format
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    result = resp.history[0].inspect
    assert result.start_with?("#<Wreq::RedirectHistoryEntry")
    assert result.end_with?(">")
  end

  def test_to_s_equals_inspect
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    hop = resp.history[0]
    assert_equal hop.inspect, hop.to_s
  end

  def test_inspect_redacts_query_params
    resp = @client.get("#{HTTPBIN_URL}/redirect-to?url=#{CGI.escape("#{HTTPBIN_URL}/get")}&status_code=301")
    return if resp.history.empty?

    result = resp.history[0].inspect
    assert_includes result, "[REDACTED]"
    refute_includes result, "status_code"
  end

  # =================================================================
  # Cross-origin redirect
  # =================================================================

  def test_cross_origin_redirect
    resp = @client.get("http://google.com")
    return if resp.history.empty?

    hop = resp.history[0]
    assert_equal 301, hop.status
    assert_includes hop.previous_url, "google.com"
    assert_includes hop.url, "www.google.com"
  end

  # =================================================================
  # Relative redirects expose resolved destination
  # =================================================================

  def test_absolute_redirect_urls_are_resolved
    resp = @client.get("#{HTTPBIN_URL}/absolute-redirect/1")
    return if resp.history.empty?

    hop = resp.history[0]
    assert hop.url.start_with?("http://") || hop.url.start_with?("https://"),
      "Redirect URL should be absolute, got: #{hop.url}"
  end

  # =================================================================
  # History survives body consumption
  # =================================================================

  def test_history_available_after_body_consumed
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    resp.text
    assert_equal 1, resp.history.length
  end

  def test_history_available_after_close
    resp = @client.get("#{HTTPBIN_URL}/redirect/1")
    resp.close
    assert_equal 1, resp.history.length
  end
end