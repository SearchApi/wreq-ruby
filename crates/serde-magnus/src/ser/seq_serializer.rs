use super::Serializer;
use crate::error::Error;
use magnus::{IntoValue, RArray, Ruby, Value};
use serde::{
    Serialize,
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct},
};

pub(super) struct SeqSerializer<'r> {
    ruby: &'r Ruby,
    array: RArray,
}

impl<'r> SeqSerializer<'r> {
    pub fn new(ruby: &'r Ruby, array: RArray) -> SeqSerializer<'r> {
        SeqSerializer { ruby, array }
    }
}

impl<'r> SerializeSeq for SeqSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<Element>(&mut self, element: &Element) -> Result<(), Self::Error>
    where
        Element: Serialize + ?Sized,
    {
        self.array
            .push(element.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        Ok(self.array.into_value_with(self.ruby))
    }
}

impl<'r> SerializeTuple for SeqSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<Element>(&mut self, element: &Element) -> Result<(), Self::Error>
    where
        Element: Serialize + ?Sized,
    {
        <Self as SerializeSeq>::serialize_element(self, element)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<'r> SerializeTupleStruct for SeqSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Field>(&mut self, field: &Field) -> Result<(), Self::Error>
    where
        Field: Serialize + ?Sized,
    {
        <Self as SerializeSeq>::serialize_element(self, field)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}
