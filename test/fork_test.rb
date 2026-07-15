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
    script = <<~'RUBY'
      require "socket"
      require "timeout"
      require "wreq"

      $stdout.sync = true
      $stderr.sync = true

      def expect_fork_error(label)
        child_pid = fork do
          begin
            Timeout.timeout(5) { yield }
          rescue Wreq::ForkError => error
            warn "#{label}=#{error.class}: #{error.message}"
            exit! 0
          rescue Exception => error
            warn "#{label}=unexpected #{error.class}: #{error.message}"
            exit! 2
          end

          warn "#{label}=missing Wreq::ForkError"
          exit! 3
        end

        _, status = Process.wait2(child_pid)
        abort "#{label} child failed with #{status.inspect}" unless status.success?
      end

      expect_fork_error("before_runtime") { Wreq::Client.new }

      server = TCPServer.new("127.0.0.1", 0)
      port = server.addr[1]
      server_pid = fork do
        2.times do
          ready = IO.select([server], nil, nil, 10)
          exit! 4 unless ready

          socket = server.accept
          begin
            while (line = socket.gets)
              break if line == "\r\n"
            end
            socket.write("HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
          ensure
            socket.close
          end
        end
        exit! 0
      ensure
        server.close
      end
      server.close

      url = "http://127.0.0.1:#{port}/"
      client = Wreq::Client.new
      abort "parent warm-up failed" unless client.get(url).bytes == "ok"

      expect_fork_error("fresh_client") { Wreq::Client.new }
      expect_fork_error("inherited_client") { client.get(url) }

      abort "parent request after fork failed" unless client.get(url).bytes == "ok"
      _, server_status = Process.wait2(server_pid)
      abort "server failed with #{server_status.inspect}" unless server_status.success?

      puts "ok"
    RUBY

    stdout, stderr, status = Timeout.timeout(20) do
      Open3.capture3(RbConfig.ruby, "-I", lib_dir, "-e", script)
    end

    assert status.success?, "subprocess failed with #{status.inspect}: #{stderr}"
    assert_equal "ok\n", stdout
    assert_match(/before_runtime=Wreq::ForkError:.*cannot be used after fork/, stderr)
    assert_match(/fresh_client=Wreq::ForkError:.*cannot be used after fork/, stderr)
    assert_match(/inherited_client=Wreq::ForkError:.*cannot be used after fork/, stderr)
    refute_match(/\[BUG\]|segmentation fault|panicked/i, stderr)
  rescue Timeout::Error
    flunk "fork safety subprocess timed out"
  end
end
