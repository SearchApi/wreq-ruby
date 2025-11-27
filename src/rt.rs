use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};

use crate::{error::interrupt_error, gvl};

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime")
});

/// Block on a future to completion on the global Tokio runtime,
/// with support for cancellation via the provided `CancelFlag`.
pub fn block_on_nogvl_cancellable<F, T>(future: F) -> Result<T, magnus::Error>
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

/// Block on a future to completion on the global Tokio runtime.
#[inline]
pub fn block_on<F: Future>(future: F) -> F::Output {
    RUNTIME.block_on(future)
}
