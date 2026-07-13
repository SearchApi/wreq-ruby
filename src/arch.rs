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
