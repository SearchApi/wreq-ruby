require "test_helper"

class ResponseTest < Minitest::Test
  def setup
    @response = Wreq.get("http://localhost:8080/json")
  end

  def test_response_code
    assert_respond_to @response, :code
    assert_instance_of Integer, @response.code
    assert_equal 200, @response.code
  end

  def test_response_text
    assert_respond_to @response, :text
    assert_instance_of String, @response.text
    assert @response.text.length > 0
  end

  def test_response_json
    assert_respond_to @response, :json
    json_data = @response.json
    assert_instance_of Hash, json_data

    # httpbin.org/json returns a specific JSON structure
    assert json_data.key?("slideshow")
  end

  def test_response_each_header
    assert_respond_to @response, :headers

    header_count = 0
    @response.headers.each do |name, value|
      assert_instance_of String, name
      assert_instance_of String, value
      header_count += 1
    end
    assert header_count > 0
  end

  def test_response_with_non_json_content
    response = Wreq.get("http://localhost:8080/html")
    assert_equal 200, response.code
    assert_instance_of String, response.text
    assert response.text.include?("<html>")

    # JSON parsing should fail for HTML content
    assert_raises(StandardError) { response.json }
  end

  def test_response_status_codes
    # Test different status codes
    [200, 404, 500].each do |status|
      response = Wreq.get("http://localhost:8080/status/#{status}")
      assert_equal status, response.code
    end
  end

  def test_response_with_query_parameters
    response = Wreq.get("http://localhost:8080/get",
                        query: { "param1" => "value1", "param2" => "value2" })
    assert_equal 200, response.code

    json_data = response.json
    args = json_data["args"]
    assert_equal "value1", args["param1"]
    assert_equal "value2", args["param2"]
  end
end
