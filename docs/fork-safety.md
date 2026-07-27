# Fork safety

## Why inherited clients are rejected

wreq-ruby uses a process-wide Tokio runtime and connection pool. `fork` copies
the parent's memory, but only the thread that called `fork` continues in the
child. Tokio's worker threads are gone, and its inherited tasks, locks, and
connections are not safe to reuse.

If the parent has already loaded wreq-ruby, native HTTP operations in the child
raise `Wreq::ForkError`. This applies to new and existing clients, module
request methods, streaming request bodies, and response body methods. Retrying
the operation in the same child raises the same error.

The parent can continue using its clients. When inherited Ruby objects are
collected in the child, their native runtime state is left for the operating
system to reclaim when the process exits.

## Child processes are unsupported

A process created with `fork` must not use wreq-ruby, even when it first loads
the extension after the fork. If the parent loaded wreq-ruby, native operations
in the child raise `Wreq::ForkError`.

When the extension was not present in the parent, no wreq-ruby state or fork
marker reaches the child. The extension cannot reliably distinguish that child
from a newly started process, so this unsupported path cannot guarantee a Ruby
error and may fail inside platform libraries.

Prefork servers should use an `exec`- or spawn-based worker model when workers
need wreq-ruby. Requiring the extension again does not reset inherited runtime
state, and there is no `after_fork!` hook.
