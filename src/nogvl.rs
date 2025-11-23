//! Allow usage of unsafe code for FFI with Ruby's GVL functions.
#![allow(unsafe_code)]

use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

use rb_sys::rb_thread_call_without_gvl;
use tokio::sync::watch;

/// Container for safely passing closure and result through C callback
struct Args<F, R> {
    func: Option<F>,
    result: MaybeUninit<R>,
}

/// Cancellation flag using tokio's watch channel for efficient async notification.
///
/// This provides zero-latency cancellation without polling - the async code
/// is notified immediately when `cancel()` is called.
#[derive(Clone)]
pub struct CancelFlag {
    rx: watch::Receiver<bool>,
}

/// Internal sender half of the cancellation flag.
struct CancelSender {
    tx: watch::Sender<bool>,
}

impl CancelSender {
    fn new() -> (Self, CancelFlag) {
        let (tx, rx) = watch::channel(false);
        (Self { tx }, CancelFlag { rx })
    }

    /// Signal cancellation to all receivers.
    fn cancel(&self) {
        let _ = self.tx.send(true);
    }
}

impl CancelFlag {
    /// Wait until cancellation is signaled.
    ///
    /// This is an async function that completes immediately if already cancelled,
    /// or waits efficiently (no polling) until `cancel()` is called.
    pub async fn cancelled(&self) {
        let mut rx = self.rx.clone();
        // If already cancelled, return immediately
        if *rx.borrow_and_update() {
            return;
        }
        // Wait for cancellation signal
        loop {
            if rx.changed().await.is_err() {
                // Sender dropped - treat as cancelled
                return;
            }
            if *rx.borrow() {
                return;
            }
        }
    }
}

/// Data passed to the unblock function for cancellation
struct UnblockData {
    sender: CancelSender,
}

unsafe extern "C" fn call_without_gvl<F, R>(arg: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> R,
    R: Sized,
{
    let args = unsafe { &mut *(arg as *mut Args<F, R>) };

    // Take closure from Option to transfer ownership
    if let Some(func) = args.func.take() {
        let result = func();
        args.result.write(result);
    }

    null_mut()
}

/// Unblock function called by Ruby when thread is interrupted.
/// This signals cancellation via the watch channel.
unsafe extern "C" fn unblock_func(arg: *mut c_void) {
    if !arg.is_null() {
        let data = unsafe { &*(arg as *const UnblockData) };
        data.sender.cancel();
    }
}

/// Execute a closure without holding the GVL (original non-cancellable version).
/// Use `nogvl_cancellable` for operations that should support thread interruption.
pub fn nogvl<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
    R: Sized,
{
    // Create stable wrapper to keep data valid during callback
    let mut args = Args {
        func: Some(func),
        result: MaybeUninit::uninit(),
    };

    let arg_ptr = &mut args as *mut _ as *mut c_void;

    unsafe {
        rb_thread_call_without_gvl(Some(call_without_gvl::<F, R>), arg_ptr, None, null_mut());
        args.result.assume_init()
    }
}

/// Execute a closure without holding the GVL, with support for thread interruption.
///
/// The closure receives a `CancelFlag` that will be signaled when Ruby wants to
/// interrupt the thread (e.g., via `Thread.kill` or `Thread.raise`).
///
/// Use `flag.cancelled().await` in async code to efficiently wait for cancellation.
///
/// # Example
///
/// ```rust
/// nogvl_cancellable(|cancel_flag| {
///     RUNTIME.block_on(async move {
///         tokio::select! {
///             biased;
///             _ = cancel_flag.cancelled() => Err(interrupted_error()),
///             result = some_async_operation() => result,
///         }
///     })
/// })
/// ```
pub fn nogvl_cancellable<F, R>(func: F) -> R
where
    F: FnOnce(CancelFlag) -> R,
    R: Sized,
{
    let (sender, flag) = CancelSender::new();
    let unblock_data = UnblockData { sender };

    // Create a wrapper that holds the function, flag, and result
    struct Wrapper<F, R> {
        func: Option<F>,
        flag: CancelFlag,
        result: MaybeUninit<R>,
    }

    let mut wrapper = Wrapper {
        func: Some(func),
        flag,
        result: MaybeUninit::uninit(),
    };

    unsafe extern "C" fn call_with_flag<F, R>(arg: *mut c_void) -> *mut c_void
    where
        F: FnOnce(CancelFlag) -> R,
    {
        let wrapper = unsafe { &mut *(arg as *mut Wrapper<F, R>) };
        if let Some(func) = wrapper.func.take() {
            let result = func(wrapper.flag.clone());
            wrapper.result.write(result);
        }
        null_mut()
    }

    let wrapper_ptr = &mut wrapper as *mut _ as *mut c_void;
    let unblock_data_ptr = &unblock_data as *const _ as *mut c_void;

    unsafe {
        rb_thread_call_without_gvl(
            Some(call_with_flag::<F, R>),
            wrapper_ptr,
            Some(unblock_func),
            unblock_data_ptr,
        );
        wrapper.result.assume_init()
    }
}
