# frozen_string_literal: true

require "test_helper"
require "rbconfig"
require "tempfile"
require "timeout"

class ForkTest < Minitest::Test
  FORK_ERROR_LABELS = %w[
    before_runtime
    invalid_client
    invalid_request
    fresh_body_sender
    inherited_body_sender_push
    inherited_body_sender_close
    inherited_body_sender_closed
    fresh_client
    inherited_client
    inherited_response
    inherited_response_text
    inherited_response_chunks
    inherited_response_close
  ].freeze

  def test_fork_error_is_a_runtime_error
    assert_operator Wreq::ForkError, :<, RuntimeError
  end

  def test_loaded_extension_is_rejected_after_fork
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    stdout, stderr, status = run_fork_script("fork_safety.rb")

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    FORK_ERROR_LABELS.each do |label|
      assert_match(/#{label}=Wreq::ForkError:.*cannot be used after fork/, stderr)
      assert_match(/#{label}_retry=Wreq::ForkError:.*cannot be used after fork/, stderr)
    end
    assert_match(/inherited_gc=ok/, stderr)
    refute_match(/\[BUG\]|segmentation fault|panicked/i, stderr)
  end

  def test_extension_can_be_loaded_after_fork
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    stdout, stderr, status = run_fork_script("fork_fresh_load.rb")

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    assert_empty stderr
  end

  private

  def run_fork_script(name)
    lib_dir = File.expand_path("../lib", __dir__)
    script = File.expand_path("scripts/#{name}", __dir__)

    Tempfile.create("wreq-fork-stdout") do |stdout|
      Tempfile.create("wreq-fork-stderr") do |stderr|
        pid = Process.spawn(
          RbConfig.ruby,
          "-I",
          lib_dir,
          script,
          out: stdout,
          err: stderr,
          pgroup: true
        )
        status = Timeout.timeout(30) { Process.wait2(pid).last }
        stdout.rewind
        stderr.rewind
        return [stdout.read, stderr.read, status]
      rescue Timeout::Error
        begin
          Process.kill("KILL", -pid)
        rescue Errno::ESRCH
          nil
        end

        begin
          Process.wait(pid)
        rescue Errno::ECHILD
          nil
        end

        flunk "#{name} timed out"
      end
    end
  end
end
