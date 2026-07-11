use ::serde::{
    Serialize,
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct},
};
use magnus::{IntoValue, RArray, Ruby, Value};

use super::Serializer;
use crate::serde::Error;

/// Serde sequence serializer backed by a Ruby array.
pub(super) struct SeqSerializer<'ruby> {
    ruby: &'ruby Ruby,
    array: RArray,
}

impl<'ruby> SeqSerializer<'ruby> {
    /// Create a sequence serializer for an allocated Ruby array.
    pub(super) fn new(ruby: &'ruby Ruby, array: RArray) -> Self {
        Self { ruby, array }
    }
}

impl SerializeSeq for SeqSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, element: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.array
            .push(element.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.into_value_with(self.ruby))
    }
}

impl SerializeTuple for SeqSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, element: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, element)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SeqSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, field: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, field)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
