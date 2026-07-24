use ::serde::{Serialize, ser::SerializeTupleVariant};
use magnus::{RArray, Ruby, Value};

use super::{Serializer, enums::nest};
use crate::serde::Error;

/// Serde tuple-variant serializer backed by a Ruby array.
pub(super) struct TupleVariantSerializer<'ruby> {
    ruby: &'ruby Ruby,
    variant: &'static str,
    array: RArray,
}

impl<'ruby> TupleVariantSerializer<'ruby> {
    /// Create a tuple-variant serializer for an allocated Ruby array.
    pub(super) fn new(ruby: &'ruby Ruby, variant: &'static str, array: RArray) -> Self {
        Self {
            ruby,
            variant,
            array,
        }
    }
}

impl SerializeTupleVariant for TupleVariantSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, field: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.array
            .push(field.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        nest(self.ruby, self.variant, self.array)
    }
}
