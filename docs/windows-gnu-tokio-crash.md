# Windows GNU Tokio multi-thread crash

## Summary

On RubyInstaller GNU/UCRT, Tokio's multi-thread runtime could crash when the
native extension was loaded by Ruby. The issue was not Tokio's scheduler logic
itself. It came from Windows symbol resolution while linking the Ruby extension.

## Root cause

RubyInstaller GNU/UCRT exports a Ruby-aware `Sleep` symbol from
`x64-ucrt-ruby400.dll`.

When the extension is linked with GNU ld, an unqualified `Sleep` reference can
bind to Ruby's `Sleep` instead of `KERNEL32!Sleep`. Tokio's multi-thread runtime
creates native worker threads that are not Ruby-managed threads. If one of those
threads calls Ruby's `Sleep`, Ruby expects Ruby thread-local state to exist, but
that TLS slot is empty on the Tokio worker thread. This can crash with an access
violation or Ruby VM bug report.

The local investigation found the fault inside:

```text
x64-ucrt-ruby400.dll!Sleep + 0x40
```

The import table also showed the bad binding:

```text
DLL Name: x64-ucrt-ruby400.dll
  Sleep
```

## Fix

For `x86_64-pc-windows-gnu`, the build now wraps `Sleep` at link time:

```text
-Wl,--wrap=Sleep
```

The wrapper lives in `src/arch.rs` and forwards calls to `KERNEL32!SleepEx`.
This keeps Tokio's worker threads on the real Windows API path while preserving
Tokio's multi-thread runtime and Ruby GVL release behavior.

After the fix, the import table shows:

```text
DLL Name: KERNEL32.dll
  Sleep
  SleepEx
```

## Notes

`windows-sys` raw-dylib was also tested. It is not enough on its own because the
problematic `Sleep` reference can come from Rust std, MinGW, or pthread-related
linking paths, not only from `windows-sys`.

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

Result:

```text
254 runs, 1198 assertions, 0 failures, 0 errors
2 Rust tests passed
```
