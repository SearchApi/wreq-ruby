# frozen_string_literal: true

require "socket"
require "timeout"

$stdout.sync = true
$stderr.sync = true

server = TCPServer.new("127.0.0.1", 0)
port = server.addr[1]

child_pid = fork do
  server.close

  begin
    require "wreq"
    response = Timeout.timeout(10) { Wreq.get("http://127.0.0.1:#{port}/") }
    abort "child request failed" unless response.bytes == "ok"
  rescue => error
    warn "unexpected #{error.class}: #{error.message}"
    exit! 2
  end

  exit! 0
end

ready = IO.select([server], nil, nil, 10)
abort "child did not connect" unless ready

socket = server.accept
begin
  while (line = socket.gets)
    break if line == "\r\n"
  end
  socket.write("HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
ensure
  socket.close
  server.close
end

_, status = Process.wait2(child_pid)
abort "child failed with #{status.inspect}" unless status.success?

puts "ok"
