use std::sync::{Arc, Mutex};

use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use magnus::{Error, Module, RModule, Ruby, block::Yield};
use tokio::sync::mpsc::{self};

use crate::{RUNTIME, nogvl};

/// A byte stream response.
/// An asynchronous iterator yielding data chunks from the response stream.
/// Used to stream response content.
/// Implemented in the `stream` method of the `Response` class.
/// Can be used in an asynchronous for loop in Python.
#[magnus::wrap(class = "Wreq::Streamer", free_immediately, size)]
pub struct Streamer(Arc<Mutex<mpsc::Receiver<wreq::Result<Bytes>>>>);

impl Streamer {
    /// Create a new [`Streamer`] instance.
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Streamer {
        let (tx, rx) = mpsc::channel(8);
        RUNTIME.spawn(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                if tx.send(item).await.is_err() {
                    break;
                }
            }
        });

        Streamer(Arc::new(Mutex::new(rx)))
    }

    /// @yard
    /// @def each
    /// Returns the next element.
    /// @return [String]
    fn each(&self) -> Result<Yield<Streamer>, Error> {
        // Magnus handles yielding to Ruby using an unsafe internal function,
        // so we don’t manage the actual iteration loop ourselves.
        //
        // Since Ruby controls when values are pulled from the iterator,
        // and could potentially call `each` from multiple threads or fibers,
        // we wrap the underlying lister in `Arc<Mutex<_>>` to ensure thread safety.
        //
        // Multi-threaded iteration is rare in Ruby, but this design ensures thread safety.
        Ok(Yield::Iter(Streamer(self.0.clone())))
    }
}

impl Iterator for Streamer {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        // Assumes low contention. Also we want an entry eventually.
        nogvl::nogvl_cancellable(|cancel_flag| {
            if let Ok(mut inner) = self.0.lock() {
                RUNTIME.block_on(async {
                    tokio::select! {
                        biased;
                        _ = cancel_flag.cancelled() => None,
                        result = inner.recv() => result.and_then(|r| r.ok()),
                    }
                })
            } else {
                None
            }
        })
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let streamer_class = gem_module.define_class("Streamer", ruby.class_object())?;
    streamer_class.define_method("each", magnus::method!(Streamer::each, 0))?;
    Ok(())
}
