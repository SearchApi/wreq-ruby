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

## Loading wreq-ruby after fork

A child can use wreq-ruby normally when it loads the extension for the first
time after `fork`. Prefork servers should therefore avoid loading wreq-ruby in
the parent and require it when each worker boots.

For Puma:

```ruby
# Gemfile
gem "wreq", require: false

# config/puma.rb
on_worker_boot do
  require "wreq"
end
```

For Unicorn:

```ruby
# Gemfile
gem "wreq", require: false

# config/unicorn.rb
after_fork do |_server, _worker|
  require "wreq"
end
```

Requiring wreq-ruby again in a child does not reset an extension that was
already loaded by the parent. There is no `after_fork!` reset hook, so the load
order must be fixed before workers are started.
