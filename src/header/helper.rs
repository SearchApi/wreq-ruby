//! Ruby value conversion helpers for `Wreq::Headers`.

use bytes::Bytes;
use http::{HeaderName, HeaderValue};
use magnus::{Error, RArray, RString, Symbol, TryConvert, Value, prelude::*, typed_data::Obj};

use crate::error::{
    header_name_error_to_magnus, header_value_error_to_magnus, type_value_error_to_magnus,
};

use super::Headers;

/// Maximum number of field-value occurrences supported by `HeaderMap`.
const MAX_HEADER_ENTRIES: usize = 1 << 15;

/// Build a header collection from a Ruby source object.
///
/// Accepts another `Wreq::Headers`, a Hash, or any object whose `to_a` result
/// contains two-element name-value pairs. Array values are delegated to
/// [`Headers::append`] so each value remains a separate occurrence.
pub(super) fn from_source(source: Value) -> Result<Headers, Error> {
    if let Ok(headers) = Obj::<Headers>::try_convert(source) {
        return Ok((*headers).clone());
    }
    if !source.respond_to("to_a", false)? {
        return Err(type_value_error_to_magnus(
            "Expected Headers, a Hash, or an enumerable of pairs",
        ));
    }

    let pairs: RArray = source.funcall_public("to_a", ())?;
    let headers = Headers::default();
    for pair in pairs {
        let pair = RArray::try_convert(pair)
            .map_err(|_| type_value_error_to_magnus("Expected each header entry to be a pair"))?;
        if pair.len() != 2 {
            return Err(type_value_error_to_magnus(
                "Expected each header entry to contain a name and value",
            ));
        }

        headers.append(pair.entry(0)?, pair.entry(1)?)?;
    }
    Ok(headers)
}

/// Convert a Ruby String or Symbol into a normalized HTTP header name.
///
/// Symbol underscores are changed to hyphens before [`HeaderName`] validates
/// and normalizes the name. Other Ruby types produce `Wreq::BuilderError`.
pub(super) fn parse_header_name(value: Value) -> Result<HeaderName, Error> {
    let name = match (RString::from_value(value), Symbol::from_value(value)) {
        (Some(name), _) => name.to_bytes(),
        (None, Some(name)) => Bytes::from(name.name()?.replace('_', "-")),
        (None, None) => {
            return Err(type_value_error_to_magnus(
                "Expected a String or Symbol header name",
            ));
        }
    };
    HeaderName::from_bytes(name.as_ref()).map_err(header_name_error_to_magnus)
}

/// Convert a Ruby String or Array of Strings into validated header values.
///
/// Each Array element becomes one [`HeaderValue`]. An empty Array therefore
/// produces no values, allowing `set` to remove a header and `append` to do
/// nothing.
pub(super) fn parse_header_values(value: Value) -> Result<Vec<HeaderValue>, Error> {
    if let Some(values) = RArray::from_value(value) {
        values.into_iter().map(parse_header_value).collect()
    } else {
        Ok(vec![parse_header_value(value)?])
    }
}

/// Convert one Ruby String into a validated HTTP header value.
///
/// Invalid Ruby types and bytes rejected by [`HeaderValue`] are mapped to
/// `Wreq::BuilderError`.
fn parse_header_value(value: Value) -> Result<HeaderValue, Error> {
    let value = RString::try_convert(value)
        .map_err(|_| type_value_error_to_magnus("Expected a String header value"))?;
    HeaderValue::from_maybe_shared(value.to_bytes()).map_err(header_value_error_to_magnus)
}

/// Validate the resulting number of header occurrences before a mutation.
///
/// `current` is the collection length, `replaced` is the number of existing
/// occurrences removed by `set`, and `added` is the incoming value count.
/// Checked arithmetic prevents overflow; an invalid calculation or a result
/// above the native [`HeaderMap`](http::HeaderMap) limit returns
/// `Wreq::BuilderError` without mutating the collection.
pub(super) fn ensure_header_count(
    current: usize,
    replaced: usize,
    added: usize,
) -> Result<(), Error> {
    let count = current
        .checked_sub(replaced)
        .and_then(|count| count.checked_add(added));
    if count.is_some_and(|count| count <= MAX_HEADER_ENTRIES) {
        Ok(())
    } else {
        Err(header_count_error())
    }
}

/// Build the error returned when the native header map reaches its entry limit.
pub(super) fn header_count_error() -> Error {
    type_value_error_to_magnus("Header collection exceeds 32,768 entries")
}
