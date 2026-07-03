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
