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

## Concurrent prefork workers on Linux

The [`multiprocess_client.rb`](../test/scripts/multiprocess_client.rb) regression
test covers a Linux prefork layout in which the parent loads wreq-ruby before it
starts the workers, but does not create a client or initialize the request
runtime. The master is clean and single-threaded at the fork boundary. The local
HTTP server is forked before `require "wreq"`, so the server process never
inherits the extension or any of its native state.

The test uses two barriers. The first releases four workers to create a fresh
`Wreq::Client` in each process. After all four workers report that their client
exists, the second barrier releases them together to send requests. Each worker
uses the same fresh client for two requests. After every worker exits, the
parent creates its own fresh client and sends one final request.

This works because the parent has not initialized the Tokio runtime when the
workers fork. Under copy-on-write process semantics, each child initializes a
separate runtime and its threads on its first request, in its own address space.
`ProcessLocal` rejects native-backed objects inherited from another process; it
does not stop separate children from creating their own objects and runtime
after the fork.

The test completed 10 consecutive runs on WSL2 x86_64 with Ruby 3.4.8. Those
runs covered 40 worker processes, 80 worker requests, and 10 parent requests.
They produced no failures, deadlocks, or `Wreq::ForkError` exceptions.
The complete fork test file also passed with 4 runs and 100 assertions.

The test server replies with `Connection: close`, so this result does not prove
connection pooling or connection reuse across requests. It verifies that
several prefork workers can create process-local clients and send requests when
the parent has only loaded the extension. If the parent initializes the runtime
before forking, the child remains unsupported even if it creates a new client.
The server waits on a release pipe after the final request so that its `SIGCHLD`
does not interrupt a no-GVL request that is still returning. This pipe is test
harness coordination, not an application requirement.

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
