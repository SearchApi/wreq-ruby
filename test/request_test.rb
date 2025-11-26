require "test_helper"

class WreqHttpbinTest < Minitest::Test
  def setup
    # Ensure we have a working client
    @client = Wreq::Client.new(timeout: 30)
  end

  def test_module_get_method
    response = Wreq.get("http://localhost:8080/get")
    assert_equal 200, response.code
    assert_respond_to response, :text
  end

  def test_module_post_with_json
    data = {name: "wreq-ruby", version: "0.1.0"}
    response = Wreq.post("http://localhost:8080/post", json: data)
    assert_equal 200, response.code
  end

  def test_client_instance_basic
    response = @client.get("http://localhost:8080/get")
    assert_equal 200, response.code
  end

  def test_client_with_custom_headers
    headers = {"User-Agent" => "wreq-ruby/test", "Accept" => "application/json"}
    response = @client.get("http://localhost:8080/headers", headers: headers)
    assert_equal 200, response.code
  end

  def test_all_http_methods_client
    methods = [
      Wreq::Method::GET,
      Wreq::Method::POST,
      Wreq::Method::PUT,
      Wreq::Method::DELETE,
      Wreq::Method::PATCH,
      Wreq::Method::HEAD,
      Wreq::Method::OPTIONS
    ]

    methods.each do |method|
      response = @client.request(method, "http://localhost:8080/#{method}")
      assert [200, 405].include?(response.code), "client.#{method} failed with status #{response.code}"
    end
  end

  def test_request_with_query_params
    params = {"param1" => "value1", "param2" => "value2"}
    response = Wreq.get("http://localhost:8080/get", query: params)
    assert_equal 200, response.code
    assert_includes response.text, "param1=value1"
    assert_includes response.text, "param2=value2"
  end

  def test_post_with_form_data
    data = {"field1" => "value1", "field2" => "value2"}
    response = Wreq.post("http://localhost:8080/post", form: data)
    assert_equal 200, response.code
    assert_includes response.text, "field1"
    assert_includes response.text, "value1"
    assert_includes response.text, "field2"
    assert_includes response.text, "value2"
  end

  def test_post_with_raw_body
    body = "This is raw body content"
    headers = {"Content-Type" => "text/plain"}
    response = Wreq.post("http://localhost:8080/post", body: body, headers: headers)
    assert_equal 200, response.code
    assert_includes response.text, body
  end

  def test_basic_authentication
    response = Wreq.get("http://localhost:8080/basic-auth/user/pass", basic_auth: ["user", "pass"])
    assert_equal 200, response.code
  end

  def test_bearer_authentication
    response = Wreq.get("http://localhost:8080/bearer", bearer_auth: "test-token")
    assert_equal 200, response.code
  end

  def test_redirect_following
    response = Wreq.get("http://localhost:8080/redirect/2", allow_redirects: true)
    assert_equal 200, response.code
  end

  def test_redirect_blocking
    response = Wreq.get("http://localhost:8080/redirect/1", allow_redirects: false)
    assert_equal 302, response.code
  end

  def test_response_status_methods
    response = Wreq.get("http://localhost:8080/status/200")
    assert_equal 200, response.code

    if response.status.respond_to?(:success?)
      assert response.status.success?
    end
  end

  def test_404_response
    response = Wreq.get("http://localhost:8080/status/404")
    assert_equal 404, response.code

    if response.status.respond_to?(:client_error?)
      assert response.status.client_error?
    end
  end

  def test_json_response_parsing
    response = Wreq.get("http://localhost:8080/json")
    assert_equal 200, response.code

    if response.respond_to?(:json)
      json_data = response.json
      assert json_data.is_a?(Hash)
    end
  end

  def test_gzip_compression
    response = Wreq.get("http://localhost:8080/gzip", gzip: true)
    assert_equal 200, response.code
  end

  def test_headers_iteration
    response = Wreq.get("http://localhost:8080/get")

    if response.respond_to?(:each_header)
      count = 0
      response.each_header do |name, value|
        assert name.is_a?(String)
        assert value.is_a?(String)
        count += 1
        break if count >= 5  # Just test first few
      end
      assert count > 0, "Should iterate over at least one header"
    end
  end

  def test_response_properties
    response = Wreq.get("http://localhost:8080/get")

    assert_respond_to response, :code
    assert_respond_to response, :status
    assert_respond_to response, :version
    assert_respond_to response, :url

    assert response.code.is_a?(Integer)
    assert response.url.is_a?(String)
  end

  def test_timeout_functionality
    # Test that short timeouts properly raise exceptions
    assert_raises(Wreq::TimeoutError) do
      Wreq.get("http://localhost:8080/delay/10", timeout: 1)
    end

    # Test that reasonable timeouts work normally
    start_time = Time.now
    response = Wreq.get("http://localhost:8080/delay/1", timeout: 5)
    elapsed = Time.now - start_time

    assert_equal 200, response.code
    assert elapsed < 5, "Request should complete within timeout"
  end

  def test_multiple_requests_performance
    start_time = Time.now

    3.times do |i|
      response = Wreq.get("http://localhost:8080/get?request=#{i}")
      assert_equal 200, response.code
    end

    elapsed = Time.now - start_time
    assert elapsed < 30, "3 requests should complete within 30 seconds"
  end

  def test_error_handling
    assert_raises do
      Wreq.get("https://this-domain-does-not-exist-12345.com")
    end
  end

  def test_client_configuration
    client = Wreq::Client.new(
      timeout: 10,
      headers: {"User-Agent" => "wreq-ruby-test"},
      allow_redirects: false
    )

    # Test that client was created successfully
    assert_instance_of Wreq::Client, client

    # Test that it can make requests
    response = client.get("http://localhost:8080/get")
    assert_equal 200, response.code
  end

  def test_enum_constants
    # Test Method enum
    assert_const_defined "Wreq::Method"
    if defined?(Wreq::Method)
      %w[GET POST PUT DELETE HEAD PATCH OPTIONS].each do |method|
        assert Wreq::Method.const_defined?(method), "Method::#{method} should be defined"
      end
    end

    # Test Version enum
    assert_const_defined "Wreq::Version"
    if defined?(Wreq::Version)
      %w[HTTP_11 HTTP_2].each do |version|
        assert Wreq::Version.const_defined?(version), "Version::#{version} should be defined"
      end
    end
  end

  private

  def assert_const_defined(const_name)
    const_name.split("::").inject(Object) { |o, c| o.const_get(c) }
  rescue NameError
    flunk "#{const_name} should be defined"
  end
end
