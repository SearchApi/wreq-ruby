# frozen_string_literal: true

require "test_helper"
require "rbconfig"
require "tempfile"
require "timeout"

class ForkTest < Minitest::Test
  FORK_ERROR_LABELS = %w[
    module_request
    fresh_client_request
    fresh_body_sender_push
    inherited_body_sender_push
    inherited_body_sender_close
    inherited_body_sender_closed
    inherited_client
    inherited_jar
    inherited_cookie_provider
    inherited_response_metadata
    inherited_response
    inherited_response_text
    inherited_response_chunks
    inherited_response_close
  ].freeze

  def test_fork_error_is_a_runtime_error
    assert_operator Wreq::ForkError, :<, RuntimeError
  end

  def test_loaded_extension_can_initialize_runtime_after_fork
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    stdout, stderr, status = run_fork_script("prefork_runtime.rb")

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    assert_match(/loaded_only=ok/, stderr)
    assert_match(/before_runtime=ok/, stderr)
    assert_match(/parent_after_children=ok/, stderr)
    %w[
      inherited_client_before_runtime
      inherited_sender_before_runtime
      inherited_jar_before_runtime
      inherited_cookie_provider_before_runtime
    ].each do |label|
      assert_match(/#{label}=Wreq::ForkError:.*cannot be used after fork/, stderr)
    end
    refute_match(/\[BUG\]|segmentation fault|panicked/i, stderr)
  end

  def test_fresh_clients_can_request_from_concurrent_forked_workers
    skip "this regression test is Linux-only" unless RbConfig::CONFIG.fetch("host_os").include?("linux")
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    stdout, stderr, status = run_fork_script("multiprocess_client.rb", timeout: 60)

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    4.times do |worker|
      assert_match(/^worker_#{worker}=ok$/, stderr)
    end
    assert_match(/^parent_after_workers=ok$/, stderr)
    refute_match(/Wreq::ForkError|\[BUG\]|segmentation fault|panicked/i, stderr)
  end

  def test_initialized_runtime_is_rejected_after_fork
    skip "fork is not supported on this platform" unless Process.respond_to?(:fork)

    stdout, stderr, status = run_fork_script("fork_safety.rb")

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    assert_match(/non_runtime_construction=ok/, stderr)
    assert_match(/inherited_snapshots=ok/, stderr)
    FORK_ERROR_LABELS.each do |label|
      assert_match(/#{label}=Wreq::ForkError:.*cannot be used after fork/, stderr)
      assert_match(/#{label}_retry=Wreq::ForkError:.*cannot be used after fork/, stderr)
    end
    assert_match(/inherited_gc=ok/, stderr)
    refute_match(/\[BUG\]|segmentation fault|panicked/i, stderr)
  end

  private

  def run_fork_script(name, timeout: 30)
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
        status = Timeout.timeout(timeout) { Process.wait2(pid).last }
        kill_process_group(pid) unless status.success?
        stdout.rewind
        stderr.rewind
        return [stdout.read, stderr.read, status]
      rescue Timeout::Error
        kill_process_group(pid)
        flunk "#{name} timed out"
      end
    end
  end

  def kill_process_group(pid)
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
  end
end
