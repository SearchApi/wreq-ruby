# frozen_string_literal: true

require "test_helper"
require "openssl"
require "rbconfig"
require "socket"
require_relative "support/ca_tls_server"

class WindowsGnuTest < Minitest::Test
  ALLOWED_LIBRUBY_IMPORT_PREFIX = /\A(?:rb_|ruby_|onig|st_|dln_|Init_)/
  PE_SIGNATURE = "PE\0\0".b
  PE32_PLUS_MAGIC = 0x20b

  def test_extension_does_not_import_shadowed_symbols_from_libruby
    skip "Windows GNU PE import audit only runs on x64-mingw-ucrt" unless RbConfig::CONFIG["arch"] == "x64-mingw-ucrt"

    so_path = find_loaded_extension_path
    refute_nil so_path, "expected wreq_ruby extension path in $LOADED_FEATURES"

    imports_by_dll = parse_pe_imports(File.binread(so_path))
    libruby_entries = imports_by_dll.select { |dll, _| dll.downcase.start_with?("x64-ucrt-ruby") }
    refute_empty libruby_entries, "expected PE import table to reference x64-ucrt-ruby*.dll"

    shadowed = libruby_entries.flat_map do |dll, symbols|
      symbols.reject { |sym| ALLOWED_LIBRUBY_IMPORT_PREFIX.match?(sym) }
        .map { |sym| "#{dll}!#{sym}" }
    end

    assert_empty shadowed, "unexpected non-Ruby symbols imported from libruby: #{shadowed.join(", ")}"
  end

  def test_concurrent_first_tls_handshakes_against_fresh_cert_store
    with_concurrent_tls_server do |url|
      6.times do
        client = Wreq::Client.new(ca_pem: CaTlsServer::CA_PEM, http1_only: true, no_proxy: true, timeout: 5)
        ready_mutex = Mutex.new
        ready_cond = ConditionVariable.new
        ready_count = 0
        thread_count = 6

        threads = Array.new(thread_count) do
          Thread.new do
            ready_mutex.synchronize do
              ready_count += 1
              if ready_count == thread_count
                ready_cond.broadcast
              else
                ready_cond.wait(ready_mutex) while ready_count < thread_count
              end
            end

            response = client.get(url)
            [response.code, response.text]
          end
        end

        results = threads.map(&:value)
        results.each do |code, body|
          assert_equal 200, code
          assert_equal "ok", body
        end
      end
    end
  end

  private

  def find_loaded_extension_path
    $LOADED_FEATURES.find { |path| path.end_with?("/wreq_ruby.so", "\\wreq_ruby.so") && File.file?(path) } ||
      File.expand_path("../lib/wreq_ruby/wreq_ruby.so", __dir__).then { |p| p if File.file?(p) }
  end

  def with_concurrent_tls_server
    ctx = OpenSSL::SSL::SSLContext.new
    ctx.cert = CaTlsServer::SERVER_CERT
    ctx.key = CaTlsServer::SERVER_KEY

    tcp = TCPServer.new("127.0.0.1", 0)
    port = tcp.addr[1]
    workers = []
    workers_mutex = Mutex.new

    accept_thread = Thread.new do
      loop do
        socket = begin
          tcp.accept
        rescue IOError
          break
        end

        worker = Thread.new(socket) do |raw_socket|
          ssl = OpenSSL::SSL::SSLSocket.new(raw_socket, ctx)
          ssl.sync_close = true
          ssl.accept
          line = ssl.gets
          while line && line != "\r\n" && line != "\n"
            line = ssl.gets
          end
          ssl.print "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
        rescue IOError, OpenSSL::SSL::SSLError, SystemCallError
          nil
        ensure
          begin
            ssl&.close
          rescue IOError, SystemCallError
            nil
          end
          raw_socket.close unless raw_socket.closed?
        end

        workers_mutex.synchronize { workers << worker }
      end
    end

    yield "https://127.0.0.1:#{port}"
  ensure
    tcp&.close unless tcp&.closed?
    accept_thread&.join(2)
    workers_mutex&.synchronize { workers.dup }&.each { |worker| worker.join(2) }
  end

  def parse_pe_imports(data)
    pe_offset = data.byteslice(0x3c, 4).unpack1("V")
    raise "invalid PE signature" unless data.byteslice(pe_offset, 4) == PE_SIGNATURE

    num_sections = data.byteslice(pe_offset + 6, 2).unpack1("v")
    opt_header_size = data.byteslice(pe_offset + 20, 2).unpack1("v")
    opt_offset = pe_offset + 24
    magic = data.byteslice(opt_offset, 2).unpack1("v")
    raise "expected PE32+ image" unless magic == PE32_PLUS_MAGIC

    import_dir_rva = data.byteslice(opt_offset + 120, 4).unpack1("V")
    return {} if import_dir_rva.zero?

    sections_offset = opt_offset + opt_header_size
    sections = Array.new(num_sections) do |idx|
      off = sections_offset + (idx * 40)
      _virtual_size, virtual_addr, raw_size, raw_ptr = data.byteslice(off + 8, 16).unpack("V4")
      {vaddr: virtual_addr, raw_size: raw_size, rptr: raw_ptr}
    end

    rva_to_offset = lambda do |rva|
      section = sections.find { |s| s[:raw_size] > 0 && rva >= s[:vaddr] && rva < s[:vaddr] + s[:raw_size] }
      raise "unmapped RVA 0x#{rva.to_s(16)}" unless section
      section[:rptr] + (rva - section[:vaddr])
    end

    read_cstr = lambda do |offset|
      nul = data.index("\0".b, offset)
      raise "unterminated C string at 0x#{offset.to_s(16)}" unless nul
      data.byteslice(offset, nul - offset)
    end

    imports = {}
    desc_offset = rva_to_offset.call(import_dir_rva)
    loop do
      orig_first_thunk, _timestamp, _fwd, name_rva, first_thunk = data.byteslice(desc_offset, 20).unpack("V5")
      break if name_rva.zero?

      dll_name = read_cstr.call(rva_to_offset.call(name_rva))
      thunk_rva = orig_first_thunk.zero? ? first_thunk : orig_first_thunk
      thunk_offset = rva_to_offset.call(thunk_rva)

      symbols = []
      loop do
        entry = data.byteslice(thunk_offset, 8).unpack1("Q<")
        break if entry.zero?

        if (entry & (1 << 63)).zero?
          hint_name_offset = rva_to_offset.call(entry & 0x7fff_ffff)
          symbols << read_cstr.call(hint_name_offset + 2)
        end
        thunk_offset += 8
      end

      imports[dll_name] = symbols
      desc_offset += 20
    end

    imports
  end
end
