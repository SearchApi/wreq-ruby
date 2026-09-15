# frozen_string_literal: true

require "openssl"
require "socket"
require "tempfile"

# A CA-signed HTTPS server for testing custom trust-store options.
#
# Unlike TlsTestServer (self-signed, ephemeral), this module provides
# a proper CA → leaf chain so tests can verify replace/augment trust
# semantics with ca_file / ca_pem / additional_ca_*.
module CaTlsServer
  # --- PKI (generated once at load time) ---

  CA_KEY  = OpenSSL::PKey::RSA.new(2048)
  CA_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 1
    cert.subject    = OpenSSL::X509::Name.parse("/CN=Test CA")
    cert.issuer     = cert.subject
    cert.public_key = CA_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600

    ef = OpenSSL::X509::ExtensionFactory.new
    ef.subject_certificate = cert
    ef.issuer_certificate  = cert
    cert.add_extension(ef.create_extension("basicConstraints", "CA:TRUE", true))
    cert.add_extension(ef.create_extension("subjectKeyIdentifier", "hash"))

    cert.sign(CA_KEY, OpenSSL::Digest::SHA256.new)
  end

  SERVER_KEY  = OpenSSL::PKey::RSA.new(2048)
  SERVER_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 2
    cert.subject    = OpenSSL::X509::Name.parse("/CN=localhost")
    cert.issuer     = CA_CERT.subject
    cert.public_key = SERVER_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600

    ef = OpenSSL::X509::ExtensionFactory.new
    ef.subject_certificate = cert
    ef.issuer_certificate  = CA_CERT
    cert.add_extension(ef.create_extension("subjectAltName", "DNS:localhost,IP:127.0.0.1"))

    cert.sign(CA_KEY, OpenSSL::Digest::SHA256.new)
  end

  CA_PEM = CA_CERT.to_pem

  OTHER_KEY  = OpenSSL::PKey::RSA.new(2048)
  OTHER_CERT = OpenSSL::X509::Certificate.new.tap do |cert|
    cert.version    = 2
    cert.serial     = 3
    cert.subject    = OpenSSL::X509::Name.parse("/CN=Other CA")
    cert.issuer     = cert.subject
    cert.public_key = OTHER_KEY.public_key
    cert.not_before = Time.now - 60
    cert.not_after  = Time.now + 3600
    cert.sign(OTHER_KEY, OpenSSL::Digest::SHA256.new)
  end

  OTHER_PEM  = OTHER_CERT.to_pem
  BUNDLE_PEM = CA_PEM + OTHER_PEM

  # --- Long-lived server (started once, cleaned up at suite end) ---

  @server_started = false

  module_function

  def start_server!
    return if @server_started
    @server_started = true

    ctx = OpenSSL::SSL::SSLContext.new
    ctx.cert = SERVER_CERT
    ctx.key  = SERVER_KEY

    tcp = TCPServer.new("127.0.0.1", 0)
    @port = tcp.addr[1]
    @ssl_server = OpenSSL::SSL::SSLServer.new(tcp, ctx)
    @server_url = "https://localhost:#{@port}"

    @thread = Thread.new do
      loop do
        begin
          client = @ssl_server.accept
        rescue OpenSSL::SSL::SSLError
          next
        rescue IOError
          break
        end
        Thread.new(client) do |c|
          begin
            c.gets
            c.print "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
          rescue
          ensure
            c.close rescue nil
          end
        end
      end
    end
    @thread.abort_on_exception = true

    Minitest.after_run do
      @ssl_server&.close
      @thread&.kill
      @thread&.join(2)
    end
  end

  def server_url
    @server_url
  end

  def with_pem_file(content)
    file = Tempfile.new(["ca", ".pem"])
    file.write(content)
    file.flush
    yield file.path
  ensure
    file&.close!
  end
end