require "test_helper"

class HeadersTest < Minitest::Test
  def setup
    @headers = Wreq::Headers.new(
      "Content-Type" => "application/json",
      "X-Custom-Header" => "custom-value"
    )
  end

  def test_headers_class_and_empty_constructor
    headers = Wreq::Headers.new

    assert_instance_of Wreq::Headers, headers
    assert headers.empty?
    assert_equal 0, headers.length
    assert_includes Wreq::Headers.ancestors, Enumerable
  end

  def test_collection_api_methods_are_available
    collection_methods = [:[], :[]=, :fetch, :delete, :size, :to_a, :to_h, :to_hash, :each]

    collection_methods.each do |method|
      assert_respond_to @headers, method
    end
  end

  def test_initialize_from_hash_and_headers
    headers = Wreq::Headers.new("Accept" => "application/json")
    copy = Wreq::Headers.new(headers)

    headers["Accept"] = "text/plain"

    assert_equal "text/plain", headers["Accept"]
    assert_equal "application/json", copy["Accept"]
  end

  def test_initialize_from_enumerable_pairs
    source = Class.new do
      include Enumerable

      def each
        yield "X-First", "one"
        yield :set_cookie, ["a=1", "b=2"]
      end
    end.new

    headers = Wreq::Headers.new(source)

    assert_includes headers.keys, "x-first"
    assert_includes headers.keys, "set-cookie"
    assert_equal ["a=1", "b=2"], headers[:set_cookie]
  end

  def test_initialize_rejects_invalid_sources_and_pairs
    assert_raises(Wreq::BuilderError) { Wreq::Headers.new(Object.new) }
    assert_raises(Wreq::BuilderError) { Wreq::Headers.new([["Accept"]]) }
    assert_raises(ArgumentError) { Wreq::Headers.new({}, {}) }
  end

  def test_string_and_symbol_names_are_normalized
    headers = Wreq::Headers.new(
      "x-CuStOm-Header" => "value",
      content_type: "application/json"
    )

    assert_includes headers.keys, "x-custom-header"
    assert_includes headers.keys, "content-type"
    assert_equal "value", headers["X-CUSTOM-HEADER"]
    assert_equal "application/json", headers[:content_type]
  end

  def test_get_returns_first_value
    headers = Wreq::Headers.new("Accept" => ["application/json", "text/plain"])

    assert_equal "application/json", headers.get("accept")
    assert_equal "application/json", headers.get(:accept)
    assert_nil headers.get(:missing)
  end

  def test_index_uses_nil_string_and_array_shapes
    headers = Wreq::Headers.new(
      "Accept" => "application/json",
      "Set-Cookie" => ["a=1", "b=2"]
    )

    assert_nil headers["Missing"]
    assert_equal "application/json", headers["Accept"]
    assert_equal ["a=1", "b=2"], headers["Set-Cookie"]
  end

  def test_get_all_always_returns_an_array
    assert_equal ["application/json"], @headers.get_all("CONTENT-TYPE")
    assert_equal [], @headers.get_all("Missing")
  end

  def test_set_replaces_existing_occurrences
    headers = Wreq::Headers.new("Accept" => ["application/json", "text/plain"])

    headers.set("Accept", ["text/html", "application/xml"])

    assert_equal ["text/html", "application/xml"], headers.get_all("Accept")
    assert_equal 2, headers.length
  end

  def test_index_assignment_replaces_existing_occurrences
    headers = Wreq::Headers.new("Set-Cookie" => "old=1")

    assigned = headers.public_send(:[]=, :set_cookie, ["a=1", "b=2"])

    assert_equal ["a=1", "b=2"], assigned
    assert_equal ["a=1", "b=2"], headers.get_all("Set-Cookie")
  end

  def test_append_keeps_values_as_separate_occurrences
    headers = Wreq::Headers.new
    headers.append("Set-Cookie", "a=1")
    headers.append("Set-Cookie", ["b=2", "c=3"])

    assert_equal ["a=1", "b=2", "c=3"], headers.get_all("Set-Cookie")
    refute_includes headers.get_all("Set-Cookie"), "a=1,b=2,c=3"
  end

  def test_set_and_append_return_nil
    headers = Wreq::Headers.new

    assert_nil headers.set("Accept", "application/json")
    assert_nil headers.append("Accept", "text/plain")
  end

  def test_empty_array_values_remove_or_leave_headers_unchanged
    headers = Wreq::Headers.new("Accept" => "application/json")

    assert_nil headers.set("Accept", [])
    refute headers.key?("Accept")

    assert_nil headers.append("X-Empty", [])
    refute headers.key?("X-Empty")
  end

  def test_fetch_existing_and_missing_values
    assert_equal "application/json", @headers.fetch(:content_type)
    assert_equal "fallback", @headers.fetch("Missing", "fallback")
    assert_equal "MISSING", @headers.fetch("Missing") { |name| name.upcase }
    assert_raises(KeyError) { @headers.fetch("Missing") }
  end

  def test_fetch_block_takes_precedence_over_default
    result = @headers.fetch("Missing", "fallback") { "from block" }

    assert_equal "from block", result
  end

  def test_fetch_preserves_repeated_values_and_explicit_nil_default
    headers = Wreq::Headers.new("Set-Cookie" => ["a=1", "b=2"])

    assert_equal ["a=1", "b=2"], headers.fetch(:set_cookie)
    assert_nil headers.fetch("Missing", nil)
  end

  def test_remove_and_delete_remove_every_occurrence
    headers = Wreq::Headers.new("Set-Cookie" => ["a=1", "b=2"])

    assert_equal "a=1", headers.delete(:set_cookie)
    assert_nil headers["Set-Cookie"]
    assert_nil headers.remove("Set-Cookie")
  end

  def test_contains_and_key_are_case_insensitive
    assert @headers.contains?("CONTENT-TYPE")
    assert @headers.contains?(:content_type)
    assert @headers.key?("content-type")
    refute @headers.key?("Missing")
  end

  def test_length_counts_occurrences_and_keys_are_unique
    headers = Wreq::Headers.new(
      "Accept" => ["application/json", "text/plain"],
      "Content-Type" => "application/json"
    )

    assert_equal 3, headers.length
    assert_equal 3, headers.size
    assert_equal 2, headers.keys.length
  end

  def test_clear_returns_self
    headers = Wreq::Headers.new("Accept" => "application/json")

    assert_same headers, headers.clear
    assert headers.empty?
  end

  def test_values_include_every_occurrence
    headers = Wreq::Headers.new("Accept" => ["application/json", "text/plain"])

    assert_equal ["application/json", "text/plain"], headers.values
  end

  def test_each_yields_every_occurrence_and_returns_self
    headers = Wreq::Headers.new("Set-Cookie" => ["a=1", "b=2"])
    pairs = []

    returned = headers.each { |name, value| pairs << [name, value] }

    assert_same headers, returned
    assert_equal [["set-cookie", "a=1"], ["set-cookie", "b=2"]], pairs
  end

  def test_each_without_a_block_returns_chainable_enumerator
    headers = Wreq::Headers.new(
      "Accept" => "application/json",
      "Set-Cookie" => ["a=1", "b=2"]
    )

    enumerator = headers.each
    cookies = enumerator.select { |name, _value| name == "set-cookie" }

    assert_instance_of Enumerator, enumerator
    assert_equal [["set-cookie", "a=1"], ["set-cookie", "b=2"]], cookies
  end

  def test_each_allows_mutating_headers_from_the_block
    headers = Wreq::Headers.new("X-First" => "one")
    yielded_names = []

    headers.each do |name, _value|
      yielded_names << name
      headers["X-Added"] = "two"
    end

    assert_equal ["x-first"], yielded_names
    assert_equal "two", headers["X-Added"]
  end

  def test_enumerable_methods_use_each
    headers = Wreq::Headers.new(
      "Accept" => "application/json",
      "Set-Cookie" => ["a=1", "b=2"]
    )

    cookies = headers.select { |name, _value| name == "set-cookie" }

    assert_equal [["set-cookie", "a=1"], ["set-cookie", "b=2"]], cookies
  end

  def test_to_a_and_to_h_preserve_duplicate_values
    headers = Wreq::Headers.new(
      "Accept" => "application/json",
      "Set-Cookie" => ["a=1", "b=2"]
    )

    expected_pairs = [
      ["accept", "application/json"],
      ["set-cookie", "a=1"],
      ["set-cookie", "b=2"]
    ]
    assert_equal expected_pairs.sort, headers.to_a.sort
    assert_equal({
      "accept" => "application/json",
      "set-cookie" => ["a=1", "b=2"]
    }, headers.to_h)
    assert_equal headers.to_h, headers.to_hash
  end

  def test_special_characters_in_header_values
    value = "Bearer token-123_abc/xyz+456=789"
    @headers.set("Authorization", value)

    assert_equal value, @headers.get("Authorization")
  end

  def test_invalid_header_names_and_values_raise_builder_error
    headers = Wreq::Headers.new

    assert_raises(Wreq::BuilderError) { headers.set(123, "value") }
    assert_raises(Wreq::BuilderError) { headers.set("Bad\nName", "value") }
    assert_raises(Wreq::BuilderError) { headers.set("X-Test", 123) }
    assert_raises(Wreq::BuilderError) { headers.append("X-Test", ["valid", 123]) }
    assert headers.empty?
  end

  def test_header_entry_limit_raises_builder_error_without_partial_update
    headers = Wreq::Headers.new
    values = Array.new(32_769, "value")

    error = assert_raises(Wreq::BuilderError) { headers.append("X-Large", values) }

    assert_match(/32,768/, error.message)
    assert headers.empty?
  end

  def test_response_headers_integration
    headers = response_headers
    pairs = headers.each.to_a

    assert_instance_of Wreq::Headers, headers
    refute headers.empty?
    assert headers.contains?("Content-Type")
    assert_equal headers.length, pairs.length
  end

  def test_response_headers_are_fresh_mutable_snapshots
    response = Wreq.get("#{HTTPBIN_URL}/response-headers", query: {"X-Test" => "original"})
    first = response.headers
    second = response.headers

    refute_same first, second
    first["X-Test"] = "changed"
    first["X-Local"] = "value"

    assert_equal "changed", first["X-Test"]
    assert_equal "original", second["X-Test"]
    assert_nil second["X-Local"]
    assert_equal "original", response.headers["X-Test"]
  end

  private

  def response_headers
    Wreq.get(
      "#{HTTPBIN_URL}/response-headers",
      query: {"X-Custom-Header" => "custom-value"}
    ).headers
  end
end
