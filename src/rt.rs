use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};

use crate::{error::interrupt_error, gvl};

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    // RubyInstaller's GNU/UCRT build can crash while Tokio starts its
    // multi-threaded runtime on Windows. Keep that target on a current-thread
    // runtime while still releasing Ruby's GVL around blocking requests.
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    let mut builder = Builder::new_current_thread();

    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    let mut builder = Builder::new_multi_thread();

    builder
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime")
});

enum BlockOnError<E> {
    Interrupted,
    Future(E),
}

/// Block on a future to completion on the global Tokio runtime.
///
/// The future runs without Ruby's GVL, so it must not construct Ruby objects or
/// Ruby exceptions. Convert Rust errors back into Ruby errors after the GVL has
/// been reacquired.
pub fn try_block_on<F, T, E, M>(future: F, map_err: M) -> Result<T, magnus::Error>
where
    F: Future<Output = Result<T, E>>,
    M: FnOnce(E) -> magnus::Error,
{
    let result = gvl::nogvl_cancellable(|flag| {
        RUNTIME.block_on(async move {
            tokio::select! {
                biased;
                _ = flag.cancelled() => Err(BlockOnError::Interrupted),
                result = future => result.map_err(BlockOnError::Future),
            }
        })
    });

    match result {
        Ok(value) => Ok(value),
        Err(BlockOnError::Interrupted) => Err(interrupt_error()),
        Err(BlockOnError::Future(err)) => Err(map_err(err)),
    }
}
