# frozen_string_literal: true

require "test_helper"

# Enum binding tests for Rust-backed enums exposed as Ruby classes.
class EnumsTest < Minitest::Test
  def test_wreq_module_defined
    assert defined?(Wreq), "Wreq module should be defined"
  end

  def test_method_constants_exist
    %i[GET POST PUT DELETE PATCH HEAD TRACE OPTIONS].each do |name|
      assert Wreq::Method.const_defined?(name), "Wreq::Method::#{name} should be defined"
    end
  end

  def test_version_constants_exist
    %i[HTTP_09 HTTP_10 HTTP_11 HTTP_2 HTTP_3].each do |name|
      assert Wreq::Version.const_defined?(name), "Wreq::Version::#{name} should be defined"
    end
  end

  def test_method_instances_have_correct_class
    get = Wreq::Method::GET
    post = Wreq::Method::POST

    assert_equal Wreq::Method, get.class
    assert_equal Wreq::Method, post.class
    refute_equal get.object_id, post.object_id, "Different variants should be distinct objects"
  end

  def test_version_instances_have_correct_class
    v11 = Wreq::Version::HTTP_11
    v2  = Wreq::Version::HTTP_2

    assert_equal Wreq::Version, v11.class
    assert_equal Wreq::Version, v2.class
    refute_equal v11.object_id, v2.object_id, "Different variants should be distinct objects"
  end

  def test_constants_not_nil
    assert Wreq::Method::GET
    assert Wreq::Version::HTTP_11
  end
end
