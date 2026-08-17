# Interrupt handling policy

wreq-ruby must not construct or raise Ruby's built-in `Interrupt` to report a
request cancellation. This rule applies to the Rust extension and to Ruby
wrappers in this repository. A pull request that turns a wreq-owned
cancellation into the built-in class must not be merged.

Represent native cancellation as a Rust value until the Ruby-owned calling
thread has reacquired the GVL. Then map a wreq-owned request cancellation to
`Wreq::InterruptError`:

```ruby
Wreq::InterruptError < Interrupt
```

Keep this class outside `StandardError`. A broad transport rescue such as
`rescue StandardError` must not swallow an interruption.

## Why `Interrupt` is reserved

Ruby documents `Interrupt` as the exception raised for an interrupt signal,
usually when the user presses Control-C. Its hierarchy is:

```text
Exception
└── SignalException
    └── Interrupt
```

`Interrupt` is not a `StandardError`. Ruby's default `rescue` catches
`StandardError`, so it does not catch `Interrupt` or `Wreq::InterruptError`.
Code that explicitly uses `rescue Interrupt` catches both because
`Wreq::InterruptError` is a subclass.

The exact built-in class therefore carries Ruby-level control-flow meaning. If
wreq creates that class for its own cancellation, callers cannot tell whether
Ruby delivered an interrupt or the HTTP library cancelled a request. A
library-specific subclass preserves that distinction while keeping the
interruption outside ordinary transport errors.

## Required behavior

| Event | wreq-ruby behavior |
| --- | --- |
| Ruby raises its built-in `Interrupt`, including an exception supplied through `Thread#raise` | Propagate the original exception. Do not replace or wrap it. |
| `Thread#kill`, `Thread#terminate`, or `Thread#exit` stops a thread | Let Ruby perform the fatal thread termination. The native unblock callback may request cancellation, but wreq must not translate the event into `Interrupt`. |
| wreq's native cancellation path finishes without a pending Ruby exception | Raise `Wreq::InterruptError`. |
| A connection, timeout, protocol, or other transport operation fails | Raise the matching wreq transport error under `StandardError`. |

Ruby's implementation also makes an important distinction here.
`Thread#raise` queues the exception chosen by the caller. `Thread#kill` queues
Ruby's internal fatal thread-kill event instead of an `Interrupt` object, and
its termination is asynchronous. Once a no-GVL callback returns, Ruby handles
that fatal event after reacquiring the GVL and before the native call can return
normally to wreq's error mapper.

## Native no-GVL boundary

There are two separate rules at this boundary:

1. A Tokio worker, other Rust background thread, no-GVL callback, or UBF must
   not construct or raise any Ruby exception.
2. Rust code running on the Ruby-owned calling thread with the GVL may construct
   Ruby exceptions, but it must not turn a wreq-owned cancellation into Ruby's
   built-in `Interrupt`.

Requests run through `rb_thread_call_without_gvl`. Ruby's C API documents this
sequence:

1. Handle pending interrupts.
2. Release the GVL.
3. Run the native callback.
4. Reacquire the GVL.
5. Handle interrupts received while the callback was running.

Ruby may call the unblock function, or UBF, when another thread interacts with
the blocked thread. The UBF is a request to stop the native operation. It does
not identify which Ruby exception, if any, is pending.

The UBF in [`src/gvl.rs`](../src/gvl.rs) must only signal cancellation. It must
not call Ruby APIs or raise an exception while the GVL is released. The request
future returns its result as a Rust value. Only after the no-GVL call returns
to the Ruby-owned thread with the GVL may [`src/rt.rs`](../src/rt.rs) map a
wreq-owned cancellation to the `Wreq::InterruptError` defined in
[`src/error.rs`](../src/error.rs).

Keep cancellation conversion centralized in `rt::block_on`. Request, response,
and body operations may call `block_on`, but they must not construct their own
Ruby cancellation exception. `block_on` returns a future's native error
unchanged so the caller can convert it after the GVL has been reacquired.

These forms are forbidden for wreq-owned cancellation:

```rust
MagnusError::new(ruby.exception_interrupt(), "request interrupted")
```

```ruby
raise Interrupt, "request interrupted"
```

Using `exception_interrupt` as the parent when defining
`Wreq::InterruptError` is still required. Using it as the class passed to
`MagnusError::new` is not.

## Review checklist

- Reject direct construction or raising of Ruby's built-in `Interrupt` for a
  wreq-owned cancellation.
- Keep `Wreq::InterruptError` as a direct subclass of `Interrupt`.
- Keep Ruby API calls and exception construction out of the no-GVL callback
  and UBF.
- Preserve an exception supplied by Ruby through `Thread#raise`.
- Do not turn `Thread#kill`, `Thread#terminate`, or `Thread#exit` into a new
  exception.
- Test the real cancellation path, the exception hierarchy, and the
  `StandardError` boundary when changing this code.

## Ruby references

- [Ruby `Interrupt`](https://docs.ruby-lang.org/en/3.4/Interrupt.html) explains
  that the class represents an interrupt signal, usually Control-C, and
  inherits from `SignalException`.
- [Ruby's built-in exception hierarchy](https://docs.ruby-lang.org/en/4.0/Exception.html#class-Exception-label-Built-In+Exception+Class+Hierarchy)
  shows that `SignalException` and `StandardError` are separate branches.
- [`Thread#raise`](https://docs.ruby-lang.org/en/4.0/Thread.html#method-i-raise)
  raises the caller-supplied exception in another thread.
- [`Thread#kill`](https://docs.ruby-lang.org/en/4.0/Thread.html#method-i-kill)
  documents asynchronous termination and its `terminate` and `exit` aliases.
- [`rb_thread_call_without_gvl`](https://docs.ruby-lang.org/capi/en/master/d6/dfb/include_2ruby_2thread_8h.html)
  documents interrupt checks, GVL reacquisition, UBF cancellation, and the
  restriction on Ruby API calls from no-GVL callbacks.
- [Issue #111](https://github.com/SearchApi/wreq-ruby/issues/111) contains the
  original error-hierarchy discussion.
