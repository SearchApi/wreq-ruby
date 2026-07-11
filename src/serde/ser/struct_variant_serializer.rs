use ::serde::{Serialize, ser::SerializeStructVariant};
use magnus::{RHash, Ruby, Value};

use super::{Serializer, enums::nest};
use crate::serde::Error;

/// Serde struct-variant serializer backed by a Ruby hash.
pub(super) struct StructVariantSerializer<'ruby> {
    ruby: &'ruby Ruby,
    variant: &'static str,
    hash: RHash,
}

impl<'ruby> StructVariantSerializer<'ruby> {
    /// Create a struct-variant serializer for an allocated Ruby hash.
    pub(super) fn new(ruby: &'ruby Ruby, variant: &'static str, hash: RHash) -> Self {
        Self {
            ruby,
            variant,
            hash,
        }
    }
}

impl SerializeStructVariant for StructVariantSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.hash
            .aset(
                self.ruby.to_symbol(name),
                value.serialize(Serializer::new(self.ruby))?,
            )
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        nest(self.ruby, self.variant, self.hash)
    }
}
