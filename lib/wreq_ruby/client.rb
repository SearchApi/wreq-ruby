# frozen_string_literal: true

module Wreq
  # HTTP client with configurable options.
  #
  # Native (Rust) implementation; all options passed as keyword arguments.
  #
  # @!method self.new(
  #   user_agent: nil,
  #   headers: nil,
  #   referer: nil,
  #   history: nil,
  #   allow_redirects: nil,
  #   max_redirects: nil,
  #   cookie_store: nil,
  #   timeout: nil,
  #   connect_timeout: nil,
  #   read_timeout: nil,
  #   tcp_keepalive: nil,
  #   tcp_keepalive_interval: nil,
  #   tcp_keepalive_retries: nil,
  #   tcp_user_timeout: nil,
  #   tcp_nodelay: nil,
  #   tcp_reuse_address: nil,
  #   pool_idle_timeout: nil,
  #   pool_max_idle_per_host: nil,
  #   pool_max_size: nil,
  #   http1_only: nil,
  #   http2_only: nil,
  #   https_only: nil,
  #   verify: nil,
  #   no_proxy: nil,
  #   proxy: nil,
  #   gzip: nil,
  #   brotli: nil,
  #   deflate: nil,
  #   zstd: nil
  # )
  #   Create a new HTTP client instance.
  #
  #   Time-related numeric options are seconds.
  #
  #   @param user_agent [String, nil] Custom User-Agent header.
  #   @param headers [Hash{String=>String}, nil] Default headers (case-insensitive keys).
  #   @param referer [Boolean, nil] Auto add Referer.
  #   @param history [Boolean, nil] Track redirect history.
  #   @param allow_redirects [Boolean, nil] Follow redirects.
  #   @param max_redirects [Integer, nil] Redirect limit.
  #   @param cookie_store [Boolean, nil] Enable in‑memory cookie jar.
  #   @param timeout [Integer, nil] Overall request timeout.
  #   @param connect_timeout [Integer, nil] Connect timeout.
  #   @param read_timeout [Integer, nil] Read timeout.
  #   @param tcp_keepalive [Integer, nil] Keepalive idle before probes.
  #   @param tcp_keepalive_interval [Integer, nil] Interval between probes.
  #   @param tcp_keepalive_retries [Integer, nil] Failed probes before drop.
  #   @param tcp_user_timeout [Integer, nil] Unacked data timeout (Linux).
  #   @param tcp_nodelay [Boolean, nil] Enable TCP_NODELAY.
  #   @param tcp_reuse_address [Boolean, nil] Enable SO_REUSEADDR.
  #   @param pool_idle_timeout [Integer, nil] Idle connection eviction.
  #   @param pool_max_idle_per_host [Integer, nil] Max idle per host.
  #   @param pool_max_size [Integer, nil] Pool size cap.
  #   @param http1_only [Boolean, nil] Force HTTP/1.
  #   @param http2_only [Boolean, nil] Force HTTP/2.
  #   @param https_only [Boolean, nil] Only allow HTTPS.
  #   @param verify [Boolean, nil] TLS certificate verification.
  #   @param no_proxy [Boolean, nil] Disable proxy usage.
  #   @param proxy [String, nil] Proxy URI.
  #   @param gzip [Boolean, nil] Accept gzip.
  #   @param brotli [Boolean, nil] Accept brotli.
  #   @param deflate [Boolean, nil] Accept deflate.
  #   @param zstd [Boolean, nil] Accept zstd.
  #   @return [Wreq::Client]
  #
  #   @example Minimal
  #     client = Wreq::Client.new
  #
  #   @example Headers
  #     client = Wreq::Client.new(headers: { "Accept" => "application/json" })
  #
  #   @example Timeouts
  #     client = Wreq::Client.new(timeout: 30, connect_timeout: 5, read_timeout: 10)
  #
  #   @example Redirects
  #     client = Wreq::Client.new(allow_redirects: true, max_redirects: 3)
  #
  #   @example Compression
  #     client = Wreq::Client.new(gzip: true, brotli: true)
  #
  #   @example Proxy
  #     client = Wreq::Client.new(proxy: "http://127.0.0.1:8080")
  #
  #   @example HTTPS only
  #     client = Wreq::Client.new(https_only: true, verify: true)
  class Client
  end
end
