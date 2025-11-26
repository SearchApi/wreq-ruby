require "test_helper"

class ModuleMethodsTest < Minitest::Test
  def test_module_get
    response = Wreq.get("http://localhost:8080/get")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_post
    response = Wreq.post("http://localhost:8080/post",
      json: {module: "test"})
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_put
    response = Wreq.put("http://localhost:8080/put",
      json: {data: "test"})
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_delete
    response = Wreq.delete("http://localhost:8080/delete")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_patch
    response = Wreq.patch("http://localhost:8080/patch",
      json: {update: "field"})
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "update"
    assert_includes response.text, "field"
  end

  def test_module_request_method
    response = Wreq.request(Wreq::Method::GET, "http://localhost:8080/get")
    refute_nil response
    assert_equal 200, response.code
  end

  def test_module_methods_with_parameters
    response = Wreq.get("http://localhost:8080/get",
      headers: {"Accept" => "application/json"},
      query: {"test" => "module"})
    refute_nil response
    assert_equal response.url, "http://localhost:8080/get?test=module"
    assert_includes response.text, "http://localhost:8080/get?test=module"
    assert_equal 200, response.code
  end

  def test_module_post_with_json
    response = Wreq.post("http://localhost:8080/post",
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
    response = Wreq.post("http://localhost:8080/post",
      form: {"field1" => "value1", "field2" => "value2"})
    refute_nil response
    assert_equal 200, response.code
    assert_includes response.text, "field1"
    assert_includes response.text, "field2"
  end
end
