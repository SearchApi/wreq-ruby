require "test_helper"

class ErrorHandlingTest < Minitest::Test
  def test_network_error_handling
    # Try to connect to a non-existent domain
    response = Wreq.get("https://definitely-not-a-real-domain-12345.com")
    flunk "Expected network error but got response: #{response.code}"
  rescue => e
    assert_instance_of Wreq::ConnectionError, e
    # Network errors should be caught and wrapped appropriately
  end

  def test_invalid_url_handling
    # Invalid URL format
    response = Wreq.get("not-a-valid-url")
    flunk "Expected URL error but got response: #{response.code}"
  rescue => e
    assert_instance_of Wreq::BuilderError, e
  end

  def test_http_error_status_codes
    # These should not raise errors, just return responses with error codes
    [400, 401, 403, 404, 500, 502, 503].each do |status_code|
      response = Wreq.get("http://localhost:8080/status/#{status_code}")
      assert_equal status_code, response.code
      # Should not raise an exception for HTTP error codes
    end
  end

  def test_timeout_handling
    # Test timeout with a delay that should definitely cause timeout

    # Request with a very short timeout that should fail
    response = Wreq.get("http://localhost:8080/delay/10", timeout: 1)
    # If we get here, the request didn't timeout (unexpected)
    flunk "Expected timeout error but got response: #{response.code}"
  rescue => e
    # Timeout error is expected
    assert_instance_of Wreq::TimeoutError, e
    # Could also check error message contains timeout-related keywords
  end

  def test_invalid_json_response
    # Get a non-JSON response and try to parse as JSON
    response = Wreq.get("http://localhost:8080/html")
    assert_equal 200, response.code

    # Should raise an error when trying to parse HTML as JSON
    begin
      response.json
    rescue => e
      assert_instance_of Wreq::DecodingError, e
    end
  end

  def test_empty_response_json
    response = Wreq.get("http://localhost:8080/status/204")
    assert_equal 204, response.code
    assert_equal "", response.text

    # Empty body should raise error when parsing as JSON
    begin
      response.json
    rescue => e
      assert_instance_of Wreq::DecodingError, e
    end
  end

  def test_proxy_error_handling
    invalid_proxies = [
      "http://invalid.proxy:8080",
      "https://invalid.proxy:8080",
      "socks4://invalid.proxy:8080",
      "socks4a://invalid.proxy:8080",
      "socks5://invalid.proxy:8080",
      "socks5h://invalid.proxy:8080"
    ]
    target_urls = ["https://example.com", "http://example.com"]

    invalid_proxies.each do |proxy|
      target_urls.each do |url|
        Wreq.get(url, proxy: proxy, timeout: 5)
        flunk "Expected proxy connection error but got response"
      rescue => e
        assert_instance_of Wreq::ProxyConnectionError, e
      end
    end
  end
end
