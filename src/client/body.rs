mod json;
mod stream;

use bytes::Bytes;
use magnus::{Error, RModule, RString, Ruby, TryConvert, Value, typed_data::Obj};

pub use self::{
    json::Json,
    stream::{Receiver, ReceiverStream, Sender},
};

/// Represents the body of an HTTP request.
/// Supports text, bytes, and streaming bodies (Proc/Enumerator).
pub enum Body {
    /// Static bytes body
    Bytes(Bytes),
    /// Streaming body
    Stream(ReceiverStream<Result<Bytes, std::io::Error>>),
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    stream::include(ruby, gem_module)?;
    Ok(())
}

impl TryConvert for Body {
    fn try_convert(val: Value) -> Result<Self, Error> {
        if let Ok(s) = RString::try_convert(val) {
            return Ok(Body::Bytes(s.to_bytes()));
        }

        let obj = Obj::<Sender>::try_convert(val)?;
        let stream = ReceiverStream::from(&*obj);
        Ok(Body::Stream(stream))
    }
}

impl Body {
    /// Convert to wreq::Body with true streaming via Ruby Queue.
    ///
    /// **Streaming Implementation:**
    /// This uses Ruby's Queue (thread-safe) and spawns a Ruby Thread to read data.
    /// The Ruby thread has GVL access and can safely call Proc/Enumerator methods.
    /// Data is passed through the Queue to Rust, enabling true streaming without
    /// loading everything into memory first.
    pub fn into_wreq_body(self) -> Result<wreq::Body, Error> {
        match self {
            Body::Bytes(b) => Ok(wreq::Body::from(b)),
            Body::Stream(stream) => Ok(wreq::Body::wrap_stream(stream)),
        }
    }
}
