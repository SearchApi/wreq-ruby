use ::serde::{Serialize, ser::SerializeMap};
use magnus::{IntoValue, RHash, Ruby, Value};

use super::Serializer;
use crate::serde::Error;

/// Serde map serializer backed by a Ruby hash.
pub(super) struct MapSerializer<'ruby> {
    ruby: &'ruby Ruby,
    hash: RHash,
    key: Option<Value>,
}

impl<'ruby> MapSerializer<'ruby> {
    /// Create a map serializer for an allocated Ruby hash.
    pub(super) fn new(ruby: &'ruby Ruby, hash: RHash) -> Self {
        Self {
            ruby,
            hash,
            key: None,
        }
    }
}

impl SerializeMap for MapSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.key = Some(key.serialize(Serializer::new(self.ruby))?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key = self
            .key
            .take()
            .ok_or_else(|| Error::message("map value has no matching key"))?;
        self.hash
            .aset(key, value.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.key.is_some() {
            return Err(Error::message("map key has no matching value"));
        }
        Ok(self.hash.into_value_with(self.ruby))
    }
}
