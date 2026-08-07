# frozen_string_literal: true

require "openssl"
require "socket"
require "timeout"

# A small HTTPS server that serves every expected request on one TLS connection.
module TlsTestServer
  RESPONSE_BODY = "ok"

  module_function

  def with_connection(request_count:)
    tcp_server = TCPServer.new("127.0.0.1", 0)
    context, certificate_der = server_context
    ssl_server = OpenSSL::SSL::SSLServer.new(tcp_server, context)
    outcome = Queue.new
    server_thread = Thread.new do
      socket = ssl_server.accept
      request_lines = []

      request_count.times do |index|
        request_lines << read_request(socket)
        connection = (index == request_count - 1) ? "close" : "keep-alive"
        socket.write(response(connection))
        socket.flush
      end

      outcome << {connections: 1, requests: request_lines}
    rescue => error
      outcome << error
    ensure
      socket&.close
    end
    server_thread.report_on_exception = false

    yield "https://127.0.0.1:#{tcp_server.addr[1]}/", certificate_der

    result = Timeout.timeout(5) { outcome.pop }
    raise result if result.is_a?(StandardError)

    result
  ensure
    tcp_server&.close
    server_thread&.join(5)
    if server_thread&.alive?
      server_thread.kill
      server_thread.join
    end
  end

  def read_request(socket)
    request_line = socket.gets
    raise EOFError, "client closed before sending a request" unless request_line

    loop do
      line = socket.gets
      raise EOFError, "client closed while sending headers" unless line
      break if line == "\r\n"
    end

    request_line
  end
  private_class_method :read_request

  def response(connection)
    [
      "HTTP/1.1 200 OK",
      "Content-Length: #{RESPONSE_BODY.bytesize}",
      "Connection: #{connection}",
      "",
      RESPONSE_BODY
    ].join("\r\n")
  end
  private_class_method :response

  def server_context
    key = OpenSSL::PKey::RSA.new(2048)
    certificate = OpenSSL::X509::Certificate.new
    certificate.version = 2
    certificate.serial = 1
    certificate.subject = certificate.issuer = OpenSSL::X509::Name.parse("/CN=127.0.0.1")
    certificate.public_key = key.public_key
    certificate.not_before = Time.now - 60
    certificate.not_after = Time.now + 3600
    certificate.sign(key, OpenSSL::Digest.new("SHA256"))

    context = OpenSSL::SSL::SSLContext.new.tap do |ssl_context|
      ssl_context.cert = certificate
      ssl_context.key = key
    end
    [context, certificate.to_der]
  end
  private_class_method :server_context
end
