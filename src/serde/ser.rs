mod enums;
mod map_serializer;
mod seq_serializer;
mod struct_serializer;
mod struct_variant_serializer;
mod tuple_variant_serializer;

use ::serde::Serialize;
use magnus::{IntoValue, Ruby, Value};

use super::Error;
use enums::nest;
use map_serializer::MapSerializer;
use seq_serializer::SeqSerializer;
use struct_serializer::StructSerializer;
use struct_variant_serializer::StructVariantSerializer;
use tuple_variant_serializer::TupleVariantSerializer;

/// Serialize any Serde value into Ruby values.
pub(super) fn serialize(ruby: &Ruby, value: &(impl Serialize + ?Sized)) -> Result<Value, Error> {
    value.serialize(Serializer::new(ruby))
}

/// Serde serializer that creates Ruby values.
struct Serializer<'ruby> {
    ruby: &'ruby Ruby,
}

impl<'ruby> Serializer<'ruby> {
    /// Create a serializer with upstream `serde_magnus` behavior.
    pub(super) fn new(ruby: &'ruby Ruby) -> Self {
        Self { ruby }
    }
}

impl<'ruby> ::serde::Serializer for Serializer<'ruby> {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ruby>;
    type SerializeTuple = SeqSerializer<'ruby>;
    type SerializeTupleStruct = SeqSerializer<'ruby>;
    type SerializeTupleVariant = TupleVariantSerializer<'ruby>;
    type SerializeMap = MapSerializer<'ruby>;
    type SerializeStruct = StructSerializer<'ruby>;
    type SerializeStructVariant = StructVariantSerializer<'ruby>;

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
        struct_serializer::integer_to_ruby(self.ruby, &value.to_string())
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
        struct_serializer::integer_to_ruby(self.ruby, &value.to_string())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(value))
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
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().into_value_with(self.ruby))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        nest(
            self.ruby,
            variant,
            value.serialize(Serializer::new(self.ruby))?,
        )
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(
            self.ruby,
            self.ruby.hash_new_capa(len.unwrap_or(0)),
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(self.ruby, name, len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer::new(
            self.ruby,
            variant,
            self.ruby.hash_new_capa(len),
        ))
    }
}
