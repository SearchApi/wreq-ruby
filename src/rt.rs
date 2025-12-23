use std::sync::LazyLock;

use magnus::{
    DataTypeFunctions, Error, Module, Object, RModule, Ruby, TypedData, function, method,
};
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
pub fn try_block_on<F, T>(future: F) -> F::Output
where
    F: Future<Output = Result<T, magnus::Error>>,
{
    gvl::nogvl_cancellable(|token| {
        RUNTIME.block_on(async move {
            tokio::select! {
                biased;
                _ = token.cancelled() => Err(interrupt_error()),
                result = future => result,
            }
        })
    })
}

/// Block on a future to completion on the global Tokio runtime,
/// returning `None` if cancelled via the provided `CancelFlag`.
#[inline]
pub fn maybe_block_on<F, T>(future: F) -> F::Output
where
    F: Future<Output = Option<T>>,
{
    gvl::nogvl_cancellable(|token| {
        RUNTIME.block_on(async move {
            tokio::select! {
                biased;
                _ = token.cancelled() => None,
                result = future => result,
            }
        })
    })
}

/// A cancellation token for cooperative cancellation of Rust async tasks from Ruby.
///
/// This type wraps [`tokio_util::sync::CancellationToken`] and is exposed to Ruby as
/// `Wreq::CancellationToken`. It allows Ruby code to signal cancellation to long-running or
/// blocking Rust tasks, enabling graceful interruption.
///
/// Typical usage:
/// - Ruby creates a `Wreq::CancellationToken` and passes it to a Rust-backed async operation.
/// - Rust code checks or awaits the token to detect cancellation and aborts work if cancelled.
/// - Calling `cancel` from Ruby triggers cancellation for all tasks observing this token or its
///   clones.
///
/// This is especially useful for interrupting HTTP requests, streaming, or other operations that
/// may need to be stopped from Ruby.
#[derive(Clone, DataTypeFunctions, TypedData)]
#[magnus(class = "Wreq::CancellationToken", free_immediately, size)]
pub struct CancellationToken(tokio_util::sync::CancellationToken);

impl CancellationToken {
    /// Create a new [`CancellationToken`].
    #[inline]
    pub fn new() -> Self {
        Self(tokio_util::sync::CancellationToken::new())
    }

    /// Signal cancellation to all tasks observing this token.
    #[inline]
    pub fn cancel(&self) {
        self.0.cancel()
    }

    #[inline]
    async fn cancelled(&self) {
        self.0.cancelled().await
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let headers_class = gem_module.define_class("CancellationToken", ruby.class_object())?;
    headers_class.define_singleton_method("new", function!(CancellationToken::new, 0))?;
    headers_class.define_method("cancel", method!(CancellationToken::cancel, 0))?;
    Ok(())
}
