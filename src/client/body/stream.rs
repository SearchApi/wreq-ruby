//! Streaming request and response body support.

use std::{
    cell::{Ref, RefCell, RefMut},
    panic::catch_unwind,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use magnus::{Error, Integer, RString, Ruby, Value, scan_args::scan_args};
use tokio::sync::{Mutex, Semaphore, mpsc};

use crate::{
    arch::ProcessLocal,
    error::{
        argument_error, body_sender_borrow_error, body_sender_borrow_mut_error,
        body_sender_send_error, closed_body_sender_error, memory_error, type_error, wreq_error,
    },
    rt,
};

/// Number of chunks buffered when Ruby omits the channel capacity.
const DEFAULT_CHANNEL_CAPACITY: usize = 8;

/// A receiver for streaming HTTP response bodies.
pub struct BodyReceiver(Mutex<Pin<Box<dyn Stream<Item = wreq::Result<Bytes>> + Send>>>);

/// A bounded producer for a single streaming HTTP request body.
///
/// The receiving side may be attached to one request. The producer remains
/// writable until [`BodySender::close`] is called or the request drops its
/// receiver. Ruby's GVL protects state access; no [`RefCell`] borrow is kept
/// while request backpressure waits without the GVL.
#[magnus::wrap(class = "Wreq::BodySender", free_immediately, size)]
pub struct BodySender(ProcessLocal<RefCell<InnerBodySender>>);

/// Mutable ownership state for both halves of the body channel.
struct InnerBodySender {
    /// Producing side, removed by [`BodySender::close`].
    tx: Option<mpsc::Sender<Bytes>>,
    /// Receiving side, removed when the sender is attached to a request.
    rx: Option<mpsc::Receiver<Bytes>>,
}

impl InnerBodySender {
    /// Return whether the channel can no longer accept body chunks.
    fn is_closed(&self) -> bool {
        match &self.tx {
            Some(tx) => tx.is_closed(),
            None => true,
        }
    }
}

// ===== impl BodyReceiver =====

impl BodyReceiver {
    /// Create a new [`BodyReceiver`] instance.
    #[inline]
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> BodyReceiver {
        BodyReceiver(Mutex::new(Box::pin(stream)))
    }

    /// Read the next body chunk, converting stream errors into Ruby errors.
    pub fn next(&self, ruby: &Ruby) -> Result<Option<Bytes>, Error> {
        rt::block_on(ruby, async {
            match self.0.lock().await.as_mut().next().await {
                Some(Ok(data)) => Ok(Some(data)),
                Some(Err(err)) => Err(err),
                None => Ok(None),
            }
        })?
        .map_err(|err| wreq_error(ruby, err))
    }
}

// ===== impl BodySender =====

impl BodySender {
    /// Create a bounded request-body channel.
    ///
    /// Ruby: `Wreq::BodySender.new(capacity = 8)`. Capacity must be greater
    /// than zero and no larger than [`Semaphore::MAX_PERMITS`].
    ///
    /// # Errors
    ///
    /// Returns `TypeError` for a non-Integer capacity and `ArgumentError` for
    /// an invalid range or argument count.
    pub fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let capacity = parse_capacity(ruby, args)?;

        // Create the Tokio channel without allowing an unwind to cross the Ruby FFI boundary.
        //
        // Known panic conditions are rejected by [`parse_capacity`]. The unwind guard
        // remains as a defensive fallback if Tokio adds another channel invariant.
        let (tx, rx) =
            catch_unwind(|| mpsc::channel(capacity)).map_err(|_| invalid_capacity_error(ruby))?;

        Ok(BodySender(ProcessLocal::new(RefCell::new(
            InnerBodySender {
                tx: Some(tx),
                rx: Some(rx),
            },
        ))))
    }

    /// Push a binary chunk, waiting for capacity when the channel is full.
    ///
    /// Ruby: `push(data)` where `data` is a String.
    ///
    /// # Errors
    ///
    /// Returns `IOError` after either channel side has closed. An interrupted
    /// wait raises `Wreq::InterruptError`. Returns `Wreq::ForkError` before
    /// reading an inherited channel.
    pub fn push(ruby: &Ruby, rb_self: &Self, data: RString) -> Result<(), Error> {
        // Clone during the shared borrow, then release it before waiting
        // for capacity. Request attachment needs a mutable borrow.
        let tx = match &rb_self.read_inner(ruby)?.tx {
            Some(tx) if !tx.is_closed() => tx.clone(),
            _ => return Err(closed_body_sender_error(ruby)),
        };

        rt::block_on(ruby, tx.send(data.to_bytes()))?
            .map_err(|err| body_sender_send_error(ruby, err))
    }

    /// Close the producing side while retaining the receiver and queued chunks.
    ///
    /// Calling this method more than once has no additional effect.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` before reading an inherited channel, or
    /// `Wreq::BodyError` if the internal state is already borrowed.
    pub fn close(ruby: &Ruby, rb_self: &Self) -> Result<(), Error> {
        let mut inner = rb_self.write_inner(ruby)?;
        inner.tx.take();
        Ok(())
    }

    /// Return whether this sender can no longer accept body chunks.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::ForkError` before reading an inherited channel, or
    /// `Wreq::BodyError` if the internal state is already borrowed.
    pub fn is_closed(ruby: &Ruby, rb_self: &Self) -> Result<bool, Error> {
        rb_self.read_inner(ruby).map(|r| r.is_closed())
    }

    /// Borrow channel state only in the process that created this sender.
    fn read_inner(&self, ruby: &Ruby) -> Result<Ref<'_, InnerBodySender>, Error> {
        self.0
            .get(ruby)?
            .try_borrow()
            .map_err(|err| body_sender_borrow_error(ruby, err))
    }

    /// Mutably borrow channel state only in the process that created this sender.
    fn write_inner(&self, ruby: &Ruby) -> Result<RefMut<'_, InnerBodySender>, Error> {
        self.0
            .get(ruby)?
            .try_borrow_mut()
            .map_err(|err| body_sender_borrow_mut_error(ruby, err))
    }

    /// Move the receiving side into one request body.
    ///
    /// # Errors
    ///
    /// Returns `Wreq::MemoryError` if the receiver was already consumed, or
    /// `Wreq::BodyError` if Ruby re-enters while the state is borrowed.
    pub(super) fn take_receiver(&self, ruby: &Ruby) -> Result<ReceiverStream<Bytes>, Error> {
        self.write_inner(ruby)?
            .rx
            .take()
            .map(ReceiverStream::new)
            .ok_or_else(|| memory_error(ruby))
    }
}

/// Parse and validate the optional Ruby channel capacity.
///
/// [`mpsc::channel`] panics for zero or values above
/// [`Semaphore::MAX_PERMITS`], so validation must finish before channel creation.
fn parse_capacity(ruby: &Ruby, args: &[Value]) -> Result<usize, Error> {
    let Some(value) = scan_args::<(), (Option<Value>,), (), (), (), ()>(args)?
        .optional
        .0
    else {
        return Ok(DEFAULT_CHANNEL_CAPACITY);
    };

    let integer = Integer::from_value(value)
        .ok_or_else(|| type_error(ruby, "capacity must be an Integer"))?;
    let capacity = integer
        .to_i64()
        .ok()
        .and_then(|capacity| usize::try_from(capacity).ok())
        .filter(|capacity| (1..=Semaphore::MAX_PERMITS).contains(capacity))
        .ok_or_else(|| invalid_capacity_error(ruby))?;

    Ok(capacity)
}

/// Build the synchronous Ruby error used for an invalid channel capacity.
fn invalid_capacity_error(ruby: &Ruby) -> Error {
    argument_error(
        ruby,
        format!("capacity must be between 1 and {}", Semaphore::MAX_PERMITS),
    )
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
