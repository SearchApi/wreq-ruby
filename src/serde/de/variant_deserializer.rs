use ::serde::de::{DeserializeSeed, Unexpected, VariantAccess};
use magnus::{RArray, RHash, Ruby, Value, value::ReprValue};

use super::{ArrayDeserializer, Deserializer, HashDeserializer, Mode};
use crate::serde::Error;

/// Serde access to the payload of a Ruby enum representation.
pub(super) struct VariantDeserializer<'ruby> {
    ruby: &'ruby Ruby,
    value: Value,
    depth: usize,
    mode: Mode,
}

impl<'ruby> VariantDeserializer<'ruby> {
    /// Create variant access for a Ruby payload.
    pub(super) fn new(ruby: &'ruby Ruby, value: Value, depth: usize, mode: Mode) -> Self {
        Self {
            ruby,
            value,
            depth,
            mode,
        }
    }

    /// Return the depth assigned to a container payload.
    fn nested_depth(&self) -> Result<usize, Error> {
        Deserializer::with_mode(self.ruby, self.value, self.depth, self.mode).nested_depth()
    }
}

impl<'de> VariantAccess<'de> for VariantDeserializer<'_> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.value.is_nil() {
            Ok(())
        } else {
            Err(::serde::de::Error::invalid_type(
                Unexpected::Other(
                    // SAFETY: conversion runs while the Ruby GVL is held.
                    &unsafe { self.value.classname() },
                ),
                &"unit variant",
            ))
        }
    }

    fn newtype_variant_seed<Seed>(self, seed: Seed) -> Result<Seed::Value, Self::Error>
    where
        Seed: DeserializeSeed<'de>,
    {
        seed.deserialize(Deserializer::with_mode(
            self.ruby, self.value, self.depth, self.mode,
        ))
    }

    fn tuple_variant<Visitor>(
        self,
        _len: usize,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if let Some(array) = RArray::from_value(self.value) {
            let depth = self.nested_depth()?;
            visitor.visit_seq(ArrayDeserializer::new(self.ruby, array, depth, self.mode))
        } else {
            Err(::serde::de::Error::invalid_type(
                Unexpected::Other(
                    // SAFETY: conversion runs while the Ruby GVL is held.
                    &unsafe { self.value.classname() },
                ),
                &"tuple variant",
            ))
        }
    }

    fn struct_variant<Visitor>(
        self,
        _fields: &'static [&'static str],
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if let Some(hash) = RHash::from_value(self.value) {
            let depth = self.nested_depth()?;
            visitor.visit_map(HashDeserializer::new(self.ruby, hash, depth, self.mode)?)
        } else {
            Err(::serde::de::Error::invalid_type(
                Unexpected::Other(
                    // SAFETY: conversion runs while the Ruby GVL is held.
                    &unsafe { self.value.classname() },
                ),
                &"struct variant",
            ))
        }
    }
}
