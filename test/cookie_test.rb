# frozen_string_literal: true

require "test_helper"

class CookieTest < Minitest::Test
  def setup
    @jar = Wreq::Jar.new
    @base_url = "https://example.com"
  end

  def test_jar_initially_empty
    assert_instance_of Wreq::Jar, @jar
    cookies = Wreq::Jar.get_all(@jar) rescue @jar.get_all # support either binding style
    assert_kind_of Array, cookies
    assert_equal 0, cookies.length
  end

  def test_add_cookie_str_and_get_all
    set_cookie = "sid=abc123; Path=/; Domain=example.com; HttpOnly; Secure"
    @jar.add_cookie_str(set_cookie, @base_url)

    cookies = @jar.get_all
    assert_kind_of Array, cookies
    assert_equal 1, cookies.length

    c = cookies.first
    assert_instance_of Wreq::Cookie, c
    assert_equal "sid", c.name
    assert_equal "abc123", c.value

    # attributes parsed from Set-Cookie
    assert_equal "/", c.path
    assert_equal "example.com", c.domain

    # predicate-ish flags
    assert_equal true, (c.http_only || c.http_only?)
    assert_equal true, (c.secure || c.secure?)
  end

  def test_add_multiple_and_remove
    @jar.add_cookie_str("a=1; Path=/", @base_url)
    @jar.add_cookie_str("b=2; Path=/", @base_url)
    @jar.add_cookie_str("c=3; Path=/", @base_url)

    cookies = @jar.get_all
    assert_equal 3, cookies.length

    # remove one by name
    @jar.remove("b", @base_url)
    names = @jar.get_all.map(&:name)
    refute_includes names, "b"
    assert_includes names, "a"
    assert_includes names, "c"
  end

  def test_clear
    @jar.add_cookie_str("x=1; Path=/", @base_url)
    @jar.add_cookie_str("y=2; Path=/", @base_url)
    refute_empty @jar.get_all

    @jar.clear
    assert_empty @jar.get_all
  end

  def test_max_age_and_expires_optional
    # Max-Age only
    @jar.clear
    @jar.add_cookie_str("ma=1; Max-Age=3600; Path=/", @base_url)
    c1 = @jar.get_all.find { |c| c.name == "ma" }
    assert c1
    # can be nil or Integer; just ensure responds and is truthy integer
    if (v = c1.max_age)
      assert_kind_of Integer, v
      assert_operator v, :>=, 0
    end

    # Expires only
    @jar.clear
    t = Time.now + 3600
    @jar.add_cookie_str("exp=1; Expires=#{t.gmtime.strftime('%a, %d %b %Y %H:%M:%S GMT')}; Path=/", @base_url)
    c2 = @jar.get_all.find { |c| c.name == "exp" }
    assert c2
    # expires returns Float (unix seconds) or nil
    if (e = c2.expires)
      assert_kind_of Float, e
      assert_operator e, :>, Time.now.to_f - 1_000_000 # sanity bound
    end
  end

  # -------- Wreq::Cookie unit tests --------

  def test_cookie_new_minimal
    c = Wreq::Cookie.new("sid", "abc", nil, nil, nil, nil, nil, nil)

    assert_instance_of Wreq::Cookie, c
    assert_equal "sid", c.name
    assert_equal "abc", c.value

    assert_nil c.path
    assert_nil c.domain
    assert_nil c.max_age
    assert_nil c.expires

    assert_equal false, (c.http_only || c.http_only?)
    assert_equal false, (c.secure || c.secure?)
    assert_equal false, c.same_site_lax?
    assert_equal false, c.same_site_strict?
  end

  def test_cookie_new_full_attributes
    exp = Time.now.to_f + 7200.0
    c = Wreq::Cookie.new("sess", "v", "example.com", "/", 3600, exp, true, true)

    assert_equal "sess", c.name
    assert_equal "v", c.value
    assert_equal "example.com", c.domain
    assert_equal "/", c.path

    # Max-Age returns seconds as Integer
    assert_equal 3600, c.max_age

    # Expires returns Float seconds-since-epoch (with small tolerance)
    assert c.expires
    assert_kind_of Float, c.expires
    assert_in_delta exp, c.expires, 2.0

    assert_equal true, (c.http_only || c.http_only?)
    assert_equal true, (c.secure || c.secure?)
    # constructor currently sets SameSite to none
    assert_equal false, c.same_site_lax?
    assert_equal false, c.same_site_strict?
  end

  def test_same_site_flags_from_parsed_header
    @jar.clear
    @jar.add_cookie_str("s1=1; Path=/; SameSite=Strict", @base_url)
    @jar.add_cookie_str("s2=1; Path=/; SameSite=Lax", @base_url)

    cookies = @jar.get_all
    h = cookies.to_h { |ck| [ck.name, [ck.same_site_strict?, ck.same_site_lax?]] }

    assert_equal [true, false], h["s1"]
    assert_equal [false, true], h["s2"]
  end
end
