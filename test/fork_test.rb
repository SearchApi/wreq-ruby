# frozen_string_literal: true

require "test_helper"
require "open3"
require "rbconfig"
require "timeout"

class ForkTest < Minitest::Test
  def test_fork_error_is_a_runtime_error
    assert_operator Wreq::ForkError, :<, RuntimeError
  end

  def test_loaded_extension_is_rejected_after_fork
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    lib_dir = File.expand_path("../lib", __dir__)
    script = File.expand_path("scripts/fork_safety.rb", __dir__)

    stdout, stderr, status = Timeout.timeout(20) do
      Open3.capture3(RbConfig.ruby, "-I", lib_dir, script)
    end

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    %w[before_runtime fresh_client inherited_client].each do |label|
      assert_match(/#{label}=Wreq::ForkError:.*cannot be used after fork/, stderr)
      assert_match(/#{label}_retry=Wreq::ForkError:.*cannot be used after fork/, stderr)
    end
    refute_match(/\[BUG\]|segmentation fault|panicked/i, stderr)
  rescue Timeout::Error
    flunk "fork safety subprocess timed out"
  end
end
