# frozen_string_literal: true

require "test_helper"
require "open3"
require "rbconfig"

class CookieTest < Minitest::Test
  def setup
    @jar = Wreq::Jar.new
    @base_url = "https://example.com"
  end

  def test_jar_initially_empty
    assert_instance_of Wreq::Jar, @jar
    cookies = begin
      Wreq::Jar.get_all(@jar)
    rescue
      @jar.get_all
    end # support either binding style
    assert_kind_of Array, cookies
    assert_equal 0, cookies.length
  end

  def test_add_and_get_all
    set_cookie = "sid=abc123; Path=/; Domain=example.com; HttpOnly; Secure"
    @jar.add(set_cookie, @base_url)

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
    @jar.add("a=1; Path=/", @base_url)
    @jar.add("b=2; Path=/", @base_url)
    @jar.add("c=3; Path=/", @base_url)

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
    @jar.add("x=1; Path=/", @base_url)
    @jar.add("y=2; Path=/", @base_url)
    refute_empty @jar.get_all

    @jar.clear
    assert_empty @jar.get_all
  end

  def test_max_age_and_expires_optional
    # Max-Age only
    @jar.clear
    @jar.add("ma=1; Max-Age=3600; Path=/", @base_url)
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
    @jar.add("exp=1; Expires=#{t.gmtime.strftime("%a, %d %b %Y %H:%M:%S GMT")}; Path=/", @base_url)
    c2 = @jar.get_all.find { |c| c.name == "exp" }
    assert c2
    # expires_at returns Time and expires retains the numeric compatibility API
    assert_kind_of Time, c2.expires_at
    assert_predicate c2.expires_at, :utc?
    if (e = c2.expires)
      assert_kind_of Float, e
      assert_operator e, :>, Time.now.to_f - 1_000_000 # sanity bound
    end
  end

  # -------- Wreq::Cookie unit tests --------

  def test_cookie_new_minimal
    c = Wreq::Cookie.new("sid", "abc")

    assert_instance_of Wreq::Cookie, c
    assert_equal "sid", c.name
    assert_equal "abc", c.value

    assert_nil c.path
    assert_nil c.domain
    assert_nil c.max_age
    assert_nil c.expires
    assert_nil c.expires_at

    assert_equal false, (c.http_only || c.http_only?)
    assert_equal false, (c.secure || c.secure?)
    assert_equal false, c.same_site_lax?
    assert_equal false, c.same_site_strict?
  end

  def test_cookie_new_full_attributes
    exp = Time.utc(2030, 1, 1, 0, 0, Rational(123_456_789, 1_000_000_000))
    c = Wreq::Cookie.new("sess", "v",
      domain: "example.com",
      path: "/",
      max_age: 3600,
      expires: exp,
      http_only: true,
      secure: true,
      same_site: Wreq::SameSite::Lax)

    assert_equal "sess", c.name
    assert_equal "v", c.value
    assert_equal "example.com", c.domain
    assert_equal "/", c.path

    # Max-Age returns seconds as Integer
    assert_equal 3600, c.max_age

    assert_equal exp, c.expires_at
    assert_predicate c.expires_at, :utc?
    assert_in_delta exp.to_f, c.expires, 1e-6

    assert_equal true, (c.http_only || c.http_only?)
    assert_equal true, (c.secure || c.secure?)
    assert_equal true, c.same_site_lax?
    assert_equal false, c.same_site_strict?
  end

  def test_cookie_new_uses_shared_keyword_validation
    cookie = Wreq::Cookie.new("sid", "abc", **{"path" => "/"})
    assert_equal "/", cookie.path

    secret = "must-not-appear"
    unknown_error = assert_raises(ArgumentError) do
      Wreq::Cookie.new("sid", "abc", domian: secret)
    end
    assert_includes unknown_error.message, ":domian"
    refute_includes unknown_error.message, secret

    duplicate_options = {path: "/one"}
    duplicate_options["path"] = "/two"
    duplicate_error = assert_raises(ArgumentError) do
      Wreq::Cookie.new("sid", "abc", **duplicate_options)
    end
    assert_includes duplicate_error.message, "duplicate option: :path"
  end

  def test_expires_accepts_past_time
    expiration = Time.at(Rational(-5, 4)).utc
    cookie = Wreq::Cookie.new("past", "value", expires: expiration)

    assert_equal expiration, cookie.expires_at
    assert_in_delta(-1.25, cookie.expires, 1e-9)
  end

  def test_expires_accepts_integer_and_fractional_timestamps
    [1_893_456_000, 1_893_456_000.125, -1.25].each do |timestamp|
      cookie = Wreq::Cookie.new("timestamp", "value", expires: timestamp)

      assert_kind_of Time, cookie.expires_at
      assert_predicate cookie.expires_at, :utc?
      assert_in_delta timestamp, cookie.expires_at.to_f, 1e-6
      assert_in_delta timestamp, cookie.expires, 1e-6
    end
  end

  def test_expires_rejects_non_finite_timestamps
    [Float::NAN, Float::INFINITY, -Float::INFINITY].each do |timestamp|
      error = assert_raises(ArgumentError) do
        Wreq::Cookie.new("invalid", "value", expires: timestamp)
      end

      assert_match(/expires.*finite/, error.message)
    end
  end

  def test_expires_rejects_unrepresentable_times
    expirations = [
      -(2**63),
      2**63 - 1,
      -Float::MAX,
      Float::MAX,
      Time.utc(10_000, 1, 1)
    ]

    expirations.each do |expiration|
      error = assert_raises(RangeError) do
        Wreq::Cookie.new("invalid", "value", expires: expiration)
      end

      assert_match(/expires.*supported range/, error.message)
    end
  end

  def test_max_age_accepts_signed_boundaries_without_wrapping
    [-(2**63), -1, 0, 2**63 - 1].each do |max_age|
      cookie = Wreq::Cookie.new("max-age", "value", max_age: max_age)

      assert_equal max_age, cookie.max_age
    end

    [-(2**63) - 1, 2**63].each do |max_age|
      assert_raises(RangeError) do
        Wreq::Cookie.new("max-age", "value", max_age: max_age)
      end
    end
  end

  def test_non_positive_max_age_removes_cookie_from_jar
    [-1, 0].each do |max_age|
      jar = Wreq::Jar.new
      jar.add("session=old; Path=/", @base_url)
      deletion = Wreq::Cookie.new("session", "gone", path: "/", max_age: max_age)

      jar.add(deletion, @base_url)

      assert_equal max_age, deletion.max_age
      assert_empty jar.get_all
    end
  end

  def test_past_expiration_removes_cookie_from_jar
    @jar.add("session=old; Path=/", @base_url)
    deletion = Wreq::Cookie.new(
      "session",
      "gone",
      path: "/",
      expires: Time.at(-1).utc
    )

    @jar.add(deletion, @base_url)

    assert_empty @jar.get_all
  end

  def test_expiration_regressions_exit_subprocess_normally
    lib_dir = File.expand_path("../lib", __dir__)
    script = <<~RUBY
      require "wreq"

      past = Wreq::Cookie.new("past", "value", expires: -1.0)
      abort "past timestamp was not retained" unless past.expires_at == Time.at(-1).utc

      [Float::NAN, Float::INFINITY, -Float::INFINITY].each do |timestamp|
        begin
          Wreq::Cookie.new("invalid", "value", expires: timestamp)
        rescue ArgumentError
          next
        end

        abort "non-finite timestamp did not raise ArgumentError"
      end

      [Float::MAX, -Float::MAX].each do |timestamp|
        begin
          Wreq::Cookie.new("invalid", "value", expires: timestamp)
        rescue RangeError
          next
        end

        abort "unrepresentable finite timestamp did not raise RangeError"
      end

      [-(2**63) - 1, 2**63].each do |max_age|
        begin
          Wreq::Cookie.new("invalid", "value", max_age: max_age)
        rescue RangeError
          next
        end

        abort "out-of-range Max-Age did not raise RangeError"
      end

      puts "ok"
    RUBY

    stdout, stderr, status = Open3.capture3(RbConfig.ruby, "-I", lib_dir, "-e", script)

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    refute_match(/panicked|fatal|access violation|cannot convert float seconds to Duration/i, stderr)
  end

  def test_same_site_flags_from_parsed_header
    @jar.clear
    @jar.add("s1=1; Path=/; SameSite=Strict", @base_url)
    @jar.add("s2=1; Path=/; SameSite=Lax", @base_url)

    cookies = @jar.get_all
    h = cookies.to_h { |ck| [ck.name, [ck.same_site_strict?, ck.same_site_lax?]] }

    assert_equal [true, false], h["s1"]
    assert_equal [false, true], h["s2"]
  end

  def test_request_uncompressed_cookies
    client = Wreq::Client.new
    resp = client.get(
      "#{HTTPBIN_URL}/cookies",
      cookies: {"foo" => "bar", "baz" => "qux"}
    )
    json = resp.json
    assert_instance_of Hash, json
    cookies = json.fetch("cookies", json)
    assert_equal "bar", cookies["foo"]
    assert_equal "qux", cookies["baz"]
  end

  def test_request_compressed_cookies
    client = Wreq::Client.new
    resp = client.get(
      "#{HTTPBIN_URL}/cookies",
      cookies: "foo=bar; baz=qux"
    )
    json = resp.json
    assert_instance_of Hash, json
    cookies = json.fetch("cookies", json)
    assert_equal "bar", cookies["foo"]
    assert_equal "qux", cookies["baz"]
  end
end
