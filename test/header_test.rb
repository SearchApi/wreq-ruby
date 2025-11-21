require "test_helper"

class HeadersTest < Minitest::Test
  def setup
    @response = Wreq.get("http://localhost:8080/response-headers",
                         query: {
                           "X-Custom-Header" => "custom-value",
                           "X-Multi-Header" => "value1",
                         })
    @headers = @response.headers
  end

  def test_headers_class
    assert_instance_of Wreq::Headers, @headers
  end

  def test_initialize
    headers = Wreq::Headers.new
    assert_instance_of Wreq::Headers, headers
    assert headers.empty?
    assert_equal 0, headers.length
  end

  def test_get_existing_header
    # Content-Type should exist in response
    content_type = @headers.get("Content-Type")
    assert_instance_of String, content_type
    refute_nil content_type
  end

  def test_get_case_insensitive
    # Test case-insensitive lookup
    value1 = @headers.get("content-type")
    value2 = @headers.get("Content-Type")
    value3 = @headers.get("CONTENT-TYPE")

    assert_equal value1, value2
    assert_equal value2, value3
  end

  def test_get_nonexistent_header
    result = @headers.get("X-Nonexistent-Header-12345")
    assert_nil result
  end

  def test_get_all_single_value
    # Most headers have single values
    values = @headers.get_all("Content-Type")
    assert_instance_of Array, values
    assert_equal 1, values.length
    assert_equal @headers.get("Content-Type"), values.first
  end

  def test_get_all_nonexistent
    values = @headers.get_all("X-Nonexistent-Header")
    assert_instance_of Array, values
    assert_equal 0, values.length
    assert_empty values
  end

  def test_set_new_header
    headers = Wreq::Headers.new
    headers.set("X-Test-Header", "test-value")

    assert_equal "test-value", headers.get("X-Test-Header")
    assert_equal 1, headers.length
  end

  def test_set_replaces_existing
    headers = Wreq::Headers.new
    headers.set("X-Test", "value1")
    headers.set("X-Test", "value2")

    assert_equal "value2", headers.get("X-Test")
    values = headers.get_all("X-Test")
    assert_equal 1, values.length
    assert_equal "value2", values.first
  end

  def test_append_to_new_header
    headers = Wreq::Headers.new
    headers.append("Accept", "application/json")

    assert_equal "application/json", headers.get("Accept")
  end

  def test_append_to_existing_header
    headers = Wreq::Headers.new
    headers.set("Accept", "application/json")
    headers.append("Accept", "text/html")
    headers.append("Accept", "application/xml")

    values = headers.get_all("Accept")
    assert_equal 3, values.length
    assert_includes values, "application/json"
    assert_includes values, "text/html"
    assert_includes values, "application/xml"
  end

  def test_remove_existing_header
    headers = Wreq::Headers.new
    headers.set("X-Remove-Me", "value")

    removed_value = headers.remove("X-Remove-Me")
    assert_equal "value", removed_value
    assert_nil headers.get("X-Remove-Me")
  end

  def test_remove_nonexistent_header
    headers = Wreq::Headers.new
    result = headers.remove("X-Nonexistent")
    assert_nil result
  end

  def test_delete_alias
    headers = Wreq::Headers.new
    headers.set("X-Delete-Me", "value")

    removed_value = headers.remove("X-Delete-Me")
    assert_equal "value", removed_value
    assert_nil headers.get("X-Delete-Me")
  end

  def test_contains_existing
    assert @headers.contains?("Content-Type")
  end

  def test_contains_nonexistent
    refute @headers.contains?("X-Nonexistent-Header-12345")
  end

  def test_contains_case_insensitive
    # If Content-Type exists
    if @headers.contains?("Content-Type")
      assert @headers.contains?("content-type")
      assert @headers.contains?("CONTENT-TYPE")
    end
  end

  def test_key_alias
    # key? is an alias for contains?
    assert_equal @headers.contains?("Content-Type"), @headers.key?("Content-Type")
  end

  def test_length
    headers = Wreq::Headers.new
    assert_equal 0, headers.length

    headers.set("Header1", "value1")
    assert_equal 1, headers.length

    headers.set("Header2", "value2")
    assert_equal 2, headers.length

    # Setting same header shouldn't increase length
    headers.set("Header1", "new-value")
    assert_equal 2, headers.length
  end

  def test_empty_on_new_headers
    headers = Wreq::Headers.new
    assert headers.empty?
  end

  def test_empty_on_headers_with_data
    refute @headers.empty?
  end

  def test_clear
    headers = Wreq::Headers.new
    headers.set("Header1", "value1")
    headers.set("Header2", "value2")

    refute headers.empty?
    headers.clear
    assert headers.empty?
    assert_equal 0, headers.length
  end

  def test_keys
    headers = Wreq::Headers.new
    headers.set("Content-Type", "application/json")
    headers.set("Authorization", "Bearer token")

    keys = headers.keys
    assert_instance_of Array, keys
    assert_equal 2, keys.length
    assert_includes keys, "content-type"
    assert_includes keys, "authorization"
  end

  def test_keys_are_lowercase
    headers = Wreq::Headers.new
    headers.set("Content-Type", "text/html")
    headers.set("X-Custom-Header", "value")

    keys = headers.keys
    keys.each do |key|
      assert_equal key, key.downcase
    end
  end

  def test_values
    headers = Wreq::Headers.new
    headers.set("Content-Type", "application/json")
    headers.set("Authorization", "Bearer token")

    values = headers.values
    assert_instance_of Array, values
    assert_equal 2, values.length
    assert_includes values, "application/json"
    assert_includes values, "Bearer token"
  end

  def test_each_with_block
    headers = Wreq::Headers.new
    headers.set("Header1", "value1")
    headers.set("Header2", "value2")

    collected = {}
    headers.each do |name, value|
      assert_instance_of String, name
      assert_instance_of String, value
      collected[name] = value
    end

    assert_equal 2, collected.length
    assert_equal "value1", collected["header1"]
    assert_equal "value2", collected["header2"]
  end

  def test_multiple_operations_sequence
    headers = Wreq::Headers.new

    # Add headers
    headers.set("Content-Type", "application/json")
    headers.set("Accept", "application/json")

    assert_equal 2, headers.length

    # Append to Accept
    headers.append("Accept", "text/html")
    assert_equal 2, headers.get_all("Accept").length

    # Clear all
    headers.clear
    assert headers.empty?
  end

  def test_special_characters_in_header_values
    headers = Wreq::Headers.new
    special_value = "Bearer token-123_abc/xyz+456=789"
    headers.set("Authorization", special_value)

    assert_equal special_value, headers.get("Authorization")
  end

  def test_response_headers_integration
    # Test that headers from actual HTTP response work correctly
    assert_instance_of Wreq::Headers, @headers
    refute @headers.empty?

    # Should have common HTTP headers
    assert @headers.length > 0
  end

  def test_response_headers_each
    # Test iteration over real response headers
    count = 0
    @headers.each do |name, value|
      assert_instance_of String, name
      assert_instance_of String, value
      count += 1
    end

    assert count > 0
    assert_equal @headers.length, count
  end

  def test_headers_immutability_across_instances
    headers1 = Wreq::Headers.new
    headers2 = Wreq::Headers.new

    headers1.set("X-Test", "value1")
    headers2.set("X-Test", "value2")

    assert_equal "value1", headers1.get("X-Test")
    assert_equal "value2", headers2.get("X-Test")
  end
end
