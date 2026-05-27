use std::future::Future;
use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};

use crate::{error::interrupt_error, gvl};

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime")
});

/// Returns a reference to the global Tokio runtime.
pub(crate) fn runtime() -> &'static Runtime {
    &RUNTIME
}

/// Block on a future to completion on the global Tokio runtime,
/// with support for cancellation via the provided `CancelFlag`.
pub fn try_block_on<F, T>(future: F) -> F::Output
where
    F: Future<Output = Result<T, magnus::Error>>,
{
    gvl::nogvl_cancellable(|flag| {
        RUNTIME.block_on(async move {
            tokio::select! {
                biased;
                _ = flag.cancelled() => Err(interrupt_error()),
                result = future => result,
            }
        })
    })
}
