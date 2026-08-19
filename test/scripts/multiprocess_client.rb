# frozen_string_literal: true

require "rbconfig"
require "socket"
require "timeout"

$stdout.sync = true
$stderr.sync = true

WORKER_COUNT = 4
REQUESTS_PER_WORKER = 2
PARENT_REQUEST_PATH = "/parent/request/0"

abort "multiprocess client test requires Linux" unless RbConfig::CONFIG.fetch("host_os").include?("linux")

server = TCPServer.new("127.0.0.1", 0)
url = "http://127.0.0.1:#{server.addr[1]}"
server_release_reader, server_release_writer = IO.pipe
server_pid = fork do
  server_release_writer.close
  Timeout.timeout(55) do
    observed_paths = Array.new((WORKER_COUNT * REQUESTS_PER_WORKER) + 1) do
      socket = server.accept
      begin
        request_line = socket.gets
        abort "server received an incomplete request" unless request_line

        while (line = socket.gets)
          break if line == "\r\n"
        end

        method, path, = request_line.split(" ", 3)
        abort "server received an unexpected method: #{method.inspect}" unless method == "GET"

        socket.write(
          "HTTP/1.1 200 OK\r\n" \
          "Content-Length: #{path.bytesize}\r\n" \
          "Connection: close\r\n\r\n" \
          "#{path}"
        )
        path
      ensure
        socket.close
      end
    end

    expected_paths = WORKER_COUNT.times.flat_map do |worker|
      REQUESTS_PER_WORKER.times.map { |request| "/worker/#{worker}/request/#{request}" }
    end
    expected_paths << PARENT_REQUEST_PATH
    abort "server received unexpected paths: #{observed_paths.inspect}" unless observed_paths.sort == expected_paths.sort
    abort "server did not receive its release signal" unless server_release_reader.read(1) == "."
  end
  exit! 0
rescue => error
  warn "server=unexpected #{error.class}: #{error.message}"
  exit! 3
ensure
  server_release_reader.close
  server.close
end
server.close
server_release_reader.close

# The parent only loads the extension. It does not create a Client or initialize
# the request runtime before forking the workers.
require "wreq"

client_start_reader, client_start_writer = IO.pipe
client_ready_reader, client_ready_writer = IO.pipe
request_start_reader, request_start_writer = IO.pipe
worker_pids = WORKER_COUNT.times.map do |worker|
  fork do
    server_release_writer.close
    client_start_writer.close
    client_ready_reader.close
    request_start_writer.close

    abort "worker #{worker} did not receive its client signal" unless client_start_reader.read(1) == "."
    client_start_reader.close

    client = Wreq::Client.new(no_proxy: true, http1_only: true, timeout: 5)
    client_ready_writer.write(".")
    client_ready_writer.close

    abort "worker #{worker} did not receive its request signal" unless request_start_reader.read(1) == "."
    request_start_reader.close

    Timeout.timeout(30) do
      REQUESTS_PER_WORKER.times do |request|
        path = "/worker/#{worker}/request/#{request}"
        response = client.get("#{url}#{path}")
        abort "worker #{worker} request #{request} failed" unless response.bytes == path
      end
    end

    warn "worker_#{worker}=ok"
    exit! 0
  rescue => error
    warn "worker_#{worker}=unexpected #{error.class}: #{error.message}"
    exit! 2
  ensure
    client_start_reader.close unless client_start_reader.closed?
    client_ready_writer.close unless client_ready_writer.closed?
    request_start_reader.close unless request_start_reader.closed?
  end
end

client_start_reader.close
client_ready_writer.close
request_start_reader.close

client_start_writer.write("." * WORKER_COUNT)
client_start_writer.close

ready_workers = client_ready_reader.read(WORKER_COUNT) || ""
client_ready_reader.close
abort "only #{ready_workers.bytesize} workers created a Client" unless ready_workers.bytesize == WORKER_COUNT

request_start_writer.write("." * WORKER_COUNT)
request_start_writer.close

failed_workers = worker_pids.filter_map do |worker_pid|
  pid, status = Process.wait2(worker_pid)
  [pid, status] unless status.success?
end

unless failed_workers.empty?
  server_release_writer.close
  begin
    Process.kill("TERM", server_pid)
  rescue Errno::ESRCH
    nil
  end
  Process.wait(server_pid)
  abort "workers failed: #{failed_workers.map { |pid, status| "#{pid}=#{status.inspect}" }.join(", ")}"
end

begin
  parent_client = Wreq::Client.new(no_proxy: true, http1_only: true, timeout: 5)
  parent_response = parent_client.get("#{url}#{PARENT_REQUEST_PATH}")
  raise "parent request after workers failed" unless parent_response.bytes == PARENT_REQUEST_PATH
  warn "parent_after_workers=ok"
rescue => error
  server_release_writer.close
  begin
    Process.kill("TERM", server_pid)
  rescue Errno::ESRCH
    nil
  end
  Process.wait(server_pid)
  abort "parent_after_workers=unexpected #{error.class}: #{error.message}"
end

server_release_writer.write(".")
server_release_writer.close

_, server_status = Process.wait2(server_pid)
abort "server failed with #{server_status.inspect}" unless server_status.success?

puts "ok"
