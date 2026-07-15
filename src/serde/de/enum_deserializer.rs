use ::serde::de::{DeserializeSeed, EnumAccess, value::StringDeserializer};
use magnus::{Ruby, Value};

use super::{Mode, VariantDeserializer};
use crate::serde::Error;

/// Serde enum access over a Ruby string or one-entry hash.
pub(super) struct EnumDeserializer<'ruby> {
    ruby: &'ruby Ruby,
    variant: String,
    value: Value,
    depth: usize,
    mode: Mode,
}

impl<'ruby> EnumDeserializer<'ruby> {
    /// Create enum access for a variant and its associated Ruby value.
    pub(super) fn new(
        ruby: &'ruby Ruby,
        variant: String,
        value: Value,
        depth: usize,
        mode: Mode,
    ) -> Self {
        Self {
            ruby,
            variant,
            value,
            depth,
            mode,
        }
    }
}

impl<'ruby, 'de> EnumAccess<'de> for EnumDeserializer<'ruby> {
    type Variant = VariantDeserializer<'ruby>;
    type Error = Error;

    fn variant_seed<Seed>(self, seed: Seed) -> Result<(Seed::Value, Self::Variant), Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        let variant = VariantDeserializer::new(self.ruby, self.value, self.depth, self.mode);
        seed.deserialize(StringDeserializer::<Error>::new(self.variant))
            .map(|value| (value, variant))
    }
}
