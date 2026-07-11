//! JSON conversion at the Ruby request and response boundary.
//!
//! Request values are converted from Ruby into an owned [`Json`] tree before
//! network I/O. Response bodies take the opposite path: [`parse`] reads JSON
//! bytes and converts the resulting tree back into Ruby values.
//!
//! The underlying `serde_json` configuration preserves object insertion order
//! and arbitrary-size integer tokens in both directions.

use ::serde::{Deserialize, Serialize, de::DeserializeOwned};
use magnus::{Error, Ruby, TryConvert, Value};

use crate::{
    error::{decoding_error_to_magnus, json_serialization_error},
    serde::{from_ruby, serialize},
};

/// An owned JSON tree shared by request and response conversion.
///
/// This wrapper keeps the configured `serde_json::Value` representation private
/// so callers use the same precision and ordering behavior in both directions.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Json(serde_json::Value);

/// Deserialize JSON bytes into an owned Serde value.
///
/// `source` may be a byte slice, `Vec<u8>`, `bytes::Bytes`, or another type that
/// exposes its contents through [`AsRef`]. The output must own its data so
/// temporary response buffers can be consumed safely.
///
/// # Errors
///
/// Returns [`serde_json::Error`] when the document is malformed or cannot be
/// represented by `T`.
pub fn from_slice<T, S>(source: S) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
    S: AsRef<[u8]>,
{
    serde_json::from_slice(source.as_ref())
}

/// Parse response bytes and convert the JSON document into a Ruby value.
///
/// `T` is normally [`magnus::Value`], but any Magnus type implementing
/// [`TryConvert`] can be requested. JSON object order is retained, and integral
/// number tokens are converted to Ruby `Integer` without narrowing.
///
/// # Errors
///
/// Malformed JSON is returned as `Wreq::DecodingError`. Errors raised while
/// creating the requested Ruby value are propagated unchanged.
pub fn parse<T, S>(ruby: &Ruby, source: S) -> Result<T, Error>
where
    T: TryConvert,
    S: AsRef<[u8]>,
{
    let json: Json = from_slice(source).map_err(decoding_error_to_magnus)?;
    serialize(ruby, &json)
}

/// Convert supported Ruby request values into an owned JSON tree.
///
/// Supported values are `Hash`, `Array`, `String`, `Symbol`, `Integer`, finite
/// `Float`, booleans, and `nil`. Hash keys must be strings or symbols. Arrays
/// and hashes are limited to 100 nesting levels, which also bounds cyclic input.
/// Unsupported values are reported as `Wreq::BuilderError` before network I/O.
impl TryConvert for Json {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let ruby = Ruby::get_with(value);
        from_ruby(&ruby, value)
            .map(Self)
            .map_err(|error| json_serialization_error(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{Json, from_slice};

    #[test]
    fn preserves_number_precision_and_object_order() {
        let source = br#"{"second":115792089237316195423570985008687907853269984665640564039457584007913129639936,"first":1}"#;
        let json: Json = from_slice(source).unwrap();

        assert_eq!(source, serde_json::to_vec(&json).unwrap().as_slice());
    }
}
