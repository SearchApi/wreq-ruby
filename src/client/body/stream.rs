use std::{
    pin::Pin,
    sync::RwLock,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryFutureExt};
use magnus::{Error, RString, TryConvert, Value};
use tokio::sync::{
    Mutex,
    mpsc::{self},
};

use crate::{
    error::{memory_error, mpsc_send_error_to_magnus},
    rt,
};

/// A receiver for streaming HTTP response bodies.
pub struct BodyReceiver(Mutex<Pin<Box<dyn Stream<Item = wreq::Result<Bytes>> + Send>>>);

/// A sender for streaming HTTP request bodies.
#[magnus::wrap(class = "Wreq::BodySender", free_immediately, size)]
pub struct BodySender(RwLock<InnerBodySender>);

struct InnerBodySender {
    tx: Option<mpsc::Sender<Bytes>>,
    rx: Option<mpsc::Receiver<Bytes>>,
}

// ===== impl BodyReceiver =====

impl BodyReceiver {
    /// Create a new [`Receiver`] instance.
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> BodyReceiver {
        BodyReceiver(Mutex::new(Box::pin(stream)))
    }
}

impl Iterator for BodyReceiver {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        rt::maybe_block_on(async {
            self.0
                .lock()
                .await
                .as_mut()
                .next()
                .await
                .and_then(|r| r.ok())
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
        BodySender(RwLock::new(InnerBodySender {
            tx: Some(tx),
            rx: Some(rx),
        }))
    }

    /// Ruby: `push(data)` where data is String or bytes
    pub fn push(rb_self: &Self, data: RString) -> Result<(), Error> {
        let bytes = data.to_bytes();
        let inner = rb_self.0.read().unwrap();
        if let Some(ref tx) = inner.tx {
            rt::try_block_on(tx.send(bytes).map_err(mpsc_send_error_to_magnus))?;
        }
        Ok(())
    }

    /// Ruby: `close` to close the sender
    pub fn close(&self) {
        let mut inner = self.0.write().unwrap();
        inner.tx.take();
        inner.rx.take();
    }
}

impl TryFrom<&BodySender> for ReceiverStream<Bytes> {
    type Error = magnus::Error;

    fn try_from(sender: &BodySender) -> Result<Self, Self::Error> {
        sender
            .0
            .write()
            .unwrap()
            .rx
            .take()
            .map(ReceiverStream::new)
            .ok_or_else(memory_error)
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
