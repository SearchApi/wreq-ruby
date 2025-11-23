//! Allow usage of unsafe code for FFI with Ruby's GVL functions.
#![allow(unsafe_code)]

use std::{
    ffi::c_void,
    mem::MaybeUninit,
    ptr::null_mut,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use rb_sys::rb_thread_call_without_gvl;

/// Container for safely passing closure and result through C callback
struct Args<F, R> {
    func: Option<F>,
    result: MaybeUninit<R>,
}

/// Cancellation flag that can be checked from async code
#[derive(Clone)]
pub struct CancelFlag(Arc<AtomicBool>);

impl CancelFlag {
    fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    /// Check if cancellation was requested
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    /// Signal cancellation
    fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

/// Data passed to the unblock function for cancellation
struct UnblockData {
    flag: CancelFlag,
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
/// This sets the cancellation flag.
unsafe extern "C" fn unblock_func(arg: *mut c_void) {
    if !arg.is_null() {
        let data = unsafe { &*(arg as *const UnblockData) };
        data.flag.cancel();
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
/// The closure receives a `CancelFlag` that will be set when Ruby wants to
/// interrupt the thread (e.g., via `Thread.kill` or `Thread.raise`).
///
/// The closure should periodically check `flag.is_cancelled()` or use it
/// with async code to properly handle interruption.
pub fn nogvl_cancellable<F, R>(func: F) -> R
where
    F: FnOnce(CancelFlag) -> R,
    R: Sized,
{
    let unblock_data = UnblockData {
        flag: CancelFlag::new(),
    };
    let flag = unblock_data.flag.clone();

    // Create a wrapper that takes the flag and calls the original function
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
