# frozen_string_literal: true

require "socket"
require "timeout"
require "wreq"

$stdout.sync = true
$stderr.sync = true

def expect_fork_error(label)
  yield
  abort "#{label}=missing Wreq::ForkError"
rescue Wreq::ForkError => error
  warn "#{label}=#{error.class}: #{error.message}"
end

def run_child(label)
  child_pid = fork do
    Timeout.timeout(10) { yield }
    warn "#{label}=ok"
    exit! 0
  rescue => error
    warn "#{label}=unexpected #{error.class}: #{error.message}"
    exit! 2
  end

  _, status = Process.wait2(child_pid)
  abort "#{label} child failed with #{status.inspect}" unless status.success?
end

server = TCPServer.new("127.0.0.1", 0)
url = "http://127.0.0.1:#{server.addr[1]}/"
server_pid = fork do
  3.times do
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

# Requiring the extension is the only parent-side Wreq operation before this fork.
run_child("loaded_only") do
  abort "module request failed" unless Wreq.get(url).bytes == "ok"
end

inherited_client = Wreq::Client.new
inherited_sender = Wreq::BodySender.new
inherited_jar = Wreq::Jar.new

run_child("before_runtime") do
  expect_fork_error("inherited_client_before_runtime") do
    inherited_client.get(url)
  end
  expect_fork_error("inherited_sender_before_runtime") { inherited_sender.closed? }
  expect_fork_error("inherited_jar_before_runtime") { inherited_jar.get_all }
  expect_fork_error("inherited_cookie_provider_before_runtime") do
    Wreq::Client.new(cookie_provider: inherited_jar)
  end

  sender = Wreq::BodySender.new
  sender.push("child")

  jar = Wreq::Jar.new
  jar.add("child=1; Path=/", url)
  abort "child jar failed" unless jar.get_all.one?

  client = Wreq::Client.new(cookie_provider: jar)
  abort "client request failed" unless client.get(url).bytes == "ok"
end

Timeout.timeout(10) do
  abort "parent client failed" unless inherited_client.get(url).bytes == "ok"
  inherited_sender.push("parent")
  inherited_jar.add("parent=1; Path=/", url)
  abort "parent jar failed" unless inherited_jar.get_all.one?
end
warn "parent_after_children=ok"

_, server_status = Process.wait2(server_pid)
abort "server failed with #{server_status.inspect}" unless server_status.success?

puts "ok"
