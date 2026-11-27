fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sets the correct linker flags for the Ruby C API, which makes it possible
    // to run Cargo commands without requiring `rb_sys/mkmf`.
    //
    // This is not a requirement, but it is a convenient if you want to use
    // `cargo test`, etc.
    let _ = rb_sys_env::activate()?;

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu")
    {
        for symbol in [
            // RubyInstaller's GNU/UCRT Ruby exports Win32 and C runtime
            // compatibility wrappers that can shadow KERNEL32/UCRT symbols
            // when libruby appears before system libraries on the link line.
            // Wrap both direct references and `dllimport` (`__imp_*`) slots
            // (for example, BoringSSL's `err.cc` calls `__imp_strerror`).
            "Sleep",
            "__imp_Sleep",
            "fclose",
            "__imp_fclose",
            "getenv",
            "__imp_getenv",
            "round",
            "__imp_round",
            "strerror",
            "__imp_strerror",
            // MinGW-w64 winpthreads builds prior to commit fcc67d0d (Bug #1044)
            // race on concurrent first use of PTHREAD_RWLOCK_INITIALIZER and
            // return EINVAL, causing BoringSSL's BSSL_CHECK to abort.
            "pthread_rwlock_rdlock",
            "pthread_rwlock_wrlock",
            "pthread_rwlock_tryrdlock",
            "pthread_rwlock_trywrlock",
        ] {
            println!("cargo:rustc-link-arg=-Wl,--wrap={symbol}");
        }
    }

    Ok(())
}
