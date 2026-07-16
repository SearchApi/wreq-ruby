use std::{io, sync::OnceLock};

use magnus::Ruby;
use tokio::runtime::{Builder, Runtime as TokioRuntime};

use crate::{
    error::{interrupt_error, runtime_initialization_error},
    gvl,
};

#[cfg(unix)]
use crate::{
    arch,
    error::{fork_error, fork_handler_error},
};

/// Initialize the global runtime lazily and preserve failures for Ruby.
static RUNTIME: OnceLock<Result<TokioRuntime, io::Error>> = OnceLock::new();

enum BlockOnError<E> {
    Interrupted,
    Future(E),
}

/// Register fork tracking while the native extension is being loaded.
///
/// # Errors
///
/// Returns `Wreq::ForkError` if the platform cannot install its child-process
/// callback.
#[cfg(unix)]
pub fn initialize(ruby: &Ruby) -> Result<(), magnus::Error> {
    arch::initialize_fork_tracking().map_err(|err| fork_handler_error(ruby, &err))
}

/// Reject a child process that inherited the loaded native extension.
///
/// # Errors
///
/// Returns `Wreq::ForkError` when the extension was loaded before the current
/// process was forked.
pub fn ensure_current(ruby: &Ruby) -> Result<(), magnus::Error> {
    #[cfg(unix)]
    if let Some((owner_pid, current_pid)) = arch::forked_process_ids() {
        return Err(fork_error(ruby, owner_pid, current_pid));
    }

    #[cfg(not(unix))]
    let _ = ruby;

    Ok(())
}

/// Block on a future to completion on the current process's global Tokio runtime.
///
/// The future runs without Ruby's GVL, so it must not construct Ruby objects or
/// Ruby exceptions. Convert Rust errors back into Ruby errors after the GVL has
/// been reacquired.
///
/// # Errors
///
/// Returns `Wreq::ForkError` if the extension belongs to a parent process,
/// `Wreq::BuilderError` if the Tokio runtime cannot be initialized,
/// `Wreq::InterruptError` if Ruby interrupts the request, or the error produced
/// by `map_err` if the future fails.
pub fn try_block_on<F, T, E, M>(ruby: &Ruby, future: F, map_err: M) -> Result<T, magnus::Error>
where
    F: Future<Output = Result<T, E>>,
    M: FnOnce(&Ruby, E) -> magnus::Error,
{
    ensure_current(ruby)?;
    let runtime = RUNTIME
        .get_or_init(|| {
            let mut builder = Builder::new_multi_thread();
            builder.enable_all().build()
        })
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
