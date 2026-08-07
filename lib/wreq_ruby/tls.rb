# frozen_string_literal: true

unless defined?(Wreq)
  module Wreq
    # Peer certificate data captured for one HTTPS response.
    #
    # Instances are returned by {Wreq::Response#tls_info}. Certificate bytes
    # remain available after the response body is read or closed, even if the
    # connection is later reused.
    #
    # The returned certificate Strings are Ruby-owned copies. Changing one does
    # not alter the stored TLS data or values returned by later calls. The chain
    # Array is frozen, but its String elements remain mutable.
    #
    # Certificates use the DER encoding described by the X.509 profile in
    # RFC 5280.
    #
    # @example Parse the peer certificate with OpenSSL
    #   require "openssl"
    #
    #   client = Wreq::Client.new(tls_info: true)
    #   response = client.get("https://example.com")
    #   der = response.tls_info&.peer_certificate
    #
    #   if der
    #     certificate = OpenSSL::X509::Certificate.new(der)
    #     puts certificate.subject
    #   end
    # @see https://www.rfc-editor.org/rfc/rfc5280#section-4.1 X.509 certificate format
    class TlsInfo
      # Return the peer's leaf certificate.
      #
      # @return [String, nil] a new DER-encoded String with
      #   +Encoding::BINARY+, or +nil+ when the transport did not provide one
      def peer_certificate
      end

      # Return the peer certificate chain.
      #
      # The Array is frozen. Each element is a new DER-encoded binary String.
      # The chain includes the leaf certificate when the transport supplies it.
      #
      # @return [Array<String>, nil] a frozen Array of certificate copies, or
      #   +nil+ when the transport did not provide a chain
      def peer_certificate_chain
      end
    end
  end
end

# ======================== Ruby API Extensions ========================

module Wreq
  class TlsInfo
    # Return a compact summary for debugging.
    #
    # The summary reports the leaf certificate size and the number of
    # certificates in the chain without printing the DER data.
    #
    # @return [String] TLS certificate metadata
    # @example
    #   tls_info.inspect
    #   # => "#<Wreq::TlsInfo peer_certificate=781B peer_certificate_chain=1>"
    def inspect
      certificate = peer_certificate
      chain = peer_certificate_chain
      certificate_size = certificate ? "#{certificate.bytesize}B" : "nil"
      chain_size = chain ? chain.length : "nil"

      "#<#{self.class} peer_certificate=#{certificate_size} peer_certificate_chain=#{chain_size}>"
    end
  end
end
