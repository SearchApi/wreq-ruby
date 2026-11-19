# Release branches

| Branch | Status | When to use it |
| --- | --- | --- |
| [`main`](https://github.com/SearchApi/wreq-ruby/tree/main) | Active development | Use this line for new features or if you need HTTP/3. It uses the newer wreq connection pool and runtime. |
| [`1.3.2.x`](https://github.com/SearchApi/wreq-ruby/tree/1.3.2.x) | Active maintenance | Use this line to stay on the 1.3.2 transport while receiving compatible fixes and browser preset updates. It retains the original connection pool and runtime and does not support HTTP/3. |

Both branches support HTTP/1.1 and HTTP/2. On `main`, opt in to HTTP/3 with `Wreq::Client.new(http3_only: true)` or pass `version: Wreq::Version::HTTP_3` on a request. HTTP/3 requires HTTPS and UDP connectivity.

Both branches include the Chrome 158 preset. A change on a branch may not be in a published gem yet. See [GitHub Releases](https://github.com/SearchApi/wreq-ruby/releases) for what shipped in each version.
