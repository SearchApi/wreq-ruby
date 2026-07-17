pub mod form;
pub mod json;
pub mod stream;

use bytes::Bytes;
use futures_util::StreamExt;
use magnus::{
    Error, Module, Object, RModule, RString, Ruby, TryConvert, Value, function, method,
    typed_data::Obj,
};

/// Represents the body of an HTTP request.
/// Supports text, bytes, and streaming bodies (Proc/Enumerator).
pub enum Body {
    /// Static bytes body
    Bytes(Bytes),
    /// Streaming body
    Stream(stream::ReceiverStream<Bytes>),
}

impl TryConvert for Body {
    fn try_convert(val: Value) -> Result<Self, Error> {
        let ruby = Ruby::get_with(val);
        if let Ok(s) = RString::try_convert(val) {
            return Ok(Body::Bytes(s.to_bytes()));
        }

        let obj = Obj::<stream::BodySender>::try_convert(val)?;
        let stream = obj.take_receiver(&ruby)?;
        Ok(Body::Stream(stream))
    }
}

impl From<Body> for wreq::Body {
    fn from(body: Body) -> Self {
        match body {
            Body::Bytes(b) => wreq::Body::from(b),
            Body::Stream(stream) => {
                let try_stream = stream.map(Ok::<Bytes, std::io::Error>);
                wreq::Body::wrap_stream(try_stream)
            }
        }
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let sender_class = gem_module.define_class("BodySender", ruby.class_object())?;
    sender_class.define_singleton_method("new", function!(stream::BodySender::new, -1))?;
    sender_class.define_method("push", method!(stream::BodySender::push, 1))?;
    sender_class.define_method("close", magnus::method!(stream::BodySender::close, 0))?;
    sender_class.define_method("closed?", magnus::method!(stream::BodySender::is_closed, 0))?;
    Ok(())
}
