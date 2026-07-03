//! Allow usage of unsafe code for FFI with Ruby's GVL functions.
#![allow(unsafe_code)]

use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

use rb_sys::rb_thread_call_without_gvl;
use tokio::sync::watch;

/// Container for safely passing closure and result through C callback.
struct Args<F, R> {
    func: Option<F>,
    result: MaybeUninit<R>,
}

/// Cancellation flag for thread interruption support.
#[derive(Clone)]
pub struct CancelFlag {
    rx: watch::Receiver<bool>,
}

struct CancelSender {
    tx: watch::Sender<bool>,
}

impl CancelSender {
    fn new() -> (Self, CancelFlag) {
        let (tx, rx) = watch::channel(false);
        (Self { tx }, CancelFlag { rx })
    }

    fn cancel(&self) {
        let _ = self.tx.send(true);
    }
}

impl CancelFlag {
    /// Wait until cancellation is signaled (zero-latency, no polling).
    pub async fn cancelled(&self) {
        let mut rx = self.rx.clone();
        if *rx.borrow_and_update() {
            return;
        }
        loop {
            if rx.changed().await.is_err() {
                return;
            }
            if *rx.borrow() {
                return;
            }
        }
    }
}

struct UnblockData {
    sender: CancelSender,
}

unsafe extern "C" fn call_without_gvl<F, R>(arg: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> R,
    R: Sized,
{
    let args = unsafe { &mut *(arg as *mut Args<F, R>) };

    // Take closure from Option to transfer ownership.
    if let Some(func) = args.func.take() {
        args.result.write(func());
    }

    null_mut()
}

unsafe extern "C" fn unblock_func(arg: *mut c_void) {
    if !arg.is_null() {
        let data = unsafe { &*(arg as *const UnblockData) };
        data.sender.cancel();
    }
}

/// Executes the given closure without holding the Ruby GVL (Global VM Lock).
///
/// WARNING: Do NOT nest calls to [`nogvl`] or [`nogvl_cancellable`] inside each other.
/// Nesting these functions will cause Ruby thread deadlock, because the inner call
/// will block waiting for the GVL while the outer call has already released it.
/// This results in all Ruby threads being suspended indefinitely.
pub fn nogvl<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
    R: Sized,
{
    // Create stable wrapper to keep data valid during callback.
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

/// Executes the given closure without GVL, supporting cancellation via thread interrupt.
///
/// WARNING: Do NOT nest calls to [`nogvl`] or [`nogvl_cancellable`] inside each other.
/// Nesting these functions will cause Ruby thread deadlock, because the inner call
/// will block waiting for the GVL while the outer call has already released it.
/// This results in all Ruby threads being suspended indefinitely.
pub fn nogvl_cancellable<F, R>(func: F) -> R
where
    F: FnOnce(CancelFlag) -> R,
    R: Sized,
{
    let (sender, flag) = CancelSender::new();
    let unblock_data = UnblockData { sender };

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
            wrapper.result.write(func(wrapper.flag.clone()));
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
