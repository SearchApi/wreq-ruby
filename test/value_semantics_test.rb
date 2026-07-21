# frozen_string_literal: true

require "test_helper"

class ValueSemanticsTest < Minitest::Test
  # ---- StatusCode ----

  def setup
    @response = Wreq.get("#{HTTPBIN_URL}/status/201")
  end

  def test_status_code_to_i
    assert_equal 201, @response.status.to_i
  end

  def test_status_code_as_int_still_works
    assert_equal 201, @response.status.as_int
  end

  def test_status_code_to_i_matches_as_int
    assert_equal @response.status.as_int, @response.status.to_i
  end

  def test_status_code_equality
    a = @response.status
    b = @response.status
    assert_equal a, b
  end

  def test_status_code_eql
    a = @response.status
    b = @response.status
    assert a.eql?(b)
  end

  def test_status_code_hash_consistent
    a = @response.status
    b = @response.status
    assert_equal a.hash, b.hash
  end

  def test_status_code_as_hash_key
    status = @response.status
    h = {status => "created"}
    assert_equal "created", h[@response.status]
  end

  def test_status_code_in_set
    s = Set.new
    s.add(@response.status)
    assert_includes s, @response.status
  end

  def test_status_code_not_equal_to_integer
    refute_equal @response.status, 201
    refute @response.status.eql?(201)
  end

  def test_status_code_different_values_not_equal
    ok = Wreq.get("#{HTTPBIN_URL}/status/200").status
    created = @response.status
    refute_equal ok, created
    refute ok.eql?(created)
  end

  def test_response_code_still_returns_integer
    assert_instance_of Integer, @response.code
    assert_equal 201, @response.code
  end

  # ---- Version ----

  def test_version_equality
    assert_equal Wreq::Version::HTTP_11, Wreq::Version::HTTP_11
    assert_equal "HTTP/0.9", Wreq::Version::HTTP_09.to_s
    assert_equal "HTTP/1.0", Wreq::Version::HTTP_10.to_s
    assert_equal "HTTP/1.1", Wreq::Version::HTTP_11.to_s
    assert_equal "HTTP/2.0", Wreq::Version::HTTP_2.to_s
    assert_equal "HTTP/3.0", Wreq::Version::HTTP_3.to_s
  end

  def test_version_eql
    v = Wreq::Version::HTTP_11
    assert v.eql?(Wreq::Version::HTTP_11)
  end

  def test_version_hash_consistent
    a = Wreq::Version::HTTP_11
    b = Wreq::Version::HTTP_11
    assert_equal a.hash, b.hash
  end

  def test_version_different_values_not_equal
    refute_equal Wreq::Version::HTTP_11, Wreq::Version::HTTP_2
  end

  def test_version_as_hash_key
    v = Wreq::Version::HTTP_11
    h = {v => "http1.1"}
    assert_equal "http1.1", h[Wreq::Version::HTTP_11]
  end

  def test_version_in_set
    s = Set.new([Wreq::Version::HTTP_11, Wreq::Version::HTTP_2])
    assert_includes s, Wreq::Version::HTTP_11
    assert_includes s, Wreq::Version::HTTP_2
    refute_includes s, Wreq::Version::HTTP_3
  end

  # ---- Method ----

  def test_method_to_s
    assert_equal "GET", Wreq::Method::GET.to_s
    assert_equal "POST", Wreq::Method::POST.to_s
    assert_equal "PUT", Wreq::Method::PUT.to_s
    assert_equal "DELETE", Wreq::Method::DELETE.to_s
    assert_equal "HEAD", Wreq::Method::HEAD.to_s
    assert_equal "OPTIONS", Wreq::Method::OPTIONS.to_s
    assert_equal "TRACE", Wreq::Method::TRACE.to_s
    assert_equal "PATCH", Wreq::Method::PATCH.to_s
  end

  def test_method_to_sym
    assert_equal :get, Wreq::Method::GET.to_sym
    assert_equal :post, Wreq::Method::POST.to_sym
    assert_equal :put, Wreq::Method::PUT.to_sym
    assert_equal :delete, Wreq::Method::DELETE.to_sym
    assert_equal :head, Wreq::Method::HEAD.to_sym
    assert_equal :options, Wreq::Method::OPTIONS.to_sym
    assert_equal :trace, Wreq::Method::TRACE.to_sym
    assert_equal :patch, Wreq::Method::PATCH.to_sym
  end

  def test_method_equality
    assert_equal Wreq::Method::GET, Wreq::Method::GET
    refute_equal Wreq::Method::GET, Wreq::Method::POST
  end

  def test_method_eql
    assert Wreq::Method::GET.eql?(Wreq::Method::GET)
    refute Wreq::Method::GET.eql?(Wreq::Method::POST)
  end

  def test_method_hash_consistent
    assert_equal Wreq::Method::GET.hash, Wreq::Method::GET.hash
    refute_equal Wreq::Method::GET.hash, Wreq::Method::POST.hash
  end

  def test_method_as_hash_key
    h = {Wreq::Method::GET => "get it"}
    assert_equal "get it", h[Wreq::Method::GET]
    assert_nil h[Wreq::Method::POST]
  end

  # ---- SameSite ----

  def test_same_site_to_s
    assert_equal "Strict", Wreq::SameSite::Strict.to_s
    assert_equal "Lax", Wreq::SameSite::Lax.to_s
    assert_equal "None", Wreq::SameSite::None.to_s
  end

  def test_same_site_to_sym
    assert_equal :strict, Wreq::SameSite::Strict.to_sym
    assert_equal :lax, Wreq::SameSite::Lax.to_sym
    assert_equal :none, Wreq::SameSite::None.to_sym
  end

  def test_same_site_equality
    assert_equal Wreq::SameSite::Lax, Wreq::SameSite::Lax
    refute_equal Wreq::SameSite::Lax, Wreq::SameSite::Strict
  end

  def test_same_site_eql_and_hash
    assert Wreq::SameSite::Lax.eql?(Wreq::SameSite::Lax)
    assert_equal Wreq::SameSite::Lax.hash, Wreq::SameSite::Lax.hash
  end

  # ---- Profile ----

  def test_profile_equality
    assert_equal Wreq::Profile::Chrome134, Wreq::Profile::Chrome134
    refute_equal Wreq::Profile::Chrome134, Wreq::Profile::Chrome135
    assert_equal "Chrome134", Wreq::Profile::Chrome134.to_s
    assert_equal "SafariIos17_4_1", Wreq::Profile::SafariIos17_4_1.to_s
    assert_equal "OkHttp4_12", Wreq::Profile::OkHttp4_12.to_s
  end

  def test_profile_eql_and_hash
    assert Wreq::Profile::Chrome134.eql?(Wreq::Profile::Chrome134)
    assert_equal Wreq::Profile::Chrome134.hash, Wreq::Profile::Chrome134.hash
  end

  def test_profile_as_hash_key
    h = {Wreq::Profile::Chrome134 => "chrome"}
    assert_equal "chrome", h[Wreq::Profile::Chrome134]
    assert_nil h[Wreq::Profile::Chrome135]
  end

  # ---- Platform ----

  def test_platform_equality
    assert_equal Wreq::Platform::Windows, Wreq::Platform::Windows
    refute_equal Wreq::Platform::Windows, Wreq::Platform::Linux
  end

  def test_platform_eql_and_hash
    assert Wreq::Platform::Windows.eql?(Wreq::Platform::Windows)
    assert_equal Wreq::Platform::Windows.hash, Wreq::Platform::Windows.hash
  end

  def test_platform_to_sym
    assert_equal "Windows", Wreq::Platform::Windows.to_s
    assert_equal "MacOS", Wreq::Platform::MacOS.to_s
    assert_equal "Linux", Wreq::Platform::Linux.to_s
    assert_equal "Android", Wreq::Platform::Android.to_s
    assert_equal "IOS", Wreq::Platform::IOS.to_s

    assert_equal :windows, Wreq::Platform::Windows.to_sym
    assert_equal :macos, Wreq::Platform::MacOS.to_sym
    assert_equal :linux, Wreq::Platform::Linux.to_sym
    assert_equal :android, Wreq::Platform::Android.to_sym
    assert_equal :ios, Wreq::Platform::IOS.to_sym
  end

  # ---- Cross-type comparisons ----

  def test_cross_type_not_equal
    refute_equal Wreq::Method::GET, Wreq::Version::HTTP_11
    refute_equal Wreq::SameSite::Lax, Wreq::Method::GET
    refute_equal Wreq::Platform::Windows, Wreq::Profile::Chrome134
  end

  def test_cross_type_eql_false
    refute Wreq::Method::GET.eql?(Wreq::Version::HTTP_11)
    refute Wreq::SameSite::Lax.eql?(Wreq::Method::GET)
  end
end
