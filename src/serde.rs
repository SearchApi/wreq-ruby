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
//! checks iterator and map state explicitly, reports unknown option fields and
//! conversion paths, and supports `i128` and `u128` when serializing Rust
//! values to Ruby.

mod de;
mod error;
mod ser;

use ::serde::{Deserialize, Serialize, de::DeserializeOwned};
use indexmap::IndexSet;
use magnus::{IntoValue, Ruby, TryConvert};
use serde_json::Value as JsonValue;

use crate::error::argument_error;

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
/// `serde_magnus::deserialize` function. JSON callers should use
/// [`deserialize_json`] when arbitrary-size integer precision is required.
///
/// # Errors
///
/// Returns the Ruby exception produced when the input cannot be represented by
/// `Output`.
#[allow(dead_code)] // Retained as part of the upstream-compatible Serde surface.
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
///
/// # Errors
///
/// Returns the Ruby exception produced while creating or converting the output
/// value.
pub(crate) fn serialize<Input, Output>(ruby: &Ruby, input: &Input) -> Result<Output, magnus::Error>
where
    Input: Serialize + ?Sized,
    Output: TryConvert,
{
    let value = ser::serialize(ruby, input).map_err(|error| error.into_magnus(ruby))?;
    Output::try_convert(value)
}

/// Deserialize a Ruby value into a validated native JSON tree.
///
/// Unlike [`deserialize`], this uses JSON conversion mode to preserve
/// arbitrary-size integers and reject non-finite floats, unsupported object
/// keys, and excessive nesting.
///
/// # Errors
///
/// Returns a local bridge error for unsupported or invalid JSON values.
pub(crate) fn deserialize_json<Input>(ruby: &Ruby, input: Input) -> Result<JsonValue, Error>
where
    Input: IntoValue,
{
    de::deserialize_json(ruby, input.into_value_with(ruby))
}

/// Validate Ruby option keys without inspecting their values.
///
/// The derived struct is the only list of accepted keys. Every option field is
/// optional, so this pass can collect unknown keys and duplicates without
/// converting or retaining any Ruby value.
///
/// # Errors
///
/// Returns `ArgumentError` for unknown or duplicate keys and `TypeError` for
/// keys that are neither Ruby Symbols nor Strings.
pub(crate) fn validate_option_keys<Input, Output>(
    ruby: &Ruby,
    input: Input,
) -> Result<(), magnus::Error>
where
    Input: IntoValue,
    Output: DeserializeOwned,
{
    let value = input.into_value_with(ruby);
    let mut unknown = IndexSet::new();
    {
        let mut callback = |path: serde_ignored::Path<'_>| {
            unknown.insert(path.to_string());
        };
        serde_ignored::deserialize::<_, _, Output>(
            de::Deserializer::new_option_keys(ruby, value),
            &mut callback,
        )
        .map_err(|error| error.into_option_magnus(ruby, None))?;
    }

    if unknown.is_empty() {
        Ok(())
    } else {
        Err(unknown_options_error(ruby, &unknown))
    }
}

/// Deserialize validated Ruby options and retain the failing field path.
///
/// # Errors
///
/// Returns the Ruby conversion error with the failing option path attached.
pub(crate) fn deserialize_options<Input, Output>(
    ruby: &Ruby,
    input: Input,
) -> Result<Output, magnus::Error>
where
    Input: IntoValue,
    Output: DeserializeOwned,
{
    let value = input.into_value_with(ruby);
    serde_path_to_error::deserialize(de::Deserializer::new(ruby, value)).map_err(|error| {
        let path = error.path().to_string();
        let path = (path != ".").then_some(path.as_str());
        error.into_inner().into_option_magnus(ruby, path)
    })
}

/// Build an `ArgumentError` containing every unknown option name.
fn unknown_options_error(ruby: &Ruby, names: &IndexSet<String>) -> magnus::Error {
    let label = if names.len() == 1 {
        "unknown option"
    } else {
        "unknown options"
    };
    let names = names
        .iter()
        .map(|name| format!(":{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    argument_error(ruby, format!("{label}: {names}"))
}
