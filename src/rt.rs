use std::sync::LazyLock;

use magnus::Ruby;
use tokio::runtime::{Builder, Runtime};

use crate::{
    error::{interrupt_error, runtime_initialization_error},
    gvl,
};

/// Initialize the global runtime lazily and preserve failures for Ruby.
static RUNTIME: LazyLock<Result<Runtime, std::io::Error>> = LazyLock::new(|| {
    let mut builder = Builder::new_multi_thread();

    builder.enable_all().build()
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
///
/// # Errors
///
/// Returns `Wreq::BuilderError` if the Tokio runtime cannot be initialized,
/// `Wreq::InterruptError` if Ruby interrupts the request, or the error produced
/// by `map_err` if the future fails.
pub fn try_block_on<F, T, E, M>(ruby: &Ruby, future: F, map_err: M) -> Result<T, magnus::Error>
where
    F: Future<Output = Result<T, E>>,
    M: FnOnce(&Ruby, E) -> magnus::Error,
{
    let runtime = RUNTIME
        .as_ref()
        .map_err(|err| runtime_initialization_error(ruby, err))?;
    let result = gvl::nogvl_cancellable(|flag| {
        runtime.block_on(async move {
            tokio::select! {
                biased;
                _ = flag.cancelled() => Err(BlockOnError::Interrupted),
                result = future => result.map_err(BlockOnError::Future),
            }
        })
    });

    match result {
        Ok(value) => Ok(value),
        Err(BlockOnError::Interrupted) => Err(interrupt_error(ruby)),
        Err(BlockOnError::Future(err)) => Err(map_err(ruby, err)),
    }
}
