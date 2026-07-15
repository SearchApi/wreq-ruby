//! JSON conversion at the Ruby request and response boundary.
//!
//! Request values are converted from Ruby into an owned [`Json`] tree before
//! network I/O. Response bodies are deserialized into the same tree by wreq and
//! then converted back into Ruby values.
//!
//! The underlying `serde_json` configuration preserves object insertion order
//! and arbitrary-size integer tokens in both directions.

use ::serde::{Deserialize, Serialize};
use magnus::{Error, Ruby, TryConvert, Value};

use crate::{error::json_serialization_error, serde::deserialize_json};

/// An owned JSON tree shared by request and response conversion.
///
/// This wrapper keeps the configured `serde_json::Value` representation private
/// so callers use the same precision and ordering behavior in both directions.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Json(serde_json::Value);

/// Convert supported Ruby request values into an owned JSON tree.
///
/// Supported values are `Hash`, `Array`, `String`, `Symbol`, `Integer`, finite
/// `Float`, booleans, and `nil`. Hash keys must be strings or symbols. Arrays
/// and hashes are limited to 100 nesting levels, which also bounds cyclic input.
/// Unsupported values are reported as `Wreq::BuilderError` before network I/O.
impl TryConvert for Json {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let ruby = Ruby::get_with(value);
        deserialize_json(&ruby, value)
            .map(Self)
            .map_err(json_serialization_error)
    }
}

#[cfg(test)]
mod tests {
    use super::Json;

    #[test]
    fn preserves_number_precision_and_object_order() {
        let source = br#"{"second":115792089237316195423570985008687907853269984665640564039457584007913129639936,"first":1}"#;
        let json: Json = serde_json::from_slice(source).unwrap();

        assert_eq!(source, serde_json::to_vec(&json).unwrap().as_slice());
    }
}
