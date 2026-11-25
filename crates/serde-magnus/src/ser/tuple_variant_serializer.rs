use super::{Serializer, enums::nest};
use crate::error::Error;
use magnus::{RArray, Ruby, Value};
use serde::{Serialize, ser::SerializeTupleVariant};

pub(super) struct TupleVariantSerializer<'r> {
    ruby: &'r Ruby,
    variant: &'static str,
    array: RArray,
}

impl<'r> TupleVariantSerializer<'r> {
    pub fn new(ruby: &'r Ruby, variant: &'static str, array: RArray) -> TupleVariantSerializer<'r> {
        TupleVariantSerializer {
            ruby,
            variant,
            array,
        }
    }
}

impl<'r> SerializeTupleVariant for TupleVariantSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Field>(&mut self, field: &Field) -> Result<(), Self::Error>
    where
        Field: Serialize + ?Sized,
    {
        self.array
            .push(field.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        nest(self.ruby, self.variant, self.array)
    }
}
