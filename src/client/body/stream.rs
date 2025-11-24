use std::{
    cell::RefCell,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryFutureExt};
use magnus::{Error, RString, TryConvert, Value};
use tokio::sync::mpsc::{self};

use crate::{error::mpsc_send_error_to_magnus, gvl, rt};

/// A receiver for streaming HTTP response bodies.
pub struct BodyReceiver(Arc<Mutex<mpsc::Receiver<wreq::Result<Bytes>>>>);

/// A sender for streaming HTTP request bodies.
#[magnus::wrap(class = "Wreq::BodySender", free_immediately, size)]
pub struct BodySender(RefCell<InnerBodySender>);

struct InnerBodySender {
    tx: Option<mpsc::Sender<Bytes>>,
    rx: Option<mpsc::Receiver<Bytes>>,
}

// ===== impl BodyReceiver =====

impl BodyReceiver {
    /// Create a new [`Receiver`] instance.
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> BodyReceiver {
        let (tx, rx) = mpsc::channel(8);
        rt::spawn(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                if tx.send(item).await.is_err() {
                    break;
                }
            }
        });

        BodyReceiver(Arc::new(Mutex::new(rx)))
    }
}

impl Iterator for BodyReceiver {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        gvl::nogvl_cancellable(|cancel_flag| {
            if let Ok(mut inner) = self.0.lock() {
                rt::block_on(async {
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

// ===== impl BodySender =====

impl BodySender {
    /// Ruby: `Wreq::Sender.new(capacity = 8)`
    pub fn new(args: &[Value]) -> Self {
        let capacity: usize = if let Some(v) = args.first() {
            usize::try_convert(*v).unwrap_or(8)
        } else {
            8
        };

        let (tx, rx) = mpsc::channel(capacity);
        BodySender(RefCell::new(InnerBodySender {
            tx: Some(tx),
            rx: Some(rx),
        }))
    }

    /// Ruby: `push(data)` where data is String or bytes
    pub fn push(rb_self: &Self, data: RString) -> Result<(), Error> {
        let bytes = data.to_bytes();
        let inner = rb_self.0.borrow();
        if let Some(ref tx) = inner.tx {
            rt::block_on_nogvl_cancellable(tx.send(bytes).map_err(mpsc_send_error_to_magnus))?;
        }
        Ok(())
    }

    /// Ruby: `close` to close the sender
    pub fn close(&self) {
        let mut inner = self.0.borrow_mut();
        inner.tx.take();
        inner.rx.take();
    }
}

impl From<&BodySender> for ReceiverStream<Bytes> {
    fn from(sender: &BodySender) -> Self {
        let rx = sender
            .0
            .borrow_mut()
            .rx
            .take()
            .expect("[BUG]: stream already consumed");
        ReceiverStream::new(rx)
    }
}

/// A wrapper around [`tokio::sync::mpsc::Receiver`] that implements [`Stream`].
pub struct ReceiverStream<T> {
    inner: mpsc::Receiver<T>,
}

impl<T> ReceiverStream<T> {
    /// Create a new [`ReceiverStream`].
    #[inline]
    pub fn new(recv: mpsc::Receiver<T>) -> Self {
        Self { inner: recv }
    }
}

impl<T> Stream for ReceiverStream<T> {
    type Item = T;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_recv(cx)
    }

    /// Returns the bounds of the stream based on the underlying receiver.
    ///
    /// For open channels, it returns `(receiver.len(), None)`.
    ///
    /// For closed channels, it returns `(receiver.len(), Some(used_capacity))`
    /// where `used_capacity` is calculated as `receiver.max_capacity() -
    /// receiver.capacity()`. This accounts for any [`Permit`] that is still
    /// able to send a message.
    ///
    /// [`Permit`]: struct@tokio::sync::mpsc::Permit
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.inner.is_closed() {
            let used_capacity = self.inner.max_capacity() - self.inner.capacity();
            (self.inner.len(), Some(used_capacity))
        } else {
            (self.inner.len(), None)
        }
    }
}
