require "test_helper"
require "json"
require "socket"

class JsonPrecisionTest < Minitest::Test
  INTEGER_BOUNDARIES = [
    (2**53) - 1,
    2**53,
    (2**63) - 1,
    2**63,
    (2**64) - 1,
    -((2**53) - 1),
    -(2**53),
    -((2**63) - 1),
    -(2**63),
    -(2**63) - 1,
    -((2**64) - 1),
    -(2**64),
    2**100,
    -(2**100),
    2**256,
    -(2**256)
  ].freeze

  def test_response_json_matches_json_parse_for_large_integers
    payload = precision_payload

    with_json_server(response_body: JSON.generate(payload)) do |url, _requests|
      response = Wreq.get(url)
      expected = JSON.parse(response.bytes)
      actual = response.json

      assert_equal payload, expected
      assert_equal expected, actual
      assert actual.fetch("integers").all? { |value| value.instance_of?(Integer) }
      assert_instance_of Float, actual.fetch("fraction")
      assert_instance_of Float, actual.fetch("exponent")
    end
  end

  def test_request_json_preserves_large_integers_and_nested_values
    payload = precision_payload

    with_json_server do |url, requests|
      response = Wreq.post(url, json: payload)
      request = requests.pop

      assert_equal "application/json", request.fetch(:headers).fetch("content-type")
      assert_equal JSON.generate(payload), request.fetch(:body)
      assert_equal payload, response.json
    end
  end

  def test_request_json_nil_is_serialized_as_null
    with_json_server do |url, requests|
      response = Wreq.post(url, json: nil)
      request = requests.pop

      assert_equal "null", request.fetch(:body)
      assert_nil response.json
    end
  end

  def test_request_json_accepts_symbols_and_preserves_object_order
    payload = {second: 2**100, first: :value}

    with_json_server do |url, requests|
      response = Wreq.post(url, json: payload)
      request = requests.pop
      actual = response.json

      assert_equal JSON.generate(payload), request.fetch(:body)
      assert_equal ["second", "first"], actual.keys
      assert_equal({"second" => 2**100, "first" => "value"}, actual)
    end
  end

  def test_request_json_enforces_documented_nesting_limit
    accepted = 0
    100.times { accepted = [accepted] }

    with_json_server do |url, requests|
      Wreq.post(url, json: accepted)

      assert_equal JSON.generate(accepted), requests.pop.fetch(:body)
    end

    error = assert_raises(Wreq::BuilderError) do
      Wreq.post("http://127.0.0.1:1/", json: [accepted])
    end
    assert_match(/nesting exceeds 100 levels/, error.message)
  end

  def test_unsupported_request_json_raises_before_socket_io
    server = TCPServer.new("127.0.0.1", 0)
    port = server.addr[1]

    error = assert_raises(Wreq::BuilderError) do
      Wreq.post("http://127.0.0.1:#{port}/", json: {"value" => Float::NAN})
    end

    cyclic = []
    cyclic << cyclic
    nesting_error = assert_raises(Wreq::BuilderError) do
      Wreq.post("http://127.0.0.1:#{port}/", json: cyclic)
    end

    cyclic_hash = {}
    cyclic_hash["self"] = cyclic_hash
    hash_nesting_error = assert_raises(Wreq::BuilderError) do
      Wreq.post("http://127.0.0.1:#{port}/", json: cyclic_hash)
    end

    key_error = assert_raises(Wreq::BuilderError) do
      Wreq.post("http://127.0.0.1:#{port}/", json: {1 => "value"})
    end

    assert_match(/non-finite/, error.message)
    assert_match(/nesting/, nesting_error.message)
    assert_match(/nesting/, hash_nesting_error.message)
    assert_match(/keys/, key_error.message)
    assert_equal :wait_readable, server.accept_nonblock(exception: false)
  ensure
    server&.close unless server&.closed?
  end

  def test_invalid_response_json_raises_decoding_error
    with_json_server(response_body: '{"id":') do |url, _requests|
      response = Wreq.get(url)

      assert_raises(Wreq::DecodingError) { response.json }
    end
  end

  def test_fractional_and_large_exponent_numbers_match_json_parse
    source = '{"fraction":0.12345678901234567890123456789,"exponent":1e400}'

    with_json_server(response_body: source) do |url, _requests|
      response = Wreq.get(url)
      expected = JSON.parse(response.bytes)
      actual = response.json

      assert_equal expected, actual
      assert_instance_of Float, actual.fetch("fraction")
      assert_equal Float::INFINITY, actual.fetch("exponent")
    end
  end

  private

  def precision_payload
    {
      "integers" => INTEGER_BOUNDARIES,
      "nested" => {
        "array" => [2**100, {"negative" => -(2**100)}],
        "object" => {"unsigned_64_max" => (2**64) - 1}
      },
      "shapes" => ["text", true, false, nil],
      "fraction" => 1.25,
      "exponent" => 1.0e40
    }
  end

  def with_json_server(response_body: nil)
    server = TCPServer.new("127.0.0.1", 0)
    port = server.addr[1]
    requests = Queue.new
    thread = Thread.new do
      socket = server.accept
      begin
        request_line = socket.gets
        headers = read_headers(socket)
        content_length = headers.fetch("content-length", "0").to_i
        body = content_length.zero? ? "" : socket.read(content_length)
        requests << {request_line: request_line, headers: headers, body: body}

        response = response_body.nil? ? body : response_body
        socket.write "HTTP/1.1 200 OK\r\n"
        socket.write "Content-Type: application/json\r\n"
        socket.write "Content-Length: #{response.bytesize}\r\n"
        socket.write "Connection: close\r\n\r\n"
        socket.write response
      ensure
        socket.close unless socket.closed?
      end
    rescue IOError, SystemCallError
      nil
    ensure
      server.close unless server.closed?
    end
    thread.report_on_exception = false

    yield "http://127.0.0.1:#{port}/", requests
  ensure
    server&.close unless server&.closed?
    thread&.join(5)
  end

  def read_headers(socket)
    headers = {}
    while (line = socket.gets)
      break if line == "\r\n"

      name, value = line.split(":", 2)
      headers[name.downcase] = value.strip
    end
    headers
  end
end
