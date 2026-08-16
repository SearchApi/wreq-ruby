# Fork safety

## Prefork checklist

- `require "wreq"` may run in the parent before workers fork.
- Create each `Wreq::Client`, `Wreq::Jar`, and `Wreq::BodySender` in the worker
  that will use it.
- Keep each `Wreq::Response` in the process that received it.
- Do not start requests or push streaming body data in the parent before workers
  fork.
- If the parent must use wreq-ruby first, start workers with `spawn` or `exec`
  instead of `fork`.

wreq-ruby checks process ownership whenever it exposes guarded native state. It
does not copy, reset, or rebuild inherited objects.

## Loading before fork

wreq-ruby creates its process-wide Tokio runtime on the first operation that
needs it. Requiring the gem does not initialize the runtime, so a prefork server
may load wreq-ruby during boot. Each worker can then create its own runtime on
its first request without an `after_fork!` hook.

Create clients and other native-backed objects inside the worker. Each client,
response, body sender, and cookie jar belongs to the process that created it.
Using an inherited object raises `Wreq::ForkError`, even when the parent never
started the runtime. wreq-ruby does not rebuild these objects.

## Forking after runtime initialization

Once the parent starts an HTTP operation or otherwise uses the Tokio runtime, a
forked child must not reuse it. Tokio's worker threads do not survive `fork`, and
the inherited connection pool may refer to those missing threads.

Operations that need the inherited runtime raise `Wreq::ForkError`. This
includes requests through new or existing clients, module request methods, and
streaming request writes. Constructing a new client, body sender, or cookie jar
does not use the runtime, but runtime-backed operations remain unavailable in
that child. Retrying them raises the same error.

An inherited `Wreq::Response` cannot be used at all. This includes status,
headers, socket addresses, TLS information, and body methods. Values copied out
before the fork, such as a `Wreq::StatusCode` or `Wreq::TlsInfo`, are separate
objects and do not retain access to the response.

The parent remains usable. Native objects collected in the child do not destroy
state owned by the parent process.

Use a spawn- or exec-based worker when the parent must perform HTTP work before
workers start. Requiring the extension again cannot replace an inherited
runtime.
