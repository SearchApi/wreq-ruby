require "test_helper"

class ClientTest < Minitest::Test
  def setup
    @client = Wreq::Client.new
  end

  def test_client_creation
    refute_nil @client
    assert_instance_of Wreq::Client, @client
  end

  def test_client_with_configuration
    client = Wreq::Client.new(
      timeout: 30,
      headers: { "User-Agent" => "wreq-ruby/test" },
    )
    refute_nil client
    assert_instance_of Wreq::Client, client
  end

  def test_get_request
    response = @client.get("http://localhost:8080/get")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_post_request
    response = @client.post("http://localhost:8080/post",
                            json: { test: "wreq-ruby" })
    refute_nil response
    assert_equal 200, response.code
  end

  def test_put_request
    response = @client.put("http://localhost:8080/put",
                           json: { data: "test" })
    refute_nil response
    assert_equal 200, response.code
  end

  def test_delete_request
    response = @client.delete("http://localhost:8080/delete")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_patch_request
    response = @client.patch("http://localhost:8080/patch",
                             json: { update: "field" })
    refute_nil response
    assert_equal 200, response.code
  end

  def test_request_with_query_params
    response = @client.get("http://localhost:8080/get",
                           query: { "param" => "value" })
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "param"
  end

  def test_request_with_form_data
    response = @client.post("http://localhost:8080/post",
                            form: { "field" => "value" })
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "field"
  end

  def test_request_with_raw_body
    response = @client.post("http://localhost:8080/post",
                            body: "raw content",
                            headers: { "Content-Type" => "text/plain" })
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "raw content"
  end

  def test_basic_authentication
    response = @client.get("http://localhost:8080/basic-auth/user/pass",
                           basic_auth: ["user", "pass"])
    refute_nil response
    assert_equal 200, response.code
  end

  def test_bearer_authentication
    response = @client.get("http://localhost:8080/bearer",
                           bearer_auth: "test-token")
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "test-token"
  end

  def test_redirect_following
    response = @client.get("http://localhost:8080/redirect/1",
                           allow_redirects: true)
    refute_nil response
    assert_equal 200, response.code
  end

  def test_redirect_blocking
    response = @client.get("http://localhost:8080/redirect/1",
                           allow_redirects: false)
    refute_nil response
    assert_equal 302, response.code
  end

  def test_gzip_compression
    response = @client.get("http://localhost:8080/gzip", gzip: true)
    refute_nil response
    assert_equal 200, response.code
  end

  def test_timeout_handling
    # Test that short timeouts properly raise exceptions
    assert_raises(Wreq::TimeoutError) do
      @client.get("http://localhost:8080/delay/10", timeout: 1)
    end

    # Test that reasonable timeouts work normally
    start_time = Time.now
    response = @client.get("http://localhost:8080/delay/1", timeout: 5)
    elapsed = Time.now - start_time

    refute_nil response
    assert_equal 200, response.code
    assert elapsed < 5, "Request should complete within timeout"
  end

  def test_status_codes
    response = @client.get("http://localhost:8080/status/404")
    refute_nil response
    assert_equal 404, response.code
  end
end
