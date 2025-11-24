use std::{
    io,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use magnus::{
    Error, Module, Object, RModule, RString, Ruby, TryConvert, Value, block::Yield, function,
    method,
};
use tokio::sync::mpsc::{self};

use crate::{RUNTIME, gvl};

#[magnus::wrap(class = "Wreq::Receiver", free_immediately, size)]
pub struct Receiver(Arc<Mutex<mpsc::Receiver<wreq::Result<Bytes>>>>);

impl Receiver {
    /// Create a new [`Receiver`] instance.
    pub fn new(stream: impl Stream<Item = wreq::Result<Bytes>> + Send + 'static) -> Receiver {
        let (tx, rx) = mpsc::channel(8);
        RUNTIME.spawn(async move {
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
        // assumes low contention. also we want an entry eventually
        gvl::nogvl(|| {
            if let Ok(mut inner) = self.0.lock() {
                match inner.blocking_recv() {
                    Some(Ok(entry)) => Some(entry),
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

#[derive(Default)]
struct Inner {
    tx: Option<mpsc::Sender<Result<Bytes, io::Error>>>,
    rx: Option<mpsc::Receiver<Result<Bytes, io::Error>>>,
    closed: bool,
}

#[magnus::wrap(class = "Wreq::Sender", free_immediately, size)]
pub struct Sender(Arc<Mutex<Inner>>);

impl Sender {
    /// Ruby: `Wreq::UploadStream.new(capacity = 8)`
    pub fn new(args: &[Value]) -> Result<Self, Error> {
        let capacity: usize = if let Some(v) = args.first() {
            usize::try_convert(*v).unwrap_or(8)
        } else {
            8
        };

        // channel
        let (tx, rx) = mpsc::channel::<Result<Bytes, io::Error>>(capacity);
        Ok(Sender(Arc::new(Mutex::new(Inner {
            tx: Some(tx),
            rx: Some(rx),
            closed: false,
        }))))
    }

    /// Ruby: `push(data)` where data is String or bytes
    pub fn push(&self, data: RString) -> Result<(), Error> {
        let bytes = data.to_bytes();
        // send with blocking_send without holding GVL
        let sender = {
            if let Ok(inner) = self.0.lock() {
                inner.tx.clone()
            } else {
                None
            }
        };
        if let Some(tx) = sender {
            let res: Result<(), mpsc::error::SendError<Result<Bytes, io::Error>>> =
                gvl::nogvl(|| tx.blocking_send(Ok(bytes)));
            match res {
                Ok(()) => Ok(()),
                Err(_closed) => {
                    let ruby = unsafe { Ruby::get_unchecked() };
                    Err(Error::new(
                        ruby.exception_runtime_error(),
                        "stream already closed",
                    ))
                }
            }
        } else {
            let ruby = unsafe { Ruby::get_unchecked() };
            Err(Error::new(
                ruby.exception_runtime_error(),
                "stream already closed",
            ))
        }
    }

    /// Ruby: `close()`
    pub fn close(&self) -> Result<(), Error> {
        if let Ok(mut inner) = self.0.lock() {
            inner.closed = true;
            inner.tx.take(); // drop sender, receiver will see EOF
            Ok(())
        } else {
            let ruby = unsafe { Ruby::get_unchecked() };
            Err(Error::new(
                ruby.exception_runtime_error(),
                "stream lock poisoned",
            ))
        }
    }

    /// Ruby: `abort(message = nil)` sends an error then closes
    pub fn abort(&self, args: &[Value]) -> Result<(), Error> {
        let message = args.first().and_then(|v| String::try_convert(*v).ok());
        let err = io::Error::other(message.unwrap_or_else(|| "aborted".to_string()));
        let sender = {
            if let Ok(inner) = self.0.lock() {
                inner.tx.clone()
            } else {
                None
            }
        };
        if let Some(tx) = sender {
            let _ = gvl::nogvl(|| tx.blocking_send(Err(err)));
        }
        self.close()
    }

    /// Take the receiver to build a request body stream. Errors if already taken.
    pub fn take_receiver(&self) -> Result<mpsc::Receiver<Result<Bytes, io::Error>>, Error> {
        if let Ok(mut inner) = self.0.lock() {
            if let Some(rx) = inner.rx.take() {
                Ok(rx)
            } else {
                Err(Error::new(
                    ruby!().exception_runtime_error(),
                    "upload stream already consumed",
                ))
            }
        } else {
            Err(Error::new(
                ruby!().exception_runtime_error(),
                "stream lock poisoned",
            ))
        }
    }
}

pub struct ChannelStream {
    rx: mpsc::Receiver<Result<Bytes, io::Error>>,
}

impl ChannelStream {
    pub fn new(rx: mpsc::Receiver<Result<Bytes, io::Error>>) -> Self {
        Self { rx }
    }
}

impl Unpin for ChannelStream {}

impl Stream for ChannelStream {
    type Item = Result<Bytes, io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let streamer_class = gem_module.define_class("Receiver", ruby.class_object())?;
    streamer_class.define_method("each", magnus::method!(Receiver::each, 0))?;

    let upload_class = gem_module.define_class("Sender", ruby.class_object())?;
    upload_class.define_singleton_method("new", function!(Sender::new, -1))?;
    upload_class.define_method("push", method!(Sender::push, 1))?;
    upload_class.define_method("close", method!(Sender::close, 0))?;
    upload_class.define_method("abort", method!(Sender::abort, -1))?;
    Ok(())
}
