fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sets the correct linker flags for the Ruby C API, which makes it possible
    // to run Cargo commands without requiring `rb_sys/mkmf`.
    //
    // This is not a requirement, but it is a convenient if you want to use
    // `cargo test`, etc.
    let _ = rb_sys_env::activate()?;
    Ok(())
}
