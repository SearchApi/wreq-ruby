mod enums;
mod map_serializer;
mod seq_serializer;
mod serializer;
mod struct_serializer;
mod struct_variant_serializer;
mod tuple_variant_serializer;

use ::serde::Serialize;
use magnus::{Ruby, Value};

use super::Error;
use serializer::Serializer;

/// Serialize any Serde value into Ruby values.
pub(super) fn serialize(ruby: &Ruby, value: &(impl Serialize + ?Sized)) -> Result<Value, Error> {
    value.serialize(Serializer::new(ruby))
}
