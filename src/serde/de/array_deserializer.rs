use ::serde::de::{DeserializeSeed, SeqAccess};
use magnus::{RArray, Ruby};

use super::{Deserializer, Mode, array_enumerator::ArrayEnumerator};
use crate::serde::Error;

/// Serde sequence access over a Ruby array.
pub(super) struct ArrayDeserializer<'ruby> {
    ruby: &'ruby Ruby,
    entries: ArrayEnumerator<'ruby>,
    depth: usize,
    mode: Mode,
}

impl<'ruby> ArrayDeserializer<'ruby> {
    /// Create sequence access at the supplied JSON nesting depth.
    pub(super) fn new(ruby: &'ruby Ruby, array: RArray, depth: usize, mode: Mode) -> Self {
        Self {
            ruby,
            entries: ArrayEnumerator::new(ruby, array),
            depth,
            mode,
        }
    }
}

impl<'de> SeqAccess<'de> for ArrayDeserializer<'_> {
    type Error = Error;

    fn next_element_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        match self.entries.next() {
            Some(Ok(entry)) => seed
                .deserialize(Deserializer::with_mode(
                    self.ruby, entry, self.depth, self.mode,
                ))
                .map(Some),
            Some(Err(error)) => Err(error),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.entries.remaining())
    }
}
