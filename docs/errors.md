# Error handling

wreq-ruby raises native HTTP failures as subclasses of `Wreq::Error`. The
exception class is the primary category selected by the binding. Predicate
methods preserve the facts reported by wreq before its native error is
consumed.

```ruby
begin
  Wreq.get("https://example.com", timeout: 5)
rescue Wreq::Error => error
  warn error
  warn "connection timed out" if error.timeout? && error.connect?
end
```

The standard exception message includes the native error chain. `warn error`,
`error.message`, `error.to_s`, and uncaught exception output therefore show the
underlying network failure without a wreq-ruby-specific logging method.

Request URIs are not included in diagnostic messages because they may contain
credentials or private query parameters. The original value remains available
through `error.uri`; redact it before logging it.

## How wreq builds an error

A native `wreq::Error` contains one top-level kind and an optional chain of
lower-level causes. The two parts answer different questions:

| Part | Ruby predicates | What it describes |
| --- | --- | --- |
| Top-level kind | `builder?`, `body?`, `tls?`, `decoding?`, `redirect?`, `status?`, `upgrade?`, `request?` | The wreq operation that created the error. These predicates are mutually exclusive. |
| Cause chain | `connection_reset?`, `timeout?`, `proxy_connect?`, `connect?` | Transport facts found while walking Rust's `std::error::Error::source()` chain. Several can be true. |

For example, a refused destination connection has a message shaped like this:

```text
error sending request: client error (Connect): tcp connect error: Connection refused
```

Each part comes from a different layer:

| Message layer | Meaning |
| --- | --- |
| `error sending request` | The top-level kind is Request, so `request?` is true. |
| `client error (Connect)` | The cause chain contains wreq's destination connection stage, so `connect?` is true. |
| `tcp connect error` | The connector identifies the operation that failed. |
| `Connection refused` | The operating system reports the root cause. |

The native cause chain is included as text in the standard Ruby exception
message. It is not converted into a chain of Ruby exception objects, so
`error.cause` continues to mean a Ruby exception cause.

wreq-ruby snapshots wreq's public predicate methods before consuming the native
error. Application code can rely on the Ruby exception classes and predicates,
but it should not match the exact message text. Connector names and root error
wording can change with wreq, its protocol libraries, or the operating system.

## Hierarchy

All regular wreq-ruby errors inherit from `Wreq::Error`, which inherits from
`RuntimeError`:

```text
RuntimeError
`-- Wreq::Error
    +-- Wreq::BuilderError
    +-- Wreq::BodyError
    +-- Wreq::TlsError
    +-- Wreq::DecodingError
    +-- Wreq::RedirectError
    +-- Wreq::StatusError
    +-- Wreq::RequestError
    +-- Wreq::ConnectionResetError
    +-- Wreq::TimeoutError
    +-- Wreq::ProxyConnectError
    +-- Wreq::ConnectError
    +-- Wreq::MemoryError
    `-- Wreq::ForkError
```

`Wreq::InterruptError` inherits from `Interrupt`, not `Wreq::Error`. A broad
`rescue StandardError` must not swallow a Ruby interrupt.

## Error classes

| Error | Meaning |
| --- | --- |
| `Wreq::Error` | Base class and fallback when no public Ruby subclass matches. Native connection upgrade errors currently use this class with `upgrade? == true`. |
| `Wreq::BuilderError` | wreq could not build a URL, request, client, header, or body, or the binding rejected input while building one. Binding-generated instances have no native predicates. |
| `Wreq::ConnectError` | The cause chain contains wreq's destination Connect stage. The root cause may be DNS, TCP, pool acquisition, TLS handshake, or certificate verification. Reset and timeout categories take precedence. |
| `Wreq::ProxyConnectError` | The cause chain identifies a proxy TCP connection, HTTP CONNECT tunnel, or SOCKS negotiation failure. Errors after the tunnel is established can use `Wreq::ConnectError`. |
| `Wreq::ConnectionResetError` | The cause chain retains `io::ErrorKind::ConnectionReset`. An EOF, graceful close, or incomplete HTTP message alone does not satisfy this condition. |
| `Wreq::TimeoutError` | The cause chain contains wreq's timeout marker, a protocol timeout, or `io::ErrorKind::TimedOut`, and no higher-priority category applies. |
| `Wreq::TlsError` | The top-level kind is TLS. This covers TLS connector, trust store, identity, or option setup, not a remote handshake nested under Connect. |
| `Wreq::RequestError` | The top-level kind is Request, but the cause chain has no reset, timeout, proxy connection, or destination connection category. Send failures and incomplete HTTP responses commonly land here. |
| `Wreq::BodyError` | The top-level kind is Body, or the binding could not access request-body sender state. With the current wreq dependency, total response-body timeout is the main native Body case. |
| `Wreq::DecodingError` | The top-level kind is Decode. It covers parsing values, decoding response data, and response-body transport or protocol failures that wreq wraps as Decode. |
| `Wreq::RedirectError` | The top-level kind is Redirect, usually because redirect policy rejected the next hop or the limit was exceeded. |
| `Wreq::StatusError` | `Response#raise_for_status!` created a Status error for a 4xx or 5xx response. It has a status and no lower-level native cause. |
| `Wreq::MemoryError` | A response body operation cannot proceed, or a `BodySender` was already used for a request. This is the compatibility error for the current one-shot APIs. |
| `Wreq::ForkError` | A forked child attempted to use native state inherited from its parent. See [Fork safety](fork-safety.md). |
| `Wreq::InterruptError` | Ruby interrupted a native request wait. This inherits from `Interrupt`, outside the hierarchy above. |

Errors created by the binding rather than by wreq have no active native
predicates. The exception class still identifies the binding operation that
failed.

`Wreq::MemoryError` does not report system memory exhaustion. It preserves the
current error class for one-shot APIs: a response cannot be read after it was
streamed or closed, only one response body operation can run at a time, and a
`BodySender` cannot be attached to a second request. Callers should rescue the
class instead of matching its message.

## Predicates

The top-level kind predicates are mutually exclusive. Cause-chain predicates
are independent and can overlap with that kind and with each other:

```ruby
error.timeout?       # the cause chain contains a timeout
error.connect?       # failure while acquiring a destination connection
error.proxy_connect? # failure while connecting through a proxy
error.request?       # the top-level native kind is Request
```

`timeout?` recursively checks for wreq's timeout marker, a timeout reported by
the HTTP protocol layer, or `io::ErrorKind::TimedOut`. `connection_reset?`
requires an `io::ErrorKind::ConnectionReset` in the chain. `connect?` and
`proxy_connect?` look for wreq's internal client-stage errors. None of these
predicates relies on matching message text.

wreq-ruby records every predicate before consuming the native error, then
selects one Ruby exception class. Native builder, body, TLS, decoding,
redirect, status, and upgrade kinds are considered first. Transport details
then use this order:

1. Connection reset
2. Timeout
3. Proxy connection
4. Destination connection
5. General request failure

This order is part of the Ruby API. A future wreq release can add another
native predicate without silently changing which existing exception class an
application rescues.

## Timeout behavior

`Wreq::TimeoutError` can represent request and connection timeouts. Predicates
show which stage wreq retained:

The `timeout?` predicate comes from `wreq::Error::is_timeout()`. It checks the
native cause chain for wreq's timeout marker, a protocol timeout, or
`std::io::ErrorKind::TimedOut`. wreq-ruby does not classify timeouts by matching
message text.

| Situation | Primary error | Common predicates |
| --- | --- | --- |
| Overall request timeout | `Wreq::TimeoutError` | `timeout?`, `request?` |
| Destination connection timeout | `Wreq::TimeoutError` | `timeout?`, `connect?`, `request?` |
| Proxy connection timeout | `Wreq::TimeoutError` | `timeout?`, `proxy_connect?`, `request?` when wreq preserves the proxy phase |
| Per-read timeout while consuming a response body | `Wreq::TimeoutError` | `timeout?`, `request?` |
| Total timeout while consuming a response body | `Wreq::BodyError` | `body?`, `timeout?` |

The total body timeout remains `Wreq::BodyError` because wreq marks that error
as a body failure, and the native body kind has higher priority. Code that
retries all timeout-related failures should check `timeout?` instead of
rescuing only `Wreq::TimeoutError`.

## Proxy failures

For an HTTPS request through an HTTP proxy, wreq first connects to the proxy,
sends `CONNECT`, and waits for a successful response. Failures in that part of
the exchange use `Wreq::ProxyConnectError`.

After the proxy accepts the tunnel, wreq performs the destination TLS handshake
through it. A proxy that accepts `CONNECT` and then closes the tunnel can
therefore produce `Wreq::ConnectError`, even though a proxy was configured.
Connector-wide timeouts can also lose the narrower proxy phase. The native
cause chain in the standard error message is the best way to distinguish a TCP
timeout, tunnel rejection, DNS failure, TLS alert, or unexpected EOF.

## TLS setup and handshakes

`tls?` checks only wreq's top-level TLS kind. wreq creates that kind while
parsing certificate or identity material, building a trust store, creating the
TLS connector, or applying TLS options.

A remote handshake happens later, inside the destination connection stage. An
expired certificate or TLS alert therefore normally raises `Wreq::ConnectError`
with `connect?` and `request?` set. The root of the message still reports the
certificate verification failure or alert. `tls?` remains false because the
top-level kind is Request, not TLS.

## Closed and reset connections

`connection_reset?` is deliberately narrow. It returns true only when wreq can
still find an operating-system ConnectionReset error in the source chain. A
server can close a socket in ways that the HTTP layer reports as an EOF or an
incomplete message instead. In that case the result is commonly RequestError or
DecodingError, even if a packet capture shows a TCP reset.

This distinction explains why a raw local TCP RST can produce
`Wreq::ConnectionResetError`, while a public reset endpoint can produce
`Wreq::RequestError` with a message such as `connection closed before message
completed`.

## Logging

Use Ruby's standard exception methods:

```ruby
rescue Wreq::Error => error
  warn error
  warn error.full_message(highlight: false)
end
```

`warn error` prints the native cause chain. `full_message` also includes the
exception class, active native predicates, Ruby causes, and backtrace. Neither
output includes `error.uri`.

## Reproducing failures

[`examples/error.rb`](../examples/error.rb) contains runnable cases for each
common transport failure. It uses [badssl.com](https://badssl.com/) for a
rejected certificate and [testserver.host](https://testserver.host/) for HTTP
status, delay, and reset responses. The connect and proxy cases use
`127.0.0.1:1`, which normally has no listening service.

Run every case, or choose individual cases:

```console
bundle exec ruby examples/error.rb
bundle exec ruby examples/error.rb tls timeout reset
```

The `tls` case demonstrates that a rejected remote certificate belongs to the
Connect stage. The public `reset` endpoint sends a TCP RST after receiving the
request. The HTTP layer may turn that into an incomplete-response error before
wreq sees the operating-system reset. In that case, the example raises
`RequestError` and `connection_reset?` returns false.

These public endpoints are useful for manual checks, but they should not be CI
fixtures. They may be unavailable, and a system TLS proxy can change the
certificate result.

## Standard Ruby exceptions

Not every call-site error is a network failure. Invalid or unknown options can
raise `ArgumentError`, values of the wrong Ruby type can raise `TypeError`, and
out-of-range values can raise `RangeError`. A closed `Wreq::BodySender` raises
`IOError`, while calling `Response#chunks` without a block raises
`LocalJumpError`. These exceptions do not inherit from `Wreq::Error`.
