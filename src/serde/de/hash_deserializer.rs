use std::iter::Peekable;

use ::serde::de::{DeserializeSeed, MapAccess};
use magnus::{RHash, RString, Ruby, Symbol, Value, value::ReprValue};

use super::{Deserializer, Mode, array_enumerator::ArrayEnumerator};
use crate::serde::Error;

/// Serde map access over a Ruby hash.
pub(super) struct HashDeserializer<'ruby> {
    ruby: &'ruby Ruby,
    hash: RHash,
    keys: Peekable<ArrayEnumerator<'ruby>>,
    depth: usize,
    mode: Mode,
}

impl<'ruby> HashDeserializer<'ruby> {
    /// Create map access while preserving Ruby hash insertion order.
    pub(super) fn new(
        ruby: &'ruby Ruby,
        hash: RHash,
        depth: usize,
        mode: Mode,
    ) -> Result<Self, Error> {
        let keys = hash.funcall("keys", ())?;
        Ok(Self {
            ruby,
            hash,
            keys: ArrayEnumerator::new(ruby, keys).peekable(),
            depth,
            mode,
        })
    }

    /// Reject object keys that JSON cannot represent.
    fn validate_key(key: Value) -> Result<(), Error> {
        if RString::from_value(key).is_some() || Symbol::from_value(key).is_some() {
            Ok(())
        } else {
            Err(Error::message(
                "JSON object keys must be String or Symbol values",
            ))
        }
    }
}

impl<'de> MapAccess<'de> for HashDeserializer<'_> {
    type Error = Error;

    fn next_key_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        match self.keys.peek() {
            Some(Ok(key)) => {
                if matches!(self.mode, Mode::Json) {
                    Self::validate_key(*key)?;
                }
                seed.deserialize(Deserializer::with_mode(
                    self.ruby, *key, self.depth, self.mode,
                ))
                .map(Some)
            }
            Some(Err(error)) => Err(Error::message(format!("failed to read map key: {error}"))),
            None => Ok(None),
        }
    }

    fn next_value_seed<Seed>(&mut self, seed: Seed) -> Result<Seed::Value, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        match self.keys.next() {
            Some(Ok(key)) => seed.deserialize(Deserializer::with_mode(
                self.ruby,
                self.hash.aref(key)?,
                self.depth,
                self.mode,
            )),
            Some(Err(error)) => Err(error),
            None => Err(Error::message("map value has no matching key")),
        }
    }
}
