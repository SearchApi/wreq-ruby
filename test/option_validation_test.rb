require "test_helper"

class OptionValidationTest < Minitest::Test
  INVALID_URL = "not a url"

  def test_unknown_client_option_names_every_invalid_key_without_values
    secret = "must-not-appear"

    error = assert_raises(ArgumentError) do
      Wreq::Client.new(timout: secret, history: secret)
    end

    assert_includes error.message, ":timout"
    assert_includes error.message, ":history"
    refute_includes error.message, secret
  end

  def test_unknown_options_are_checked_before_known_values
    secret = "must-not-appear"

    error = assert_raises(ArgumentError) do
      Wreq::Client.new(timeout: secret, history: Object.new, timout: secret)
    end

    assert_includes error.message, ":history"
    assert_includes error.message, ":timout"
    refute_includes error.message, secret

    conflict_error = assert_raises(ArgumentError) do
      Wreq::Client.new(http1_only: true, http2_only: true, history: secret)
    end
    assert_includes conflict_error.message, ":history"
    refute_includes conflict_error.message, secret
  end

  def test_unknown_request_options_raise_for_client_and_module_methods
    client_error = assert_raises(ArgumentError) do
      Wreq::Client.new.get(INVALID_URL, history: true)
    end
    module_error = assert_raises(ArgumentError) do
      Wreq.get(INVALID_URL, history: true)
    end

    assert_includes client_error.message, ":history"
    assert_includes module_error.message, ":history"
  end

  def test_hash_expansion_uses_the_same_validation
    valid = {timeout: 1, allow_redirects: true, max_redirects: 2}
    invalid = {timout: 1}

    assert_instance_of Wreq::Client, Wreq::Client.new(**valid)
    error = assert_raises(ArgumentError) { Wreq::Client.new(**invalid) }
    assert_includes error.message, ":timout"
  end

  def test_string_option_keys_are_supported_and_duplicate_forms_are_rejected
    assert_instance_of Wreq::Client, Wreq::Client.new({"timeout" => 1})

    error = assert_raises(ArgumentError) do
      Wreq::Client.new({:timeout => 1, "timeout" => 2})
    end
    assert_includes error.message, "duplicate option: :timeout"
  end

  def test_non_string_or_symbol_option_key_raises_type_error
    error = assert_raises(TypeError) { Wreq::Client.new({1 => true}) }

    assert_includes error.message, "option keys"
  end

  def test_hash_subclass_cannot_hide_unknown_options
    options_class = Class.new(Hash) do
      def keys
        []
      end
    end
    options = options_class.new
    options[:timout] = 1

    error = assert_raises(ArgumentError) { Wreq::Client.new(options) }
    assert_includes error.message, ":timout"
  end

  def test_invalid_known_values_name_the_option
    timeout_error = assert_raises(TypeError) do
      Wreq::Client.new(timeout: "slow")
    end
    address_error = assert_raises(ArgumentError) do
      Wreq::Client.new(local_address: "not-an-address")
    end
    proxy_error = assert_raises(TypeError) do
      Wreq::Client.new(proxy: Object.new)
    end
    proxy_uri_error = assert_raises(Wreq::BuilderError) do
      Wreq::Client.new(proxy: "invalid://")
    end
    orig_headers_error = assert_raises(Wreq::BuilderError) do
      Wreq.get(INVALID_URL, orig_headers: ["Accept", Object.new])
    end

    assert_includes timeout_error.message, ":timeout"
    assert_includes address_error.message, ":local_address"
    assert_includes proxy_error.message, ":proxy"
    assert_includes proxy_uri_error.message, ":proxy"
    assert_includes orig_headers_error.message, ":orig_headers"
  end

  def test_consumed_body_sender_error_preserves_class_and_names_option
    sender = Wreq::BodySender.new(1)

    assert_raises(Wreq::BuilderError) do
      Wreq.post(INVALID_URL, body: sender)
    end
    error = assert_raises(Wreq::MemoryError) do
      Wreq.post(INVALID_URL, body: sender)
    end

    assert_includes error.message, ":body"
  ensure
    sender&.close
  end

  def test_option_conversion_preserves_raised_exception_class
    source = Object.new
    source.define_singleton_method(:to_a) { raise IOError, "boom" }

    error = assert_raises(IOError) do
      Wreq::Client.new(headers: source)
    end

    assert_includes error.message, ":headers"
    assert_includes error.message, "boom"
  end

  def test_out_of_range_integer_names_the_option
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(timeout: 2**256)
    end

    assert_includes error.message, ":timeout"
  end

  def test_invalid_request_value_names_the_option_before_network_io
    error = assert_raises(TypeError) do
      Wreq.get(INVALID_URL, cookies: Object.new)
    end

    assert_includes error.message, ":cookies"
  end

  def test_client_constructor_rejects_non_hash_and_extra_arguments
    assert_raises(TypeError) { Wreq::Client.new("ignored") }
    assert_raises(ArgumentError) { Wreq::Client.new({}, {}) }
  end

  def test_emulation_constructor_rejects_invalid_arguments_and_options
    assert_raises(TypeError) { Wreq::Emulation.new("ignored") }
    assert_raises(ArgumentError) { Wreq::Emulation.new({}, {}) }

    error = assert_raises(ArgumentError) { Wreq::Emulation.new(unknown: true) }
    assert_includes error.message, ":unknown"
  end

  def test_invalid_emulation_values_name_the_option
    http2_error = assert_raises(TypeError) do
      Wreq::Emulation.new(http2: "yes")
    end
    profile_error = assert_raises(TypeError) do
      Wreq::Emulation.new(profile: Object.new)
    end

    assert_includes http2_error.message, ":http2"
    assert_includes profile_error.message, ":profile"
  end

  def test_body_options_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq.post(INVALID_URL, body: "raw", form: {a: 1}, json: {b: 2})
    end

    assert_includes error.message, ":body"
    assert_includes error.message, ":form"
    assert_includes error.message, ":json"
  end

  def test_body_conflict_does_not_consume_sender
    sender = Wreq::BodySender.new(1)

    assert_raises(ArgumentError) do
      Wreq.post(INVALID_URL, body: sender, json: {value: true})
    end
    assert_raises(Wreq::BuilderError) do
      Wreq.post(INVALID_URL, body: sender)
    end
  ensure
    sender&.close
  end

  def test_dependent_option_error_does_not_consume_sender
    sender = Wreq::BodySender.new(1)

    assert_raises(ArgumentError) do
      Wreq.post(INVALID_URL, body: sender, max_redirects: 2)
    end
    assert_raises(Wreq::BuilderError) do
      Wreq.post(INVALID_URL, body: sender)
    end
  ensure
    sender&.close
  end

  def test_authentication_options_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq.get(
        INVALID_URL,
        auth: "secret",
        bearer_auth: "secret",
        basic_auth: ["user", "secret"]
      )
    end

    assert_includes error.message, ":auth"
    assert_includes error.message, ":bearer_auth"
    assert_includes error.message, ":basic_auth"
    refute_includes error.message, "secret"
  end

  def test_protocol_only_options_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(http1_only: true, http2_only: true)
    end

    assert_includes error.message, ":http1_only"
    assert_includes error.message, ":http2_only"
  end

  def test_validation_reports_the_first_failed_rule
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(
        http1_only: true,
        http2_only: true,
        max_redirects: 2,
        headers: Object.new
      )
    end

    assert_includes error.message, ":http1_only"
    assert_includes error.message, ":http2_only"
    refute_includes error.message, ":max_redirects"
  end

  def test_explicit_proxy_and_no_proxy_are_mutually_exclusive
    error = assert_raises(ArgumentError) do
      Wreq::Client.new(proxy: "http://127.0.0.1:8080", no_proxy: true)
    end

    assert_includes error.message, ":proxy"
    assert_includes error.message, ":no_proxy"
  end

  def test_max_redirects_requires_redirects_to_be_enabled
    client_error = assert_raises(ArgumentError) do
      Wreq::Client.new(max_redirects: 2)
    end
    disabled_error = assert_raises(ArgumentError) do
      Wreq::Client.new(allow_redirects: false, max_redirects: 2)
    end
    request_error = assert_raises(ArgumentError) do
      Wreq.get(INVALID_URL, max_redirects: 2)
    end

    [client_error, disabled_error, request_error].each do |error|
      assert_includes error.message, ":max_redirects"
      assert_includes error.message, ":allow_redirects"
    end
  end

  def test_unsupported_platform_option_raises_argument_error
    return if RUBY_PLATFORM.match?(/linux|android|fuchsia/)

    error = assert_raises(ArgumentError) do
      Wreq::Client.new(tcp_user_timeout: 1)
    end
    assert_instance_of ArgumentError, error
    assert_includes error.message, ":tcp_user_timeout"
  end

  def test_windows_rejects_interface_for_client_and_request
    return unless Gem.win_platform?

    client_error = assert_raises(ArgumentError) do
      Wreq::Client.new(interface: "Ethernet")
    end
    request_error = assert_raises(ArgumentError) do
      Wreq.get(INVALID_URL, interface: "Ethernet")
    end

    assert_instance_of ArgumentError, client_error
    assert_instance_of ArgumentError, request_error
    assert_includes client_error.message, ":interface"
    assert_includes request_error.message, ":interface"
  end

  def test_valid_nil_and_zero_option_construction
    assert_instance_of Wreq::Client, Wreq::Client.new
    assert_instance_of Wreq::Client,
      Wreq::Client.new(proxy: nil, interface: nil, tcp_user_timeout: nil)
    assert_instance_of Wreq::Emulation, Wreq::Emulation.new(profile: nil, http2: true)
  end
end
