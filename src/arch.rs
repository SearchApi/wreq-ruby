//! Platform-specific support code.
//!
//! Keep this module small and focused on platform quirks that affect linking,
//! ABI boundaries, or OS APIs used by the Rust extension. Normal HTTP client
//! behavior should stay in the client/runtime modules so platform workarounds
//! do not leak into the rest of the binding.
#![allow(unsafe_code)]

/// Whether the native client exposes TCP user-timeout configuration.
pub(crate) const SUPPORTS_TCP_USER_TIMEOUT: bool = cfg!(any(
    target_os = "android",
    target_os = "fuchsia",
    target_os = "linux"
));

/// Whether the native client exposes network-interface binding.
pub(crate) const SUPPORTS_INTERFACE: bool = cfg!(any(
    target_os = "android",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "solaris",
    target_os = "tvos",
    target_os = "visionos",
    target_os = "watchos",
));

#[cfg(unix)]
mod unix {
    use std::{
        ffi::c_int,
        io, process,
        sync::atomic::{AtomicBool, AtomicU32, Ordering},
    };

    static FORKED: AtomicBool = AtomicBool::new(false);
    static OWNER_PID: AtomicU32 = AtomicU32::new(0);

    unsafe extern "C" {
        fn pthread_atfork(
            prepare: Option<unsafe extern "C" fn()>,
            parent: Option<unsafe extern "C" fn()>,
            child: Option<unsafe extern "C" fn()>,
        ) -> c_int;
    }

    /// Mark the copied extension state as unusable in the forked child.
    unsafe extern "C" fn mark_forked() {
        FORKED.store(true, Ordering::Relaxed);
    }

    /// Register process fork tracking before the extension exposes its API.
    pub(crate) fn initialize_fork_tracking() -> io::Result<()> {
        OWNER_PID.store(process::id(), Ordering::Relaxed);

        // POSIX runs the child handler before fork returns. The callback has a
        // static lifetime and only stores an atomic flag.
        // https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_atfork.html
        let status = unsafe { pthread_atfork(None, None, Some(mark_forked)) };
        if status == 0 {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(status))
        }
    }

    /// Return process IDs only when this process inherited the extension.
    pub(crate) fn forked_process_ids() -> Option<(u32, u32)> {
        FORKED
            .load(Ordering::Relaxed)
            .then(|| (OWNER_PID.load(Ordering::Relaxed), process::id()))
    }
}

#[cfg(unix)]
pub(crate) use unix::{forked_process_ids, initialize_fork_tracking};

#[cfg(all(target_os = "windows", target_env = "gnu"))]
mod windows_gnu {
    //! Windows GNU support.
    //!
    //! RubyInstaller's GNU/UCRT Ruby exports some symbols with the same names as
    //! Win32 APIs. When GNU ld links this extension against Ruby, those symbols
    //! can shadow the real Windows APIs unless we pin the calls we care about.

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SleepEx(milliseconds: u32, alertable: i32) -> u32;
    }

    // RubyInstaller's GNU/UCRT Ruby exports a Ruby-aware Sleep symbol. GNU ld can
    // bind extension calls to that symbol instead of KERNEL32!Sleep, which crashes
    // on native worker threads that have no Ruby TLS. The linker wrapper keeps those
    // calls on the Windows API path.
    #[unsafe(no_mangle)]
    pub extern "system" fn __wrap_Sleep(milliseconds: u32) {
        // SAFETY: SleepEx is a Win32 API. Passing alertable=false matches Sleep's
        // behavior, and any u32 duration is accepted by the API.
        unsafe {
            SleepEx(milliseconds, 0);
        }
    }
}
