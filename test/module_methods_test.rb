require "test_helper"

class ModuleMethodsTest < Minitest::Test
  def test_module_get
    response = Wreq.get("#{HTTPBIN_URL}/get")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_post
    response = Wreq.post("#{HTTPBIN_URL}/post",
      json: {module: "test"})
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_put
    response = Wreq.put("#{HTTPBIN_URL}/put",
      json: {data: "test"})
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_delete
    response = Wreq.delete("#{HTTPBIN_URL}/delete")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_patch
    response = Wreq.patch("#{HTTPBIN_URL}/patch",
      json: {update: "field"})
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "update"
    assert_includes response.text, "field"
  end

  def test_module_request_method
    response = Wreq.request(Wreq::Method::GET, "#{HTTPBIN_URL}/get")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_methods_with_parameters
    response = Wreq.get("#{HTTPBIN_URL}/get",
      headers: {"Accept" => "application/json"},
      query: {"test" => "module"})
    refute_nil response
    assert_equal "#{HTTPBIN_URL}/get?test=module", response.url
    assert_equal "module", httpbin_fetch(response.json["args"], "test")
    assert_equal 200, response.code
  end

  def test_module_post_with_json
    response = Wreq.post("#{HTTPBIN_URL}/post",
      json: {
        string: "test",
        number: 123,
        boolean: true,
        array: [1, 2, 3]
      })
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_post_with_form
    response = Wreq.post("#{HTTPBIN_URL}/post",
      form: {"field1" => "value1", "field2" => "value2"})
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "field1"
    assert_includes response.text, "field2"
  end
end
