# frozen_string_literal: true

require "socket"
require "timeout"
require "weakref"
require "wreq"

$stdout.sync = true
$stderr.sync = true

def expect_fork_error(label)
  2.times do |attempt|
    attempt_label = attempt.zero? ? label : "#{label}_retry"

    begin
      yield
    rescue Wreq::ForkError => error
      warn "#{attempt_label}=#{error.class}: #{error.message}"
      next
    rescue => error
      abort "#{attempt_label}=unexpected #{error.class}: #{error.message}"
    end

    abort "#{attempt_label}=missing Wreq::ForkError"
  end
end

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
      socket.write("HTTP/1.1 200 OK\r\nX-Fork-Test: ok\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
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

# Start the server process before Tokio creates worker threads in the parent.
runtime_probe = Wreq::BodySender.new(1)
runtime_probe.push("warmup")

client = Wreq::Client.new
abort "parent warm-up failed" unless client.get(url).bytes == "ok"

inherited_objects = {
  client: Wreq::Client.new,
  sender: Wreq::BodySender.new,
  response: client.get(url),
  jar: Wreq::Jar.new
}
inherited_weak_refs = inherited_objects.values.map { |object| WeakRef.new(object) }
status_snapshot = inherited_objects[:response].status
headers_snapshot = inherited_objects[:response].headers

guard_pid = fork do
  Timeout.timeout(10) do
    jar = Wreq::Jar.new
    jar.add("child=1; Path=/", url)
    abort "fresh child jar failed" unless jar.get_all.one?

    Wreq::Client.new(cookie_provider: jar)
    Wreq::BodySender.new
    warn "non_runtime_construction=ok"

    abort "status snapshot changed" unless status_snapshot.to_i == 200
    abort "headers snapshot changed" unless headers_snapshot["X-Fork-Test"] == "ok"
    warn "inherited_snapshots=ok"

    expect_fork_error("module_request") { Wreq.get(url) }
    expect_fork_error("fresh_client_request") { Wreq::Client.new.get(url) }
    expect_fork_error("fresh_body_sender_push") { Wreq::BodySender.new.push("chunk") }
    expect_fork_error("inherited_body_sender_push") do
      inherited_objects[:sender].push("chunk")
    end
    expect_fork_error("inherited_body_sender_close") { inherited_objects[:sender].close }
    expect_fork_error("inherited_body_sender_closed") { inherited_objects[:sender].closed? }
    expect_fork_error("inherited_client") { inherited_objects[:client].get(url) }
    expect_fork_error("inherited_jar") { inherited_objects[:jar].get_all }
    expect_fork_error("inherited_cookie_provider") do
      Wreq::Client.new(cookie_provider: inherited_objects[:jar])
    end
    expect_fork_error("inherited_response_metadata") { inherited_objects[:response].status }
    expect_fork_error("inherited_response") { inherited_objects[:response].bytes }
    expect_fork_error("inherited_response_text") { inherited_objects[:response].text }
    expect_fork_error("inherited_response_chunks") { inherited_objects[:response].chunks { nil } }
    expect_fork_error("inherited_response_close") { inherited_objects[:response].close }
  end
  exit! 0
rescue => error
  warn "guard_checks=unexpected #{error.class}: #{error.message}"
  exit! 2
end
_, guard_status = Process.wait2(guard_pid)
abort "guard checks child failed with #{guard_status.inspect}" unless guard_status.success?

# The hash is now the only strong reference to the inherited native objects.
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
