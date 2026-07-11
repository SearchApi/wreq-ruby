use ::serde::de::{DeserializeSeed, MapAccess, value::StringDeserializer};

use super::super::{Error, JSON_NUMBER_TOKEN};

/// Serde map representation used by `serde_json` for arbitrary-precision numbers.
pub(super) struct NumberDeserializer {
    source: Option<String>,
}

impl NumberDeserializer {
    /// Create a one-entry number map from a Ruby Integer decimal string.
    pub(super) fn new(source: String) -> Self {
        Self {
            source: Some(source),
        }
    }
}

impl<'de> MapAccess<'de> for NumberDeserializer {
    type Error = Error;

    fn next_key_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        if self.source.is_none() {
            return Ok(None);
        }

        seed.deserialize(StringDeserializer::<Error>::new(
            JSON_NUMBER_TOKEN.to_owned(),
        ))
        .map(Some)
    }

    fn next_value_seed<Seed>(&mut self, seed: Seed) -> Result<Seed::Value, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        let source = self
            .source
            .take()
            .ok_or_else(|| Error::message("JSON number value is missing"))?;
        seed.deserialize(StringDeserializer::<Error>::new(source))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(usize::from(self.source.is_some()))
    }
}
