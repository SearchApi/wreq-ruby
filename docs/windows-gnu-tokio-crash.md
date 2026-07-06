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

Verified with:

```powershell
.\script\build_windows_gnu.ps1 -SkipBundleInstall -SkipToolInstall -RunTests
```

Result:

```text
160 runs, 775 assertions, 0 failures, 0 errors
```
