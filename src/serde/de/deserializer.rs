use ::serde::forward_to_deserialize_any;
use magnus::{
    Fixnum, Float, Integer, RArray, RBignum, RHash, RString, Ruby, Symbol, Value,
    value::{Qfalse, Qtrue, ReprValue},
};

use super::super::{Error, MAX_JSON_NESTING};
use super::{
    array_deserializer::ArrayDeserializer, enum_deserializer::EnumDeserializer,
    hash_deserializer::HashDeserializer, number_deserializer::NumberDeserializer,
};
use crate::options::NATIVE_OPTION_TOKEN;

/// Implement typed Serde integer entry points with Magnus's checked conversions.
macro_rules! impl_deserialize_integers {
    ($($method:ident => ($visit:ident, $convert:ident)),+ $(,)?) => {
        $(
            fn $method<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
            where
                Visitor: ::serde::de::Visitor<'de>,
            {
                match Integer::from_value(self.value) {
                    Some(value) => visitor.$visit(value.$convert()?),
                    None => self.deserialize_any(visitor),
                }
            }
        )+
    };
}

/// Data model applied to a Ruby value during deserialization.
#[derive(Clone, Copy)]
pub(super) enum Mode {
    /// Preserve the native Ruby-to-Serde conversion behavior.
    Ruby,
    /// Visit option names while skipping their values.
    OptionKeys,
    /// Enforce the JSON data model and preserve arbitrary-size numbers.
    Json,
}

impl Mode {
    /// Return whether JSON-specific validation is enabled.
    fn is_json(self) -> bool {
        matches!(self, Self::Json)
    }
}

/// Serde deserializer over Ruby values.
pub(in crate::serde) struct Deserializer<'ruby> {
    ruby: &'ruby Ruby,
    value: Value,
    depth: usize,
    mode: Mode,
}

impl<'ruby> Deserializer<'ruby> {
    /// Create a deserializer with native `serde_magnus` Ruby behavior.
    pub(in crate::serde) fn new_ruby(ruby: &'ruby Ruby, value: Value) -> Self {
        Self::with_mode(ruby, value, 0, Mode::Ruby)
    }

    /// Create a deserializer that validates option keys without reading values.
    pub(in crate::serde) fn new_option_keys(ruby: &'ruby Ruby, value: Value) -> Self {
        Self::with_mode(ruby, value, 0, Mode::OptionKeys)
    }

    /// Create a JSON deserializer with validation and arbitrary precision.
    pub(super) fn new_json(ruby: &'ruby Ruby, value: Value) -> Self {
        Self::with_mode(ruby, value, 0, Mode::Json)
    }

    /// Create a nested deserializer that inherits its conversion mode.
    pub(super) fn with_mode(ruby: &'ruby Ruby, value: Value, depth: usize, mode: Mode) -> Self {
        Self {
            ruby,
            value,
            depth,
            mode,
        }
    }

    /// Validate and return the depth used by a nested container.
    pub(super) fn nested_depth(&self) -> Result<usize, Error> {
        let depth = self
            .depth
            .checked_add(1)
            .ok_or_else(|| Error::message("JSON nesting depth overflow"))?;
        if self.mode.is_json() && depth > MAX_JSON_NESTING {
            Err(Error::message(format!(
                "JSON nesting exceeds {MAX_JSON_NESTING} levels"
            )))
        } else {
            Ok(depth)
        }
    }
}

impl<'de> ::serde::Deserializer<'de> for Deserializer<'_> {
    type Error = Error;

    fn deserialize_any<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if self.value.is_nil() {
            return visitor.visit_unit();
        }

        if let Some(value) = Qtrue::from_value(self.value) {
            return visitor.visit_bool(value.to_bool());
        }

        if let Some(value) = Qfalse::from_value(self.value) {
            return visitor.visit_bool(value.to_bool());
        }

        if let Some(value) = Fixnum::from_value(self.value) {
            return visitor.visit_i64(value.to_i64());
        }

        if let Some(value) = RBignum::from_value(self.value) {
            if self.mode.is_json() {
                let source: String = value.funcall_public("to_s", ())?;
                return visitor.visit_map(NumberDeserializer::new(source));
            }

            return visitor.visit_i64(value.to_i64()?);
        }

        if let Some(value) = Float::from_value(self.value) {
            let value = value.to_f64();
            if self.mode.is_json() && !value.is_finite() {
                return Err(Error::message("non-finite Float values are not valid JSON"));
            }

            return visitor.visit_f64(value);
        }

        if let Some(value) = RString::from_value(self.value) {
            return visitor.visit_string(value.to_string()?);
        }

        if let Some(value) = Symbol::from_value(self.value) {
            return visitor.visit_string(value.name()?.into_owned());
        }

        if let Some(value) = RArray::from_value(self.value) {
            let depth = self.nested_depth()?;
            return visitor.visit_seq(ArrayDeserializer::new(self.ruby, value, depth, self.mode));
        }

        if let Some(value) = RHash::from_value(self.value) {
            let depth = self.nested_depth()?;
            return visitor.visit_map(HashDeserializer::new(self.ruby, value, depth, self.mode)?);
        }

        Err(Error::type_error(format!(
            "can't deserialize {}",
            // SAFETY: conversion runs while the Ruby GVL is held.
            unsafe { self.value.classname() }
        )))
    }

    fn deserialize_bytes<Visitor>(self, _visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        Err(Error::type_error("can't deserialize into byte slice"))
    }

    fn deserialize_byte_buf<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if let Some(string) = RString::from_value(self.value) {
            // SAFETY: the bytes are copied before any further Ruby API call.
            visitor.visit_byte_buf(unsafe { string.as_slice() }.to_owned())
        } else {
            Err(Error::type_error(format!(
                "no implicit conversion of {} to String",
                // SAFETY: conversion runs while the Ruby GVL is held.
                unsafe { self.value.classname() }
            )))
        }
    }

    fn deserialize_option<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if matches!(self.mode, Mode::OptionKeys) || self.value.is_nil() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_enum<Visitor>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if let Some(variant) = RString::from_value(self.value) {
            return visitor.visit_enum(EnumDeserializer::new(
                self.ruby,
                variant.to_string()?,
                self.ruby.qnil().as_value(),
                self.depth,
                self.mode,
            ));
        }

        if let Some(hash) = RHash::from_value(self.value) {
            if hash.len() == 1 {
                let keys: RArray = hash.funcall("keys", ())?;
                let key: String = keys.entry(0)?;
                let value = hash
                    .get(key.as_str())
                    .unwrap_or_else(|| self.ruby.qnil().as_value());
                return visitor.visit_enum(EnumDeserializer::new(
                    self.ruby, key, value, self.depth, self.mode,
                ));
            }
            return Err(Error::type_error(format!(
                "can't deserialize Hash of length {} to Enum",
                hash.len()
            )));
        }

        Err(Error::type_error(format!(
            "can't deserialize {} to Enum",
            // SAFETY: conversion runs while the Ruby GVL is held.
            unsafe { self.value.classname() }
        )))
    }

    fn deserialize_newtype_struct<Visitor>(
        self,
        name: &'static str,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if name == NATIVE_OPTION_TOKEN {
            visitor.visit_unit()
        } else {
            visitor.visit_newtype_struct(self)
        }
    }

    fn deserialize_ignored_any<Visitor>(
        self,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_identifier<Visitor>(
        self,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: ::serde::de::Visitor<'de>,
    {
        if matches!(self.mode, Mode::OptionKeys) {
            if let Some(value) = RString::from_value(self.value) {
                return visitor.visit_string(value.to_string()?);
            }
            if let Some(value) = Symbol::from_value(self.value) {
                return match value.name()? {
                    std::borrow::Cow::Borrowed(name) => visitor.visit_borrowed_str(name),
                    std::borrow::Cow::Owned(name) => visitor.visit_string(name),
                };
            }
            return Err(Error::type_error("option keys must be Symbols or Strings"));
        }

        self.deserialize_any(visitor)
    }

    impl_deserialize_integers! {
        deserialize_i8 => (visit_i8, to_i8),
        deserialize_i16 => (visit_i16, to_i16),
        deserialize_i32 => (visit_i32, to_i32),
        deserialize_i64 => (visit_i64, to_i64),
        deserialize_i128 => (visit_i128, to_i128),
        deserialize_u8 => (visit_u8, to_u8),
        deserialize_u16 => (visit_u16, to_u16),
        deserialize_u32 => (visit_u32, to_u32),
        deserialize_u64 => (visit_u64, to_u64),
        deserialize_u128 => (visit_u128, to_u128),
    }

    forward_to_deserialize_any! {
        <Visitor: Visitor<'de>>
        bool f32 f64 char str string unit unit_struct seq tuple tuple_struct map struct
    }
}
