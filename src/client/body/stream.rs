use std::{
    cell::RefCell,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryFutureExt};
use magnus::{
    Error, Module, Object, RModule, RString, Ruby, TryConvert, Value, block::Yield, function,
    method,
};
use tokio::sync::mpsc::{self};

use crate::{error::mpsc_send_error_to_magnus, gvl, rt};

#[magnus::wrap(class = "Wreq::Receiver", free_immediately, size)]
pub struct Receiver(Arc<Mutex<mpsc::Receiver<wreq::Result<Bytes>>>>);

impl Receiver {
    /// Create a new [`Receiver`] instance.
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Receiver {
        let (tx, rx) = mpsc::channel(8);
        rt::spawn(async move {
            futures_util::pin_mut!(stream);
            while let Some(item) = stream.next().await {
                if tx.send(item).await.is_err() {
                    break;
                }
            }
        });

        Receiver(Arc::new(Mutex::new(rx)))
    }

    fn each(&self) -> Result<Yield<Receiver>, Error> {
        // Magnus handles yielding to Ruby using an unsafe internal function,
        // so we don’t manage the actual iteration loop ourselves.
        //
        // Since Ruby controls when values are pulled from the iterator,
        // and could potentially call `each` from multiple threads or fibers,
        // we wrap the underlying lister in `Arc<Mutex<_>>` to ensure thread safety.
        //
        // Multi-threaded iteration is rare in Ruby, but this design ensures thread safety.
        Ok(Yield::Iter(Receiver(self.0.clone())))
    }
}

impl Iterator for Receiver {
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

#[magnus::wrap(class = "Wreq::Sender", free_immediately, size)]
pub struct Sender {
    tx: mpsc::Sender<Result<Bytes, io::Error>>,
    rx: RefCell<Option<mpsc::Receiver<Result<Bytes, io::Error>>>>,
}

impl Sender {
    /// Ruby: `Wreq::Sender.new(capacity = 8)`
    pub fn new(args: &[Value]) -> Self {
        let capacity: usize = if let Some(v) = args.first() {
            usize::try_convert(*v).unwrap_or(8)
        } else {
            8
        };

        let (tx, rx) = mpsc::channel(capacity);
        Sender {
            tx,
            rx: RefCell::new(Some(rx)),
        }
    }

    /// Ruby: `push(data)` where data is String or bytes
    pub fn push(rb_self: &Self, data: RString) -> Result<(), Error> {
        let bytes = data.to_bytes();
        let tx = rb_self.tx.clone();
        rt::block_on_nogvl_cancellable(tx.send(Ok(bytes)).map_err(mpsc_send_error_to_magnus))
    }
}

impl From<&Sender> for ReceiverStream<Result<Bytes, io::Error>> {
    fn from(sender: &Sender) -> Self {
        let mut inner = sender.rx.borrow_mut();
        let rx = inner.take().expect("[BUG]: stream already consumed");
        ReceiverStream::new(rx)
    }
}

/// A wrapper around [`tokio::sync::mpsc::Receiver`] that implements [`Stream`].
pub struct ReceiverStream<T> {
    inner: mpsc::Receiver<T>,
}

impl<T> ReceiverStream<T> {
    /// Create a new `ReceiverStream`.
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

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let receiver_class = gem_module.define_class("Receiver", ruby.class_object())?;
    receiver_class.define_method("each", magnus::method!(Receiver::each, 0))?;

    let sender_class = gem_module.define_class("Sender", ruby.class_object())?;
    sender_class.define_singleton_method("new", function!(Sender::new, -1))?;
    sender_class.define_method("push", method!(Sender::push, 1))?;
    Ok(())
}
