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
    //! Win32 and C runtime APIs. When GNU ld links this extension against Ruby,
    //! `rb-sys`'s libruby flag can appear before UCRT and KERNEL32 on the link
    //! line, causing calls from BoringSSL, Rust `core`, or MinGW CRT helpers to
    //! bind to Ruby's compatibility wrappers instead of the real OS/UCRT APIs.
    //!
    //! BoringSSL also selects `OPENSSL_PTHREADS` on MinGW and initializes its
    //! `StaticMutex`/`Mutex` locks with `PTHREAD_RWLOCK_INITIALIZER`. MinGW-w64
    //! `winpthreads` builds prior to commit `fcc67d0d` (MinGW-w64 Bug #1044)
    //! have a race on concurrent first use of a statically initialized rwlock
    //! where losing threads receive `EINVAL`, causing BoringSSL's `BSSL_CHECK`
    //! to abort the process. Wrapping the rwlock acquisition entry points
    //! serializes lazy initialization safely before delegating to winpthreads.

    use std::{
        cell::{RefCell, UnsafeCell},
        ffi::{CStr, c_char, c_int, c_void},
        mem, ptr, slice,
        sync::atomic::{AtomicIsize, AtomicPtr, Ordering},
    };

    /// Matches MinGW-w64 `winpthreads`'s `typedef intptr_t pthread_rwlock_t;`.
    type PthreadRwlock = isize;

    /// Matches MinGW-w64 `winpthreads`'s `PTHREAD_RWLOCK_INITIALIZER`
    /// (`GENERIC_INITIALIZER`, i.e. `-1`).
    const PTHREAD_RWLOCK_INITIALIZER: PthreadRwlock = -1;

    /// Win32 `LOAD_LIBRARY_SEARCH_SYSTEM32` flag for `LoadLibraryExA`.
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

    /// Win32 `SRWLOCK` (`RTL_SRWLOCK` is a single pointer-sized field; all-zero
    /// matches `SRWLOCK_INIT` and needs no destruction).
    #[repr(transparent)]
    struct SrwLock(UnsafeCell<*mut c_void>);

    // SAFETY: Win32 `SRWLOCK` is designed for cross-thread synchronization via
    // `AcquireSRWLockExclusive` / `ReleaseSRWLockExclusive`.
    unsafe impl Sync for SrwLock {}

    impl SrwLock {
        const fn new() -> Self {
            Self(UnsafeCell::new(ptr::null_mut()))
        }

        fn lock_exclusive(&self) -> SrwLockExclusiveGuard<'_> {
            // SAFETY: `self.0.get()` points to a valid, zero-initialized Win32
            // `SRWLOCK` that lives for the duration of the returned guard.
            unsafe {
                AcquireSRWLockExclusive(self.0.get());
            }
            SrwLockExclusiveGuard(self)
        }
    }

    struct SrwLockExclusiveGuard<'a>(&'a SrwLock);

    impl Drop for SrwLockExclusiveGuard<'_> {
        fn drop(&mut self) {
            // SAFETY: The guard is only constructed after acquiring `self.0`
            // exclusively on the current thread and drops at most once.
            unsafe {
                ReleaseSRWLockExclusive(self.0.0.get());
            }
        }
    }

    static UCRTBASE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_FCLOSE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_DUPENV_S: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_FREE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_GETENV: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_ROUND: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static UCRT_STRERROR: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
    static RWLOCK_INIT_LOCK: SrwLock = SrwLock::new();

    thread_local! {
        // Holds the calling thread's copy of the most recent `getenv` result so
        // concurrent environment updates on other threads cannot invalidate a
        // pointer still in use (for example, BoringSSL loading `SSL_CERT_FILE`
        // or `SSL_CERT_DIR` while the Ruby GVL is released).
        // On `x86_64-pc-windows-gnu`, `thread_local!` expands through
        // `os::Storage` even with `const { ... }`, which triggers a false
        // positive in `missing_const_for_thread_local`.
        #[allow(clippy::missing_const_for_thread_local)]
        static GETENV_BUFFER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    struct UcrtAllocGuard {
        ptr: *mut c_void,
        free_fn: unsafe extern "C" fn(*mut c_void),
    }

    impl Drop for UcrtAllocGuard {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                // SAFETY: `self.ptr` was allocated by `ucrtbase.dll!_dupenv_s`
                // and `self.free_fn` is the matching `ucrtbase.dll!free`.
                unsafe {
                    (self.free_fn)(self.ptr);
                }
            }
        }
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SleepEx(milliseconds: u32, alertable: i32) -> u32;
        fn AcquireSRWLockExclusive(srw_lock: *mut *mut c_void);
        fn ReleaseSRWLockExclusive(srw_lock: *mut *mut c_void);
        fn GetModuleHandleA(module_name: *const c_char) -> *mut c_void;
        fn LoadLibraryExA(
            lib_file_name: *const c_char,
            file: *mut c_void,
            flags: u32,
        ) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *mut c_void;
    }

    unsafe extern "C" {
        fn pthread_rwlock_init(rwlock: *mut PthreadRwlock, attr: *const c_void) -> c_int;
        fn __real_pthread_rwlock_rdlock(rwlock: *mut PthreadRwlock) -> c_int;
        fn __real_pthread_rwlock_wrlock(rwlock: *mut PthreadRwlock) -> c_int;
        fn __real_pthread_rwlock_tryrdlock(rwlock: *mut PthreadRwlock) -> c_int;
        fn __real_pthread_rwlock_trywrlock(rwlock: *mut PthreadRwlock) -> c_int;
    }

    fn ucrtbase_module() -> *mut c_void {
        let cached = UCRTBASE.load(Ordering::Acquire);
        if !cached.is_null() {
            return cached;
        }

        // SAFETY: `ucrtbase.dll` is a NUL-terminated static ASCII string and
        // `GetModuleHandleA` / `LoadLibraryExA` are thread-safe Win32 APIs that
        // require neither the Ruby GVL nor Ruby TLS. On RubyInstaller UCRT,
        // `ucrtbase.dll` is already mapped for the lifetime of the process.
        let mut handle = unsafe { GetModuleHandleA(c"ucrtbase.dll".as_ptr()) };
        if handle.is_null() {
            // SAFETY: Same preconditions as above; `LOAD_LIBRARY_SEARCH_SYSTEM32`
            // restricts resolution to the Windows system directory.
            handle = unsafe {
                LoadLibraryExA(
                    c"ucrtbase.dll".as_ptr(),
                    ptr::null_mut(),
                    LOAD_LIBRARY_SEARCH_SYSTEM32,
                )
            };
        }

        if !handle.is_null() {
            UCRTBASE.store(handle, Ordering::Release);
        }
        handle
    }

    fn resolve_ucrt_symbol(cache: &AtomicPtr<c_void>, name: &CStr) -> Option<*mut c_void> {
        let cached = cache.load(Ordering::Acquire);
        if !cached.is_null() {
            return Some(cached);
        }

        let module = ucrtbase_module();
        if module.is_null() {
            return None;
        }

        // SAFETY: `module` is a valid HMODULE for `ucrtbase.dll` and `name` is
        // a valid NUL-terminated ASCII export name.
        let symbol = unsafe { GetProcAddress(module, name.as_ptr()) };
        if symbol.is_null() {
            return None;
        }

        cache.store(symbol, Ordering::Release);
        Some(symbol)
    }

    fn fallback_round(x: f64) -> f64 {
        // Values at or above 2^52 already have no fractional part in IEEE-754
        // binary64, so `x as i64` below cannot overflow.
        if !x.is_finite() || x.abs() >= (1u64 << 52) as f64 {
            return x;
        }

        let truncated = (x as i64 as f64).copysign(x);
        if (x - truncated).abs() >= 0.5 {
            truncated + 1.0_f64.copysign(x)
        } else {
            truncated
        }
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

    /// Linker wrapper for `fclose` that delegates to UCRT rather than Ruby's
    /// exported `rb_w32_fclose`.
    ///
    /// # Safety
    ///
    /// Caller must pass a valid UCRT `FILE*` stream (or uphold the C `fclose`
    /// contract).
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_fclose(stream: *mut c_void) -> c_int {
        let Some(addr) = resolve_ucrt_symbol(&UCRT_FCLOSE, c"fclose") else {
            return -1;
        };

        // SAFETY: `addr` was resolved from `ucrtbase.dll!fclose`, whose ABI is
        // `extern "C" fn(*mut FILE) -> c_int`, and `stream` meets `fclose`'s
        // contract per the caller.
        unsafe {
            let fclose_fn: unsafe extern "C" fn(*mut c_void) -> c_int = mem::transmute(addr);
            fclose_fn(stream)
        }
    }

    /// Linker wrapper for `getenv` that snapshots the value under UCRT's
    /// internal environment lock via `_dupenv_s` and keeps a copy in the
    /// calling thread's TLS buffer.
    ///
    /// Returning a raw pointer directly from UCRT `getenv` would expose callers
    /// (such as BoringSSL reading `SSL_CERT_FILE` / `SSL_CERT_DIR` without the
    /// Ruby GVL) to UCRT freeing or reallocating that environment entry when
    /// another thread mutates the environment.
    ///
    /// # Safety
    ///
    /// Caller must pass `NULL` or a valid NUL-terminated C string per the C
    /// `getenv` contract.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_getenv(name: *const c_char) -> *mut c_char {
        if name.is_null() {
            return ptr::null_mut();
        }

        if let (Some(dupenv_addr), Some(free_addr)) = (
            resolve_ucrt_symbol(&UCRT_DUPENV_S, c"_dupenv_s"),
            resolve_ucrt_symbol(&UCRT_FREE, c"free"),
        ) {
            // SAFETY: `dupenv_addr` and `free_addr` were resolved from
            // `ucrtbase.dll!_dupenv_s` and `ucrtbase.dll!free` with matching
            // C ABIs. `name` is non-null and NUL-terminated per the caller.
            let dupenv_fn: unsafe extern "C" fn(
                *mut *mut c_char,
                *mut usize,
                *const c_char,
            ) -> c_int = unsafe { mem::transmute(dupenv_addr) };
            let free_fn: unsafe extern "C" fn(*mut c_void) = unsafe { mem::transmute(free_addr) };

            let mut raw_buf: *mut c_char = ptr::null_mut();
            let mut raw_len: usize = 0;
            // SAFETY: Output pointers are valid local storage and `name` is
            // non-null per the check above.
            let rc = unsafe { dupenv_fn(&mut raw_buf, &mut raw_len, name) };
            let _guard = UcrtAllocGuard {
                ptr: raw_buf.cast::<c_void>(),
                free_fn,
            };
            if rc != 0 || raw_buf.is_null() {
                return ptr::null_mut();
            }

            // SAFETY: `_dupenv_s` succeeded with non-null `raw_buf` holding a
            // NUL-terminated string of `raw_len` bytes (including NUL).
            let bytes = if raw_len > 0 {
                let raw = unsafe { slice::from_raw_parts(raw_buf.cast::<u8>(), raw_len) };
                match raw.iter().position(|&b| b == 0) {
                    Some(nul_idx) => &raw[..=nul_idx],
                    None => raw,
                }
            } else {
                unsafe { CStr::from_ptr(raw_buf).to_bytes_with_nul() }
            };

            let copied = GETENV_BUFFER.try_with(|cell| {
                let Ok(mut buf) = cell.try_borrow_mut() else {
                    return ptr::null_mut();
                };
                let extra_nul = usize::from(bytes.last() != Some(&0));
                let Some(required) = bytes.len().checked_add(extra_nul) else {
                    return ptr::null_mut();
                };
                buf.clear();
                if buf.try_reserve(required).is_err() {
                    return ptr::null_mut();
                }
                buf.extend_from_slice(bytes);
                if extra_nul != 0 {
                    buf.push(0);
                }
                buf.as_mut_ptr().cast::<c_char>()
            });
            if let Ok(result) = copied {
                return result;
            }
        }

        // Fallback if thread-local storage is already torn down during thread
        // exit (or if `_dupenv_s` could not be resolved).
        let Some(addr) = resolve_ucrt_symbol(&UCRT_GETENV, c"getenv") else {
            return ptr::null_mut();
        };

        // SAFETY: `addr` was resolved from `ucrtbase.dll!getenv`, whose ABI is
        // `extern "C" fn(*const c_char) -> *mut c_char`, and `name` meets
        // `getenv`'s contract per the caller.
        unsafe {
            let getenv_fn: unsafe extern "C" fn(*const c_char) -> *mut c_char =
                mem::transmute(addr);
            getenv_fn(name)
        }
    }

    /// Linker wrapper for `round` that delegates to UCRT rather than Ruby's
    /// exported `round`.
    #[unsafe(no_mangle)]
    pub extern "C" fn __wrap_round(x: f64) -> f64 {
        let Some(addr) = resolve_ucrt_symbol(&UCRT_ROUND, c"round") else {
            return fallback_round(x);
        };

        // SAFETY: `addr` was resolved from `ucrtbase.dll!round`, whose ABI is
        // `extern "C" fn(f64) -> f64` and accepts any `f64`.
        unsafe {
            let round_fn: extern "C" fn(f64) -> f64 = mem::transmute(addr);
            round_fn(x)
        }
    }

    /// Linker wrapper for `strerror` that delegates to UCRT rather than Ruby's
    /// exported `rb_w32_strerror`.
    #[unsafe(no_mangle)]
    pub extern "C" fn __wrap_strerror(errnum: c_int) -> *mut c_char {
        let Some(addr) = resolve_ucrt_symbol(&UCRT_STRERROR, c"strerror") else {
            return c"Unknown error".as_ptr().cast_mut();
        };

        // SAFETY: `addr` was resolved from `ucrtbase.dll!strerror`, whose ABI
        // is `extern "C" fn(c_int) -> *mut c_char` and accepts any `c_int`.
        unsafe {
            let strerror_fn: extern "C" fn(c_int) -> *mut c_char = mem::transmute(addr);
            strerror_fn(errnum)
        }
    }

    // Also satisfy `dllimport` (`__imp_*`) indirect calls emitted by C/C++ code
    // compiled with `_CRTIMP` or `DECLSPEC_IMPORT` (for example, BoringSSL's
    // `err.cc` calls `*__imp_strerror`), so those references cannot pull
    // shadowed symbols from `x64-ucrt-ruby*.dll` either.
    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    pub static __wrap___imp_Sleep: extern "system" fn(u32) = __wrap_Sleep;

    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    pub static __wrap___imp_fclose: unsafe extern "C" fn(*mut c_void) -> c_int = __wrap_fclose;

    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    pub static __wrap___imp_getenv: unsafe extern "C" fn(*const c_char) -> *mut c_char =
        __wrap_getenv;

    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    pub static __wrap___imp_round: extern "C" fn(f64) -> f64 = __wrap_round;

    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    pub static __wrap___imp_strerror: extern "C" fn(c_int) -> *mut c_char = __wrap_strerror;

    /// Ensures a statically initialized `pthread_rwlock_t` is initialized
    /// before entering winpthreads's lock path.
    ///
    /// Returns `0` when the slot is already initialized, was initialized here,
    /// or is null/misaligned (delegating error reporting to winpthreads).
    /// Returns the non-zero POSIX error code if `pthread_rwlock_init` fails.
    ///
    /// # Safety
    ///
    /// When `rwlock` is non-null and aligned, it must point to a valid,
    /// writable `pthread_rwlock_t` whose lifetime covers this call and which is
    /// not being concurrently destroyed.
    unsafe fn ensure_rwlock_initialized(rwlock: *mut PthreadRwlock) -> c_int {
        if rwlock.is_null() || !rwlock.is_aligned() {
            return 0;
        }

        // SAFETY: `rwlock` is non-null, aligned to `align_of::<AtomicIsize>()`
        // (same layout as `isize` / `intptr_t`), and valid per the caller's
        // contract. The Acquire load pairs with the Release store below so all
        // writes performed by `pthread_rwlock_init` are visible before
        // winpthreads dereferences the handle.
        let slot = unsafe { AtomicIsize::from_ptr(rwlock) };
        if slot.load(Ordering::Acquire) != PTHREAD_RWLOCK_INITIALIZER {
            return 0;
        }

        let _guard = RWLOCK_INIT_LOCK.lock_exclusive();
        if slot.load(Ordering::Relaxed) != PTHREAD_RWLOCK_INITIALIZER {
            return 0;
        }

        // Initialize into a local variable first because winpthreads zeroes
        // `*rwlock` before allocating and populating the rwlock struct; writing
        // directly to `rwlock` would expose a transient `0` to lock-free fast
        // path readers.
        let mut initialized: PthreadRwlock = 0;
        // SAFETY: `&mut initialized` is a valid, aligned local `pthread_rwlock_t`
        // and `NULL` selects default rwlock attributes.
        let rc = unsafe { pthread_rwlock_init(&mut initialized, ptr::null()) };
        if rc == 0 {
            slot.store(initialized, Ordering::Release);
        }
        rc
    }

    /// Linker wrapper for `pthread_rwlock_rdlock`.
    ///
    /// # Safety
    ///
    /// Caller must uphold the POSIX `pthread_rwlock_rdlock` contract for `rwlock`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pthread_rwlock_rdlock(rwlock: *mut PthreadRwlock) -> c_int {
        // SAFETY: Preconditions match `ensure_rwlock_initialized` and
        // `pthread_rwlock_rdlock`.
        unsafe {
            let rc = ensure_rwlock_initialized(rwlock);
            if rc != 0 {
                return rc;
            }
            __real_pthread_rwlock_rdlock(rwlock)
        }
    }

    /// Linker wrapper for `pthread_rwlock_wrlock`.
    ///
    /// # Safety
    ///
    /// Caller must uphold the POSIX `pthread_rwlock_wrlock` contract for `rwlock`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pthread_rwlock_wrlock(rwlock: *mut PthreadRwlock) -> c_int {
        // SAFETY: Preconditions match `ensure_rwlock_initialized` and
        // `pthread_rwlock_wrlock`.
        unsafe {
            let rc = ensure_rwlock_initialized(rwlock);
            if rc != 0 {
                return rc;
            }
            __real_pthread_rwlock_wrlock(rwlock)
        }
    }

    /// Linker wrapper for `pthread_rwlock_tryrdlock`.
    ///
    /// # Safety
    ///
    /// Caller must uphold the POSIX `pthread_rwlock_tryrdlock` contract for `rwlock`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pthread_rwlock_tryrdlock(rwlock: *mut PthreadRwlock) -> c_int {
        // SAFETY: Preconditions match `ensure_rwlock_initialized` and
        // `pthread_rwlock_tryrdlock`.
        unsafe {
            let rc = ensure_rwlock_initialized(rwlock);
            if rc != 0 {
                return rc;
            }
            __real_pthread_rwlock_tryrdlock(rwlock)
        }
    }

    /// Linker wrapper for `pthread_rwlock_trywrlock`.
    ///
    /// # Safety
    ///
    /// Caller must uphold the POSIX `pthread_rwlock_trywrlock` contract for `rwlock`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn __wrap_pthread_rwlock_trywrlock(rwlock: *mut PthreadRwlock) -> c_int {
        // SAFETY: Preconditions match `ensure_rwlock_initialized` and
        // `pthread_rwlock_trywrlock`.
        unsafe {
            let rc = ensure_rwlock_initialized(rwlock);
            if rc != 0 {
                return rc;
            }
            __real_pthread_rwlock_trywrlock(rwlock)
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

    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    mod windows_gnu_tests {
        use std::{
            cell::UnsafeCell,
            ffi::{CStr, c_char, c_int, c_void},
            panic, ptr,
            sync::Barrier,
            thread,
        };

        const PTHREAD_RWLOCK_INITIALIZER: isize = -1;
        const EBUSY: c_int = 16;

        #[repr(transparent)]
        struct TestRwlock(UnsafeCell<isize>);

        // SAFETY: `pthread_rwlock_t` is synchronized across threads via the
        // pthread rwlock APIs in this test.
        unsafe impl Sync for TestRwlock {}

        unsafe extern "system" {
            fn Sleep(milliseconds: u32);
        }

        unsafe extern "C" {
            fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
            fn fclose(stream: *mut c_void) -> c_int;
            fn getenv(name: *const c_char) -> *mut c_char;
            fn _putenv_s(name: *const c_char, value: *const c_char) -> c_int;
            fn round(x: f64) -> f64;
            fn strerror(errnum: c_int) -> *mut c_char;
            fn pthread_rwlock_rdlock(rwlock: *mut isize) -> c_int;
            fn pthread_rwlock_wrlock(rwlock: *mut isize) -> c_int;
            fn pthread_rwlock_tryrdlock(rwlock: *mut isize) -> c_int;
            fn pthread_rwlock_trywrlock(rwlock: *mut isize) -> c_int;
            fn pthread_rwlock_unlock(rwlock: *mut isize) -> c_int;
            fn pthread_rwlock_destroy(rwlock: *mut isize) -> c_int;
        }

        #[test]
        fn pthread_rwlock_static_initializer_concurrent_first_use() {
            const THREADS: usize = 8;
            const ROUNDS: usize = 256;

            for round_idx in 0..ROUNDS {
                let rwlock = TestRwlock(UnsafeCell::new(PTHREAD_RWLOCK_INITIALIZER));
                let barrier = Barrier::new(THREADS);

                let first_panic = thread::scope(|scope| {
                    let mut handles = Vec::with_capacity(THREADS);
                    for thread_idx in 0..THREADS {
                        let rwlock = &rwlock;
                        let barrier = &barrier;
                        handles.push(scope.spawn(move || {
                            barrier.wait();

                            let ptr = rwlock.0.get();
                            let mode = (round_idx + thread_idx) % 4;
                            // SAFETY: `rwlock` outlives the enclosing
                            // `thread::scope`, so `ptr` remains valid until
                            // every worker thread has exited.
                            unsafe {
                                match mode {
                                    0 => {
                                        assert_eq!(pthread_rwlock_rdlock(ptr), 0);
                                        assert_eq!(pthread_rwlock_unlock(ptr), 0);
                                    }
                                    1 => {
                                        assert_eq!(pthread_rwlock_wrlock(ptr), 0);
                                        assert_eq!(pthread_rwlock_unlock(ptr), 0);
                                    }
                                    2 => {
                                        let rc = pthread_rwlock_tryrdlock(ptr);
                                        assert!(
                                            rc == 0 || rc == EBUSY,
                                            "unexpected tryrdlock rc={rc}"
                                        );
                                        if rc == 0 {
                                            assert_eq!(pthread_rwlock_unlock(ptr), 0);
                                        }
                                    }
                                    _ => {
                                        let rc = pthread_rwlock_trywrlock(ptr);
                                        assert!(
                                            rc == 0 || rc == EBUSY,
                                            "unexpected trywrlock rc={rc}"
                                        );
                                        if rc == 0 {
                                            assert_eq!(pthread_rwlock_unlock(ptr), 0);
                                        }
                                    }
                                }
                            }
                        }));
                    }

                    // Join every worker before propagating any failure so no
                    // thread can still touch `rwlock` while unwinding.
                    let mut first_panic = None;
                    for handle in handles {
                        if let Err(err) = handle.join()
                            && first_panic.is_none()
                        {
                            first_panic = Some(err);
                        }
                    }
                    first_panic
                });

                if let Some(err) = first_panic {
                    // SAFETY: All worker threads have joined inside `thread::scope`.
                    unsafe {
                        let _ = pthread_rwlock_destroy(rwlock.0.get());
                    }
                    panic::resume_unwind(err);
                }

                // SAFETY: All worker threads have joined and released the lock.
                unsafe {
                    assert_eq!(pthread_rwlock_destroy(rwlock.0.get()), 0);
                }
            }
        }

        #[test]
        fn wrapped_crt_and_sleep_functions_run_on_native_threads() {
            thread::spawn(|| {
                // SAFETY: Calls wrapped Win32/UCRT symbols from a native OS thread
                // without Ruby TLS to verify they do not bind to libruby wrappers.
                unsafe {
                    Sleep(0);

                    let file = fopen(c"NUL".as_ptr(), c"rb".as_ptr());
                    assert!(!file.is_null(), "UCRT fopen(NUL) should succeed");
                    assert_eq!(fclose(file), 0);

                    assert!(getenv(ptr::null()).is_null());
                    assert!(!getenv(c"SystemRoot".as_ptr()).is_null());
                    assert!(getenv(c"WREQ_RUBY_NONEXISTENT_ENV_VAR_217".as_ptr()).is_null());

                    // Verify `getenv` returns a calling-thread copy that stays
                    // valid even after the underlying UCRT environment entry is
                    // overwritten and deleted.
                    let key = c"WREQ_RUBY_TEST_GETENV_COPY_221";
                    assert_eq!(_putenv_s(key.as_ptr(), c"initial_value".as_ptr()), 0);
                    let snapshot = getenv(key.as_ptr());
                    assert!(!snapshot.is_null());
                    assert_eq!(
                        _putenv_s(
                            key.as_ptr(),
                            c"mutated_value_with_different_length".as_ptr()
                        ),
                        0
                    );
                    assert_eq!(_putenv_s(key.as_ptr(), c"".as_ptr()), 0);
                    assert_eq!(CStr::from_ptr(snapshot), c"initial_value");

                    assert_eq!(round(2.5), 3.0);
                    assert_eq!(round(-2.5), -3.0);
                    assert_eq!(round(2.4), 2.0);
                    assert_eq!(round(-2.4), -2.0);
                    assert!(round(-0.2).is_sign_negative());
                    assert_eq!(round(-0.2), 0.0);

                    let msg_ptr = strerror(2);
                    assert!(!msg_ptr.is_null());
                    let msg = CStr::from_ptr(msg_ptr).to_str().expect("valid UTF-8");
                    assert!(!msg.is_empty());

                    // Also exercise the `__imp_*` function-pointer slots used
                    // by `dllimport` callers (e.g. BoringSSL's `err.cc`).
                    (super::super::windows_gnu::__wrap___imp_Sleep)(0);
                    let imp_file = fopen(c"NUL".as_ptr(), c"rb".as_ptr());
                    assert!(!imp_file.is_null());
                    assert_eq!(
                        (super::super::windows_gnu::__wrap___imp_fclose)(imp_file),
                        0
                    );
                    assert!(
                        !(super::super::windows_gnu::__wrap___imp_getenv)(c"SystemRoot".as_ptr())
                            .is_null()
                    );
                    assert_eq!((super::super::windows_gnu::__wrap___imp_round)(2.5), 3.0);
                    let imp_msg_ptr = (super::super::windows_gnu::__wrap___imp_strerror)(2);
                    assert!(!imp_msg_ptr.is_null());
                    assert_eq!(CStr::from_ptr(imp_msg_ptr), CStr::from_ptr(msg_ptr));
                }
            })
            .join()
            .expect("native thread should not panic");
        }
    }
}
