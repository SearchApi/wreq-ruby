require "test_helper"

class RequestParametersTest < Minitest::Test
  def test_query_parameters
    response = Wreq.get("http://localhost:8080/get",
                        query: {
                          "string" => "value",
                          "number" => "123",
                          "boolean" => "true",
                        })
    assert_equal 200, response.code

    json_data = response.json
    args = json_data["args"]
    assert_equal "value", args["string"]
    assert_equal "123", args["number"]  # Query params are strings
    assert_equal "true", args["boolean"]
  end

  def test_headers
    custom_headers = {
      "X-Custom-Header" => "custom-value",
      "User-Agent" => "wreq-ruby-test/1.0",
      "Accept" => "application/json",
    }

    response = Wreq.get("http://localhost:8080/get", headers: custom_headers)
    assert_equal 200, response.code

    json_data = response.json
    request_headers = json_data["headers"]

    assert_equal "custom-value", request_headers["X-Custom-Header"]
    assert_equal "wreq-ruby-test/1.0", request_headers["User-Agent"]
    assert_equal "application/json", request_headers["Accept"]
  end

  def test_json_body
    json_data = {
      "string" => "test",
      "number" => 42,
      "boolean" => true,
      "array" => [1, 2, 3],
      "object" => { "nested" => "value" },
    }

    response = Wreq.post("http://localhost:8080/post", json: json_data)
    assert_equal 200, response.code

    response_data = response.json
    sent_data = response_data["json"]

    assert_equal "test", sent_data["string"]
    assert_equal 42, sent_data["number"]
    assert_equal true, sent_data["boolean"]
    assert_equal [1, 2, 3], sent_data["array"]
    assert_equal({ "nested" => "value" }, sent_data["object"])
  end

  def test_form_body
    form_data = {
      "field1" => "value1",
      "field2" => "value2",
      "number" => "123",
    }

    response = Wreq.post("http://localhost:8080/post", form: form_data)
    assert_equal 200, response.code

    response_data = response.json
    sent_form = response_data["form"]

    assert_equal "value1", sent_form["field1"]
    assert_equal "value2", sent_form["field2"]
    assert_equal "123", sent_form["number"]
  end

  def test_combined_parameters
    response = Wreq.post("http://localhost:8080/post",
                         query: { "q" => "search" },
                         headers: { "X-Test" => "combined" },
                         json: { "data" => "payload" })

    assert_equal 200, response.code

    json_data = response.json

    # Check query parameters
    assert_equal "search", json_data["args"]["q"]

    # Check headers
    assert_equal "combined", json_data["headers"]["X-Test"]

    # Check JSON body
    assert_equal "payload", json_data["json"]["data"]
  end

  def test_empty_parameters
    # Test with no additional parameters
    response = Wreq.get("http://localhost:8080/get")
    assert_equal 200, response.code

    json_data = response.json
    assert_instance_of Hash, json_data["args"]
    assert json_data["args"].empty?
  end

  def test_special_characters_in_query
    special_data = {
      "space" => "value with spaces",
      "symbols" => "!@#$%^&*()",
      "unicode" => "测试中文",
      "url_chars" => "a=b&c=d",
    }

    response = Wreq.get("http://localhost:8080/get", query: special_data)
    assert_equal 200, response.code

    json_data = response.json
    args = json_data["args"]

    assert_equal "value with spaces", args["space"]
    assert_equal "!@#$%^&*()", args["symbols"]
    assert_equal "测试中文", args["unicode"]
    assert_equal "a=b&c=d", args["url_chars"]
  end

  def test_nested_json_data
    nested_data = {
      "level1" => {
        "level2" => {
          "level3" => {
            "value" => "deep",
          },
        },
        "array" => [
          { "item" => 1 },
          { "item" => 2 },
        ],
      },
    }

    response = Wreq.post("http://localhost:8080/post", json: nested_data)
    assert_equal 200, response.code

    response_data = response.json
    sent_json = response_data["json"]

    assert_equal "deep", sent_json["level1"]["level2"]["level3"]["value"]
    assert_equal 1, sent_json["level1"]["array"][0]["item"]
    assert_equal 2, sent_json["level1"]["array"][1]["item"]
  end

  def test_method_specific_parameters
    methods_and_urls = {
      :get => "http://localhost:8080/get",
      :post => "http://localhost:8080/post",
      :put => "http://localhost:8080/put",
      :patch => "http://localhost:8080/patch",
      :delete => "http://localhost:8080/delete",
    }

    methods_and_urls.each do |method, url|
      response = Wreq.send(method, url,
                           headers: { "X-Method" => method.to_s },
                           query: { "method" => method.to_s })

      assert_equal 200, response.code, "#{method} request failed"

      json_data = response.json
      assert_equal method.to_s, json_data["headers"]["X-Method"]
      assert_equal method.to_s, json_data["args"]["method"]
    end
  end
end
