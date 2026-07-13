mod array_deserializer;
mod array_enumerator;
mod deserializer;
mod enum_deserializer;
mod hash_deserializer;
mod number_deserializer;
mod variant_deserializer;

use ::serde::Deserialize;
use magnus::{Ruby, Value};

use super::Error;
use array_deserializer::ArrayDeserializer;
use deserializer::{Deserializer, Mode};
use hash_deserializer::HashDeserializer;
use variant_deserializer::VariantDeserializer;

/// Deserialize one Ruby value using native Ruby data model semantics.
pub(super) fn deserialize_ruby<'de, Output>(ruby: &Ruby, value: Value) -> Result<Output, Error>
where
    Output: Deserialize<'de>,
{
    Output::deserialize(Deserializer::new_ruby(ruby, value))
}

/// Deserialize one Ruby value using JSON-specific conversion rules.
pub(super) fn deserialize_json<'de, Output>(ruby: &Ruby, value: Value) -> Result<Output, Error>
where
    Output: Deserialize<'de>,
{
    Output::deserialize(Deserializer::new_json(ruby, value))
}
