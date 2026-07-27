//! Platform-specific support code.
//!
//! Keep this module small and focused on platform quirks that affect linking,
//! ABI boundaries, or OS APIs used by the Rust extension. Normal HTTP client
//! behavior should stay in the client/runtime modules so platform workarounds
//! do not leak into the rest of the binding.
#![allow(unsafe_code)]

use std::mem::ManuallyDrop;

/// Native state that belongs to the process where the extension was loaded.
///
/// A forked child must not destroy inherited clients, channels, or response
/// bodies because their synchronization state may belong to threads that no
/// longer exist. The child intentionally leaks the value and lets the operating
/// system reclaim it when the process exits.
///
/// This wrapper only controls destruction. Call [`crate::rt::ensure_current`]
/// before accessing the inner value.
#[derive(Clone)]
pub(crate) struct ProcessLocal<T>(ManuallyDrop<T>);

impl<T> ProcessLocal<T> {
    /// Wrap native state created by the current process.
    pub(crate) fn new(value: T) -> Self {
        Self(ManuallyDrop::new(value))
    }
}

impl<T> AsRef<T> for ProcessLocal<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for ProcessLocal<T> {
    fn drop(&mut self) {
        #[cfg(unix)]
        if forked_process_ids().is_some() {
            return;
        }

        // SAFETY: `new` initializes the value exactly once, `ManuallyDrop`
        // prevents an automatic second drop, and this wrapper's `Drop`
        // implementation runs at most once.
        unsafe {
            ManuallyDrop::drop(&mut self.0);
        }
    }
}

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
    use std::{io, process, sync::OnceLock};

    /// Process state captured when the extension initializes.
    ///
    /// The atfork guard uses a POSIX child handler to advance an atomic fork
    /// generation without running Ruby code.
    /// https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_atfork.html
    struct ForkGuard {
        detector: forkguard::Guard,
        owner_pid: u32,
    }

    impl ForkGuard {
        /// Create a guard and register fork detection with the process.
        fn new() -> io::Result<Self> {
            forkguard::Guard::try_new()
                .map(|detector| Self {
                    detector,
                    owner_pid: process::id(),
                })
                .map_err(|error| io::Error::from_raw_os_error(error.code().get()))
        }

        /// Return process IDs when this guard was inherited through a fork.
        fn forked_process_ids(&self) -> Option<(u32, u32)> {
            // Keep the stored generation unchanged so every runtime access in
            // the child remains rejected. Cloning the detector copies one usize.
            let mut detector = self.detector.clone();
            detector
                .detected_fork()
                .then(|| (self.owner_pid, process::id()))
        }
    }

    static FORK_GUARD: OnceLock<ForkGuard> = OnceLock::new();

    /// Register process fork tracking before the extension exposes its API.
    pub(crate) fn initialize_fork_tracking() -> io::Result<()> {
        if FORK_GUARD.get().is_some() {
            return Ok(());
        }

        let guard = ForkGuard::new()?;
        let _ = FORK_GUARD.set(guard);
        Ok(())
    }

    /// Return process IDs only when this process inherited the extension.
    pub(crate) fn forked_process_ids() -> Option<(u32, u32)> {
        FORK_GUARD.get().and_then(ForkGuard::forked_process_ids)
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

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::ProcessLocal;

    struct DropCounter<'a>(&'a Cell<usize>);

    impl Drop for DropCounter<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn process_local_drops_in_its_owner_process() {
        let drops = Cell::new(0);

        {
            let value = ProcessLocal::new(DropCounter(&drops));
            assert_eq!(value.as_ref().0.get(), 0);
        }

        assert_eq!(drops.get(), 1);
    }
}
