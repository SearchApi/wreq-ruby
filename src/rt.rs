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

/// Block on a future to completion on the current process's global Tokio runtime.
///
/// The future runs without Ruby's GVL, so it must not construct Ruby objects or
/// Ruby exceptions. Its output is returned unchanged. If that output is a
/// `Result`, convert its error after this function returns and reacquires the
/// GVL.
///
/// # Errors
///
/// Returns `Wreq::ForkError` if the runtime belongs to a parent process,
/// `Wreq::BuilderError` if the Tokio runtime cannot be initialized,
/// or `Wreq::InterruptError` if Ruby interrupts the operation.
pub(crate) fn block_on<F>(ruby: &Ruby, future: F) -> Result<F::Output, magnus::Error>
where
    F: Future,
{
    // Install fork tracking at the same point as the lazy runtime. Loading the
    // extension alone must not claim the runtime for the parent process.
    #[cfg(unix)]
    arch::initialize_fork_tracking().map_err(|err| fork_handler_error(ruby, &err))?;

    // A forked child must not read or use the parent's inherited Tokio state.
    #[cfg(unix)]
    if let Some((owner_pid, current_pid)) = arch::forked_process_ids() {
        return Err(fork_error(ruby, owner_pid, current_pid));
    }

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
                _ = flag.cancelled() => None,
                result = future => Some(result),
            }
        })
    });

    result.ok_or_else(|| interrupt_error(ruby))
}
