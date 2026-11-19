use super::{Serializer, enums::nest};
use crate::error::Error;
use magnus::{RHash, Ruby, Value};
use serde::{Serialize, ser::SerializeStructVariant};

pub(super) struct StructVariantSerializer<'r> {
    ruby: &'r Ruby,
    variant: &'static str,
    hash: RHash,
}

impl<'r> StructVariantSerializer<'r> {
    pub fn new(ruby: &'r Ruby, variant: &'static str, hash: RHash) -> StructVariantSerializer<'r> {
        StructVariantSerializer {
            ruby,
            variant,
            hash,
        }
    }
}

impl<'r> SerializeStructVariant for StructVariantSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Value>(
        &mut self,
        name: &'static str,
        value: &Value,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + ?Sized,
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
