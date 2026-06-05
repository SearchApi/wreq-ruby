require "test_helper"

class OrigHeaderTest < Minitest::Test
  URL = "https://tls.browserleaks.com/http1"

  CASES = [
    {
      name: "mixed_case_descending",
      headers: {
        "X-Zeta-Token" => "zeta",
        "x-alpha-key" => "alpha",
        "X-MiXeD-CaSe" => "mixed"
      },
      orig_headers: ["X-Zeta-Token", "x-alpha-key", "X-MiXeD-CaSe"]
    },
    {
      name: "reverse_alpha_order",
      headers: {
        "X-Third" => "3",
        "X-Second" => "2",
        "X-First" => "1"
      },
      orig_headers: ["X-Third", "X-Second", "X-First"]
    },
    {
      name: "preserve_weird_casing",
      headers: {
        "x-a" => "a",
        "X-B" => "b",
        "x-C" => "c"
      },
      orig_headers: ["x-C", "x-a", "X-B"]
    },
    {
      name: "interleaved_tokens",
      headers: {
        "X-Token-3" => "v3",
        "X-Token-1" => "v1",
        "X-Token-2" => "v2"
      },
      orig_headers: ["X-Token-1", "X-Token-2", "X-Token-3"]
    }
  ].freeze

  def test_client_default_orig_headers_preserves_header_order_in_multiple_shuffled_cases
    CASES.each do |kase|
      client = Wreq::Client.new(
        headers: kase[:headers],
        orig_headers: kase[:orig_headers]
      )

      response = client.get(URL, version: Wreq::Version::HTTP_11)
      assert_equal 200, response.code, "case=#{kase[:name]}"

      echoed_headers = extract_http1_headers(response.json, kase[:name])
      assert_header_order(echoed_headers, kase[:orig_headers], kase[:name])
      assert_header_values(echoed_headers, kase[:headers], kase[:name])
    end
  end

  def test_module_request_orig_headers_preserves_header_order_in_multiple_shuffled_cases
    CASES.each do |kase|
      response = Wreq.get(
        URL,
        headers: kase[:headers],
        orig_headers: kase[:orig_headers],
        version: Wreq::Version::HTTP_11
      )
      assert_equal 200, response.code, "case=#{kase[:name]}"

      echoed_headers = extract_http1_headers(response.json, kase[:name])
      assert_header_order(echoed_headers, kase[:orig_headers], kase[:name])
      assert_header_values(echoed_headers, kase[:headers], kase[:name])
    end
  end

  private

  def extract_http1_headers(json, case_name)
    http1 = fetch_by_name(json, "http1")
    refute_nil http1, "case=#{case_name}: expected JSON key 'http1', got #{json.keys.inspect}"

    headers = fetch_by_name(http1, "headers")
    refute_nil headers, "case=#{case_name}: expected JSON key 'http1.headers'"
    headers
  end

  def fetch_by_name(hash_like, key_name)
    return hash_like[key_name] if hash_like.respond_to?(:key?) && hash_like.key?(key_name)
    return hash_like[key_name.to_sym] if hash_like.respond_to?(:key?) && hash_like.key?(key_name.to_sym)

    pair = hash_like.find { |k, _| k.to_s == key_name }
    pair&.last
  end

  def assert_header_order(echoed_headers, ordered_names, case_name)
    echoed_keys = echoed_headers.keys
    positions = ordered_names.map do |expected_name|
      index = echoed_keys.index(expected_name)
      refute_nil index, "case=#{case_name}: expected header to exist in echo: #{expected_name}"
      index
    end

    assert_equal positions.sort, positions,
      "case=#{case_name}: expected header order #{ordered_names.inspect}, got keys #{echoed_keys.inspect}"
  end

  def assert_header_values(echoed_headers, expected_headers, case_name)
    expected_headers.each do |name, expected_value|
      assert echoed_headers.key?(name),
        "case=#{case_name}: expected exact-case header name #{name}, got #{echoed_headers.keys.inspect}"
      assert_equal expected_value, echoed_headers[name]
    end
  end
end
