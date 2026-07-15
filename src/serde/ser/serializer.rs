use ::serde::Serialize;
use magnus::{IntoValue, Ruby, Value};

use super::super::Error;
use super::{
    enums::nest, map_serializer::MapSerializer, seq_serializer::SeqSerializer,
    struct_serializer::StructSerializer, struct_variant_serializer::StructVariantSerializer,
    tuple_variant_serializer::TupleVariantSerializer,
};

/// Implement primitive numeric serialization through Magnus's `IntoValue`.
macro_rules! impl_serialize_numbers {
    ($($method:ident => $type:ty),+ $(,)?) => {
        $(
            fn $method(self, value: $type) -> Result<Self::Ok, Self::Error> {
                Ok(value.into_value_with(self.ruby))
            }
        )+
    };
}

/// Serde serializer that creates Ruby values.
pub(super) struct Serializer<'ruby> {
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

    impl_serialize_numbers! {
        serialize_i8 => i8,
        serialize_i16 => i16,
        serialize_i32 => i32,
        serialize_i64 => i64,
        serialize_i128 => i128,
        serialize_u8 => u8,
        serialize_u16 => u16,
        serialize_u32 => u32,
        serialize_u64 => u64,
        serialize_u128 => u128,
        serialize_f32 => f32,
        serialize_f64 => f64,
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
