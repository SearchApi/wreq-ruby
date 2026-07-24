# frozen_string_literal: true

require "socket"
require "timeout"
require "weakref"
require "wreq"

$stdout.sync = true
$stderr.sync = true

def expect_fork_error(label)
  child_pid = fork do
    2.times do |attempt|
      attempt_label = attempt.zero? ? label : "#{label}_retry"

      begin
        Timeout.timeout(5) { yield }
      rescue Wreq::ForkError => error
        warn "#{attempt_label}=#{error.class}: #{error.message}"
        next
      rescue => error
        warn "#{attempt_label}=unexpected #{error.class}: #{error.message}"
        exit! 2
      end

      warn "#{attempt_label}=missing Wreq::ForkError"
      exit! 3
    end

    exit! 0
  end

  _, status = Process.wait2(child_pid)
  abort "#{label} child failed with #{status.inspect}" unless status.success?
end

expect_fork_error("before_runtime") { Wreq::Client.new }
expect_fork_error("invalid_client") { Wreq::Client.new(unknown: true) }
expect_fork_error("invalid_request") { Wreq.get(1) }
expect_fork_error("fresh_body_sender") { Wreq::BodySender.new(0) }

server = TCPServer.new("127.0.0.1", 0)
port = server.addr[1]
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

url = "http://127.0.0.1:#{port}/"
client = Wreq::Client.new
abort "parent warm-up failed" unless client.get(url).bytes == "ok"

def build_inherited_objects(client, url)
  [Wreq::Client.new, Wreq::BodySender.new, client.get(url)]
end

inherited_objects = build_inherited_objects(client, url)
inherited_weak_refs = inherited_objects.map { |object| WeakRef.new(object) }

expect_fork_error("inherited_body_sender_push") do
  inherited_objects[1].push("chunk")
end
expect_fork_error("inherited_body_sender_close") { inherited_objects[1].close }
expect_fork_error("inherited_body_sender_closed") { inherited_objects[1].closed? }
expect_fork_error("fresh_client") { Wreq::Client.new }
expect_fork_error("inherited_client") { client.get(url) }
expect_fork_error("inherited_response") { inherited_objects[2].bytes }
expect_fork_error("inherited_response_status") { inherited_objects[2].raise_for_status! }
expect_fork_error("inherited_response_text") { inherited_objects[2].text(1) }
expect_fork_error("inherited_response_chunks") { inherited_objects[2].chunks }
expect_fork_error("inherited_response_close") { inherited_objects[2].close }

# Release the earlier test blocks so this array is the only strong reference.
GC.start(full_mark: true, immediate_sweep: true)
gc_pid = fork do
  inherited_objects = nil
  3.times { GC.start(full_mark: true, immediate_sweep: true) }
  alive = inherited_weak_refs.each_index.select do |index|
    inherited_weak_refs[index].weakref_alive?
  end
  abort "inherited objects were not collected: #{alive.join(", ")}" unless alive.empty?
  warn "inherited_gc=ok"
  exit! 0
rescue => error
  warn "inherited_gc=unexpected #{error.class}: #{error.message}"
  exit! 5
end
_, gc_status = Process.wait2(gc_pid)
abort "inherited GC child failed with #{gc_status.inspect}" unless gc_status.success?

abort "parent request after fork failed" unless client.get(url).bytes == "ok"
_, server_status = Process.wait2(server_pid)
abort "server failed with #{server_status.inspect}" unless server_status.success?

puts "ok"
