//! Platform-specific support code.
//!
//! Keep this module small and focused on platform quirks that affect linking,
//! ABI boundaries, or OS APIs used by the Rust extension. Normal HTTP client
//! behavior should stay in the client/runtime modules so platform workarounds
//! do not leak into the rest of the binding.
#![allow(unsafe_code)]

use std::mem::ManuallyDrop;

use magnus::Ruby;

#[cfg(unix)]
use crate::error::fork_error;

/// Native state that belongs to the process where it was created.
///
/// A forked child must not destroy inherited clients, channels, or response
/// bodies because their synchronization state may belong to threads that no
/// longer exist. The child intentionally leaks the value and lets the operating
/// system reclaim it when the process exits.
///
/// `Send` and `Sync` only describe access between threads in one process. They
/// do not make a runtime, lock, channel, or connection pool safe after `fork`.
///
/// [`ProcessLocal::get`] is the only access path and checks the object's own
/// process generation before exposing its value.
pub(crate) struct ProcessLocal<T> {
    value: ManuallyDrop<T>,
    #[cfg(unix)]
    owner: unix::ProcessToken,
}

impl<T> ProcessLocal<T> {
    /// Wrap native state created by the current process.
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            #[cfg(unix)]
            owner: unix::ProcessToken::current(),
        }
    }

    /// Borrow native state only from the process that created it.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` when the value was inherited from a parent
    /// process.
    #[inline]
    pub(crate) fn get(&self, ruby: &Ruby) -> Result<&T, magnus::Error> {
        #[cfg(unix)]
        if let Some((owner_pid, current_pid)) = self.owner.forked_process_ids() {
            return Err(fork_error(ruby, owner_pid, current_pid));
        }

        #[cfg(not(unix))]
        let _ = ruby;

        Ok(&self.value)
    }
}

impl<T> Drop for ProcessLocal<T> {
    fn drop(&mut self) {
        #[cfg(unix)]
        if self.owner.forked_process_ids().is_some() {
            return;
        }

        // SAFETY: `new` initializes the value exactly once, `ManuallyDrop`
        // prevents an automatic second drop, and this wrapper's `Drop`
        // implementation runs at most once.
        unsafe {
            ManuallyDrop::drop(&mut self.value);
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

    /// Identity of the process generation that created native state.
    ///
    /// Forkguard's child callback only advances an atomic generation counter.
    /// The PID is retained for diagnostics and as a fallback if registering
    /// the callback fails.
    /// https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_atfork.html
    pub(super) struct ProcessToken {
        detector: Option<forkguard::Guard>,
        owner_pid: u32,
    }

    impl ProcessToken {
        /// Capture the current process and fork generation.
        pub(super) fn current() -> Self {
            Self::try_current().unwrap_or_else(|_| Self {
                detector: None,
                owner_pid: process::id(),
            })
        }

        /// Capture the current process after registering fork detection.
        fn try_current() -> io::Result<Self> {
            forkguard::Guard::try_new()
                .map(|detector| Self {
                    detector: Some(detector),
                    owner_pid: process::id(),
                })
                .map_err(|error| io::Error::from_raw_os_error(error.code().get()))
        }

        /// Return process IDs when this token was inherited through a fork.
        pub(super) fn forked_process_ids(&self) -> Option<(u32, u32)> {
            if let Some(detector) = &self.detector {
                // Keep the stored generation unchanged so repeated accesses
                // continue to reject the same inherited object. Cloning the
                // detector copies one usize.
                return detector
                    .clone()
                    .detected_fork()
                    .then(|| (self.owner_pid, process::id()));
            }

            let current_pid = process::id();
            (self.owner_pid != current_pid).then_some((self.owner_pid, current_pid))
        }
    }

    /// Runtime owner captured when Tokio first initializes.
    static RUNTIME_OWNER: OnceLock<ProcessToken> = OnceLock::new();

    /// Register process fork tracking before the Tokio runtime is initialized.
    pub(crate) fn initialize_fork_tracking() -> io::Result<()> {
        if RUNTIME_OWNER.get().is_some() {
            return Ok(());
        }

        let owner = ProcessToken::try_current()?;
        let _ = RUNTIME_OWNER.set(owner);
        Ok(())
    }

    /// Return process IDs when this process inherited an initialized runtime.
    pub(crate) fn forked_process_ids() -> Option<(u32, u32)> {
        RUNTIME_OWNER
            .get()
            .and_then(ProcessToken::forked_process_ids)
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
            assert_eq!(value.value.0.get(), 0);
        }

        assert_eq!(drops.get(), 1);
    }
}
