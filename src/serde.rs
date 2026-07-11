/*
Copyright 2022 George Claghorn

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Serde integration for Magnus with JSON-specific conversion behavior.
//!
//! This module is adapted from `serde_magnus` 0.11.0 and retains its generic
//! Ruby serialization and deserialization surface. Local changes add a JSON
//! mode with arbitrary-size integer support, finite-float and object-key
//! validation, insertion-order preservation, and bounded container nesting.
//! The bridge also avoids the upstream `Ruby::get().unwrap()` error path,
//! checks iterator and map state explicitly, and supports `i128` and `u128`
//! when serializing Rust values to Ruby.

mod de;
mod error;
mod ser;

use ::serde::{Deserialize, Serialize};
use magnus::{IntoValue, Ruby, TryConvert, Value};
use serde_json::Value as JsonValue;

pub(super) use error::Error;

/// Private map key used to carry arbitrary-precision numbers through Serde.
///
/// This mirrors the representation used by `serde_json` when its
/// `arbitrary_precision` feature is enabled. The precision tests protect this
/// integration point when `serde_json` is updated.
pub(super) const JSON_NUMBER_TOKEN: &str = "$serde_json::private::Number";

/// Maximum nesting accepted while converting a Ruby request value.
pub(super) const MAX_JSON_NESTING: usize = 100;

/// Deserialize a Ruby value into any Serde type.
///
/// This preserves the public conversion behavior provided by the upstream
/// `serde_magnus::deserialize` function.
pub(crate) fn deserialize<'de, Input, Output>(
    ruby: &Ruby,
    input: Input,
) -> Result<Output, magnus::Error>
where
    Input: IntoValue,
    Output: Deserialize<'de>,
{
    de::deserialize(ruby, input.into_value_with(ruby)).map_err(|error| error.into_magnus(ruby))
}

/// Serialize any Serde value into a Ruby value.
///
/// This preserves the public conversion behavior provided by the upstream
/// `serde_magnus::serialize` function.
pub(crate) fn serialize<Input, Output>(ruby: &Ruby, input: &Input) -> Result<Output, magnus::Error>
where
    Input: Serialize + ?Sized,
    Output: TryConvert,
{
    let value = ser::serialize(ruby, input).map_err(|error| error.into_magnus(ruby))?;
    Output::try_convert(value)
}

/// Deserialize a Ruby value into a native JSON tree.
pub(crate) fn from_ruby(ruby: &Ruby, value: Value) -> Result<JsonValue, Error> {
    de::deserialize_json(ruby, value)
}
