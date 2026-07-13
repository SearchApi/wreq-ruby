use std::{collections::BTreeMap, fmt};

use ::serde::{Deserialize, Serialize, de::Visitor};
use magnus::{RArray, RHash, RString, Ruby, Value, encoding::EncodingCapable, value::ReprValue};

use super::{deserialize_json, deserialize_ruby, serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Record {
    count: u64,
    enabled: bool,
    tags: Vec<String>,
    note: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct UnitRecord;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct NewtypeRecord(u64);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct TupleRecord(u64, bool, String);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum State {
    Ready,
    Count(u64),
    Progress(u64, bool),
    Failed { message: String },
}

/// Byte sequence that exercises Serde's owned byte-buffer entry points.
#[derive(Debug, PartialEq)]
struct ByteBuffer(Vec<u8>);

impl Serialize for ByteBuffer {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: ::serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for ByteBuffer {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: ::serde::Deserializer<'de>,
    {
        struct ByteBufferVisitor;

        impl<'de> Visitor<'de> for ByteBufferVisitor {
            type Value = ByteBuffer;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an owned byte buffer")
            }

            fn visit_bytes<Error>(self, value: &[u8]) -> Result<Self::Value, Error>
            where
                Error: ::serde::de::Error,
            {
                Ok(ByteBuffer(value.to_vec()))
            }

            fn visit_byte_buf<Error>(self, value: Vec<u8>) -> Result<Self::Value, Error>
            where
                Error: ::serde::de::Error,
            {
                Ok(ByteBuffer(value))
            }
        }

        deserializer.deserialize_byte_buf(ByteBufferVisitor)
    }
}

/// Assert that a supported Serde value survives conversion through Ruby.
fn assert_ruby_round_trip<Input>(ruby: &Ruby, input: Input) -> Result<(), magnus::Error>
where
    Input: fmt::Debug + PartialEq + Serialize + for<'de> Deserialize<'de>,
{
    let value: Value = serialize(ruby, &input)?;
    let output: Input = deserialize_ruby(ruby, value)?;
    assert_eq!(input, output);
    Ok(())
}

/// Assert that a failed conversion preserves Ruby's `TypeError` classification.
fn assert_type_error(error: magnus::Error, message: &str) {
    let error = error.to_string();
    assert!(error.starts_with("TypeError: "), "{error}");
    assert!(error.contains(message), "{error}");
}

/// Verify scalar, option, result, and byte-string conversion behavior.
fn assert_scalar_conversions(ruby: &Ruby) -> Result<(), magnus::Error> {
    assert_ruby_round_trip(ruby, true)?;
    assert_ruby_round_trip(ruby, 1.25_f32)?;
    assert_ruby_round_trip(ruby, 1.25_f64)?;
    assert_ruby_round_trip(ruby, Option::<u64>::None)?;
    assert_ruby_round_trip(ruby, Some(123_u64))?;
    assert_ruby_round_trip(ruby, Result::<u64, String>::Ok(1234))?;
    assert_ruby_round_trip(ruby, Result::<u64, String>::Err("failed".to_owned()))?;

    let character: RString = serialize(ruby, &'\u{2603}')?;
    assert_eq!("\u{2603}", character.to_string()?);
    assert!(character.enc_get() == ruby.utf8_encindex());
    let output: char = deserialize_ruby(ruby, character)?;
    assert_eq!('\u{2603}', output);

    let string: RString = serialize(ruby, &"Hello, world!")?;
    assert_eq!("Hello, world!", string.to_string()?);
    assert!(string.enc_get() == ruby.utf8_encindex());
    assert_eq!(
        "Hello, world!",
        deserialize_ruby::<_, String>(ruby, string)?
    );

    let bytes = ByteBuffer(b"\0binary\xff".to_vec());
    let value: RString = serialize(ruby, &bytes)?;
    assert_eq!(bytes.0.as_slice(), value.to_bytes().as_ref());
    assert!(value.enc_get() == ruby.ascii8bit_encindex());
    assert_eq!(bytes, deserialize_ruby(ruby, value)?);

    let none: Value = serialize(ruby, &Option::<u64>::None)?;
    assert!(none.is_nil());

    let ok: RHash = serialize(ruby, &Result::<u64, String>::Ok(1234))?;
    let value: u64 = ok.aref("Ok")?;
    assert_eq!(1234, value);

    let error: RHash = serialize(ruby, &Result::<u64, String>::Err("failed".to_owned()))?;
    assert_eq!("failed", error.aref::<_, String>("Err")?);
    Ok(())
}

/// Verify collection, tuple, struct, and enum conversion behavior.
fn assert_composite_conversions(ruby: &Ruby) -> Result<(), magnus::Error> {
    assert_ruby_round_trip(ruby, ())?;
    assert_ruby_round_trip(ruby, [1_i64, 2, 3])?;
    assert_ruby_round_trip(ruby, (123_i64, true, "tuple".to_owned()))?;
    assert_ruby_round_trip(ruby, UnitRecord)?;
    assert_ruby_round_trip(ruby, NewtypeRecord(123))?;
    assert_ruby_round_trip(ruby, TupleRecord(123, true, "tuple struct".to_owned()))?;

    let record = Record {
        count: 42,
        enabled: true,
        tags: vec!["ruby".into(), "rust".into()],
        note: Some("present".into()),
    };
    let value: RHash = serialize(ruby, &record)?;
    let count: u64 = value.aref(ruby.to_symbol("count"))?;
    assert_eq!(record.count, count);
    assert_eq!(record, deserialize_ruby(ruby, value)?);

    let map = BTreeMap::from([("first".to_owned(), 1_u64), ("second".to_owned(), 2)]);
    assert_ruby_round_trip(ruby, map)?;

    let tuple = (123_i64, true, "tuple".to_owned());
    let value: RArray = serialize(ruby, &tuple)?;
    assert_eq!(3, value.len());
    let first: i64 = value.entry(0)?;
    assert_eq!(123, first);

    for state in [
        State::Ready,
        State::Count(2),
        State::Progress(3, false),
        State::Failed {
            message: "failed".into(),
        },
    ] {
        assert_ruby_round_trip(ruby, state)?;
    }

    let count: RHash = serialize(ruby, &State::Count(7))?;
    let value: u64 = count.aref("Count")?;
    assert_eq!(7, value);
    Ok(())
}

/// Verify exact typed integer conversion in both deserialization modes.
fn assert_integer_conversions(ruby: &Ruby) -> Result<(), magnus::Error> {
    let value: Value = serialize(ruby, &i128::MIN)?;
    let decimal: String = value.funcall("to_s", ())?;
    assert_eq!(i128::MIN.to_string(), decimal);
    assert_eq!(i128::MIN, deserialize_ruby::<_, i128>(ruby, value)?);
    assert_eq!(i128::MIN, deserialize_json::<_, i128>(ruby, value)?);

    let value: Value = serialize(ruby, &u128::MAX)?;
    let decimal: String = value.funcall("to_s", ())?;
    assert_eq!(u128::MAX.to_string(), decimal);
    assert_eq!(u128::MAX, deserialize_ruby::<_, u128>(ruby, value)?);
    assert_eq!(u128::MAX, deserialize_json::<_, u128>(ruby, value)?);

    let value: Value = serialize(ruby, &u64::MAX)?;
    assert_eq!(u64::MAX, deserialize_ruby::<_, u64>(ruby, value)?);
    assert_eq!(u64::MAX, deserialize_json::<_, u64>(ruby, value)?);
    Ok(())
}

/// Verify unsupported borrowed values and malformed enum shapes return errors.
fn assert_conversion_errors(ruby: &Ruby) -> Result<(), magnus::Error> {
    let error = deserialize_ruby::<_, &str>(ruby, ruby.str_new("borrowed"))
        .expect_err("borrowed strings must not outlive their Ruby value");
    assert_type_error(error, "expected a borrowed string");

    let error = deserialize_ruby::<_, &[u8]>(ruby, ruby.str_new("borrowed"))
        .expect_err("borrowed byte slices must not outlive their Ruby value");
    assert_type_error(error, "can't deserialize into byte slice");

    let variants = ruby.hash_new();
    variants.aset("Ready", ruby.qnil())?;
    variants.aset("Count", 1)?;
    let error = deserialize_ruby::<_, State>(ruby, variants)
        .expect_err("an enum hash must contain exactly one variant");
    assert_type_error(error, "Hash of length 2");
    Ok(())
}

#[test]
fn retains_ruby_serde_conversion_surface() -> Result<(), magnus::Error> {
    // SAFETY: this is the only test that initializes the embedded Ruby VM.
    let ruby = unsafe { magnus::embed::init() };

    assert_scalar_conversions(&ruby)?;
    assert_composite_conversions(&ruby)?;
    assert_integer_conversions(&ruby)?;
    assert_conversion_errors(&ruby)?;
    Ok(())
}
