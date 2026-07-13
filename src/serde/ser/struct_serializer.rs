use ::serde::{Serialize, ser::SerializeStruct};
use magnus::{Integer, IntoValue, RHash, RString, Ruby, Value, value::ReprValue};

use super::Serializer;
use crate::serde::{Error, JSON_NUMBER_TOKEN};

/// Serde struct serializer, including `serde_json` arbitrary-precision numbers.
pub(super) enum StructSerializer<'ruby> {
    Map {
        ruby: &'ruby Ruby,
        hash: RHash,
    },
    Number {
        ruby: &'ruby Ruby,
        value: Option<Value>,
    },
}

impl<'ruby> StructSerializer<'ruby> {
    /// Create the serializer selected by the Serde struct name.
    pub(super) fn new(ruby: &'ruby Ruby, name: &'static str, len: usize) -> Self {
        if name == JSON_NUMBER_TOKEN {
            Self::Number { ruby, value: None }
        } else {
            Self::Map {
                ruby,
                hash: ruby.hash_new_capa(len),
            }
        }
    }
}

impl SerializeStruct for StructSerializer<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        match self {
            Self::Map { ruby, hash } => hash
                .aset(
                    ruby.to_symbol(name),
                    value.serialize(Serializer::new(ruby))?,
                )
                .map_err(Into::into),
            Self::Number {
                ruby,
                value: output,
            } => {
                if name != JSON_NUMBER_TOKEN {
                    return Err(Error::message("invalid arbitrary-precision number field"));
                }

                if output.is_some() {
                    return Err(Error::message(
                        "arbitrary-precision number has duplicate fields",
                    ));
                }

                let source = value.serialize(Serializer::new(ruby))?;
                let source = RString::from_value(source)
                    .ok_or_else(|| Error::message("JSON number token must be a String"))?
                    .to_string()?;
                *output = Some(number_to_ruby(ruby, &source)?);
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Map { ruby, hash } => Ok(hash.into_value_with(ruby)),
            Self::Number { value, .. } => {
                value.ok_or_else(|| Error::message("JSON number token is missing"))
            }
        }
    }
}

/// Convert a validated JSON number token into a Ruby Integer or Float.
fn number_to_ruby(ruby: &Ruby, source: &str) -> Result<Value, Error> {
    if is_integral_number(source) {
        integer_to_ruby(ruby, source)
    } else {
        let value = source.parse::<f64>().map_err(|error| {
            Error::message(format!("failed to convert JSON number {source}: {error}"))
        })?;
        Ok(ruby.float_from_f64(value).as_value())
    }
}

/// Convert a decimal integer token into an arbitrary-precision Ruby Integer.
fn integer_to_ruby(ruby: &Ruby, source: &str) -> Result<Value, Error> {
    let source = ruby.str_new(source);
    let value: Integer = ruby.module_kernel().funcall("Integer", (source, 10))?;
    Ok(value.as_value())
}

/// Return whether a validated JSON number token is integral.
fn is_integral_number(source: &str) -> bool {
    !source
        .bytes()
        .any(|byte| matches!(byte, b'.' | b'e' | b'E'))
}
