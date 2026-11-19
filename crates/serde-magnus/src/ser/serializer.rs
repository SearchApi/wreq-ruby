use magnus::{IntoValue, Ruby, Value};
use serde::Serialize;

use super::{
    MapSerializer, SeqSerializer, StructSerializer, StructVariantSerializer,
    TupleVariantSerializer, enums::nest,
};
use crate::error::Error;

pub(super) struct Serializer<'r> {
    ruby: &'r Ruby,
}

impl<'r> Serializer<'r> {
    pub fn new(ruby: &'r Ruby) -> Self {
        Self { ruby }
    }
}

impl<'r> serde::Serializer for Serializer<'r> {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'r>;
    type SerializeTuple = SeqSerializer<'r>;
    type SerializeTupleStruct = SeqSerializer<'r>;
    type SerializeTupleVariant = TupleVariantSerializer<'r>;
    type SerializeMap = MapSerializer<'r>;
    type SerializeStruct = StructSerializer<'r>;
    type SerializeStructVariant = StructVariantSerializer<'r>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(value.into_value_with(self.ruby))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.ruby.str_from_slice(value).into_value_with(self.ruby))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(().into_value_with(self.ruby))
    }

    fn serialize_some<Value>(self, value: &Value) -> Result<Self::Ok, Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().into_value_with(self.ruby))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(().into_value_with(self.ruby))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.into_value_with(self.ruby))
    }

    fn serialize_newtype_struct<Value>(
        self,
        _name: &'static str,
        value: &Value,
    ) -> Result<Self::Ok, Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<Value>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &Value,
    ) -> Result<Self::Ok, Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        nest(self.ruby, variant, value.serialize(self)?)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(
            self.ruby,
            self.ruby.ary_new_capa(len.unwrap_or(0)),
        ))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer::new(
            self.ruby,
            variant,
            self.ruby.ary_new_capa(len),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self.ruby, self.ruby.hash_new()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(self.ruby, name))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer::new(
            self.ruby,
            variant,
            self.ruby.hash_new(),
        ))
    }
}
