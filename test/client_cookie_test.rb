# frozen_string_literal: true

require "test_helper"

class ClientCookieProviderTest < Minitest::Test
  HOST = HTTPBIN_URL

  def setup
    @jar = Wreq::Jar.new
    @client = Wreq::Client.new(
      cookie_store: true,
      cookie_provider: @jar,
      allow_redirects: true
    )
  end

  def test_custom_jar_captures_set_cookie_and_sends_back
    # starts empty
    assert_kind_of Array, @jar.get_all
    assert_equal 0, @jar.get_all.length

    # server sets cookie; client follows redirect; jar should store it
    res1 = @client.get("#{HOST}/cookies/set?foo=bar")
    assert_equal 200, res1.code

    names = @jar.get_all.map(&:name)
    assert_includes names, "foo"

    # subsequent request should send the stored cookie automatically
    res2 = @client.get("#{HOST}/cookies")
    assert_equal 200, res2.code
    cookies = httpbin_cookies(res2.json)
    assert_kind_of Hash, cookies
    assert_equal "bar", httpbin_fetch(cookies, "foo")
  end

  def test_prepopulated_jar_is_used_by_client
    # pre-populate jar
    @jar.add("pref=1; Path=/", "#{HOST}/")

    res = @client.get("#{HOST}/cookies")
    assert_equal 200, res.code
    cookies = httpbin_cookies(res.json)
    assert_equal "1", httpbin_fetch(cookies, "pref")
  end
end
