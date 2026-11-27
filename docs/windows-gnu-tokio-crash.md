# Windows GNU native extension crashes and linker workarounds

## Summary

On RubyInstaller GNU/UCRT (`x64-mingw-ucrt`), the native extension hits two
platform-specific hazards when linking and running multi-threaded code under
Ruby:

1. **Ruby DLL symbol shadowing**: RubyInstaller's `x64-ucrt-ruby*.dll` exports
   Win32 and C runtime compatibility wrappers (`Sleep`, `fclose`, `getenv`,
   `round`, `strerror`, etc.) that can shadow `KERNEL32` and UCRT symbols when
   GNU `ld` scans libruby before system libraries.
2. **MinGW-w64 `winpthreads` rwlock lazy-init race**: BoringSSL builds with
   `OPENSSL_PTHREADS` on MinGW and initializes its `StaticMutex` / `Mutex`
   locks with `PTHREAD_RWLOCK_INITIALIZER`. MinGW-w64 `winpthreads` builds
   prior to commit `fcc67d0d` ([MinGW-w64 Bug #1044](https://sourceforge.net/p/mingw-w64/bugs/1044/))
   return `EINVAL` to losing threads on concurrent first use of a statically
   initialized rwlock, causing BoringSSL's `BSSL_CHECK` to abort the process
   with `0xC0000409` (reported by Ruby/Rake on Windows as `SignalException: SIGSEGV`).

## Root causes

### 1. RubyInstaller DLL symbol shadowing

RubyInstaller GNU/UCRT exports Ruby-aware wrappers for `Sleep` and several C
runtime functions from `x64-ucrt-ruby*.dll` (including `fclose`, `getenv`,
`round`, and `strerror`).

Because `rb-sys` links `x64-ucrt-ruby*.dll` after dependency crates such as
`btls-sys` (BoringSSL) and `quic-proto`, GNU `ld` can bind unqualified calls
from Rust `core`, BoringSSL, or MinGW CRT helpers to Ruby's DLL instead of
`KERNEL32.dll` or UCRT (`api-ms-win-crt-*.dll` / `ucrtbase.dll`):

- `Sleep` on Tokio native worker threads crashes inside `x64-ucrt-ruby*.dll!Sleep`
  because those threads have no Ruby TLS.
- `fclose` can receive a UCRT `FILE*` opened by BoringSSL (`BIO_new_file`) and
  pass it to Ruby's `rb_w32_fclose`.
- `getenv`, `round`, and `strerror` route through Ruby's wrappers instead of
  UCRT, which is unsafe or unexpected on non-Ruby / no-GVL threads.

### 2. MinGW-w64 `winpthreads` `PTHREAD_RWLOCK_INITIALIZER` race

BoringSSL's `crypto/internal.h` selects `OPENSSL_PTHREADS` under `__MINGW32__`,
so every `bssl::StaticMutex` and `bssl::Mutex` (including the per-`X509` mutex
inside cached root certificates in `wreq`'s shared `BUNDLED_CERT_STORE` and
custom `CertStore` instances) starts as `PTHREAD_RWLOCK_INITIALIZER` (`-1`).

In MinGW-w64 `winpthreads` builds prior to upstream commit
[`fcc67d0d78970c7c670e3094026c048ad732b769`](https://github.com/mingw-w64/mingw-w64/commit/fcc67d0d78970c7c670e3094026c048ad732b769)
(such as MSYS2 `mingw-w64-ucrt-x86_64-winpthreads 14.0.0.r37.g2bfe61fba-1`),
`rwlock_static_init()` returns `EINVAL` instead of `EBUSY` when two threads
race to lazily initialize the same static rwlock. `rwl_ref()` propagates that
`EINVAL` out of `pthread_rwlock_rdlock()` / `pthread_rwlock_wrlock()`, tripping
`BSSL_CHECK(pthread_rwlock_rdlock(&lock_) == 0)` in `bssl::StaticMutex::LockRead()`
and aborting the process via `ucrtbase!abort()` (`STATUS_STACK_BUFFER_OVERRUN`,
`0xC0000409`).

## Fix

For `x86_64-pc-windows-gnu`, `build.rs` wraps both groups of symbols at link
time and `src/arch.rs` provides the wrappers:

```text
-Wl,--wrap=Sleep          -Wl,--wrap=__imp_Sleep
-Wl,--wrap=fclose         -Wl,--wrap=__imp_fclose
-Wl,--wrap=getenv         -Wl,--wrap=__imp_getenv
-Wl,--wrap=round          -Wl,--wrap=__imp_round
-Wl,--wrap=strerror       -Wl,--wrap=__imp_strerror
-Wl,--wrap=pthread_rwlock_rdlock
-Wl,--wrap=pthread_rwlock_wrlock
-Wl,--wrap=pthread_rwlock_tryrdlock
-Wl,--wrap=pthread_rwlock_trywrlock
```

- `__wrap_Sleep` forwards directly to `KERNEL32!SleepEx`.
- `__wrap_fclose`, `__wrap_round`, and `__wrap_strerror` resolve and cache the
  real exports from `ucrtbase.dll` via `GetModuleHandleA` / `GetProcAddress`.
  `__wrap_getenv` snapshots the value under UCRT's environment lock via
  `ucrtbase.dll!_dupenv_s` and keeps a copy in a thread-local buffer so
  concurrent environment changes on other threads do not invalidate a pointer
  still in use by the caller. The matching `__wrap___imp_*` function-pointer
  statics intercept `dllimport` indirect calls (such as BoringSSL's `err.cc`
  calling `*__imp_strerror`), ensuring `x64-ucrt-ruby*.dll` imports only Ruby
  API symbols.
- `__wrap_pthread_rwlock_*` check for `PTHREAD_RWLOCK_INITIALIZER` (`-1`) with
  an atomic acquire load; on first use they serialize initialization under a
  Win32 `SRWLOCK`, call `pthread_rwlock_init` into a stack-local variable, and
  publish the initialized handle with a release store before delegating to
  `__real_pthread_rwlock_*`. This works identically whether `winpthreads` is
  linked statically (`libwinpthread.a`) or dynamically (`libwinpthread-1.dll`),
  and remains compatible with newer fixed `winpthreads` releases.

## Notes

`windows-sys` raw-dylib and adding `-l ucrt` in `build.rs` are not sufficient on
their own because `rb-sys`'s libruby link directive is emitted after dependency
crates such as `btls-sys` and `quic-proto`, and problematic references also come
from Rust `core`, BoringSSL, and MinGW CRT archives (`libmingwex.a`). Link-time
`--wrap` redirects unresolved references to the wrapped symbols at final link
time regardless of library scan order.

Windows builds use the `x86_64-pc-windows-gnu` target and the RubyInstaller
UCRT toolchain. The Rakefile configures that environment automatically, so the
extension and tests can be built with:

```powershell
rustup target add x86_64-pc-windows-gnu
ridk exec pacman -S --needed --noconfirm `
    mingw-w64-ucrt-x86_64-gcc `
    mingw-w64-ucrt-x86_64-clang `
    mingw-w64-ucrt-x86_64-cmake `
    mingw-w64-ucrt-x86_64-pkgconf
bundle exec rake test
```

The first Rake invocation also writes a machine-local `.cargo/windows.toml`.
The checked-in `.cargo/config.toml` loads it automatically, so Cargo and Rust
language servers use the same GNU target and RubyInstaller tools without
editor-specific settings.

## RubyInstaller development tools

RubyInstaller adds Ruby itself to `PATH`, but it does not enable the bundled
MSYS2 development environment globally. Running `ridk enable` adds tools such
as GCC, CMake, and Make to the current terminal only. This is normally useful
because it avoids conflicts with other Windows toolchains.

Rust language servers start Cargo directly and do not run `ridk enable` or the
Rakefile first. Add RubyInstaller's `ucrt64\bin` and `usr\bin` directories to
the user `PATH` so Cargo can load GCC, libclang, and their dependent DLLs from
any editor:

```powershell
$rubyRoot = ruby -rrbconfig -e "print RbConfig::CONFIG.fetch('prefix')"
$required = @(
    (Join-Path $rubyRoot "msys64\ucrt64\bin"),
    (Join-Path $rubyRoot "msys64\usr\bin")
)
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$entries = @($userPath -split ";" | Where-Object {
    $_ -and $_ -notin $required
})
$entries = $required + $entries
[Environment]::SetEnvironmentVariable("Path", ($entries -join ";"), "User")
```

Restart the editor after changing `PATH` so its language server inherits the
updated environment. Run `bundle exec rake compile` once after installing or
switching Ruby so `.cargo/windows.toml` points to the active RubyInstaller.
Run a Rake command after switching the same checkout between Windows and WSL;
Windows generates this local configuration, while other platforms remove it.
