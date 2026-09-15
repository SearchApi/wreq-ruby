//! Shared helpers

use magnus::{RString, TryConvert, Value, value::ReprValue};

/// Convert a Ruby value to a file-system path `String`.
///
/// Accepts a plain `String` or any object responding to `to_path` (e.g. `Pathname`).
pub(crate) fn convert_path(value: Value) -> Result<String, magnus::Error> {
    if let Ok(path) = value.funcall::<_, _, RString>("to_path", ()) {
        return path.to_string();
    }
    String::try_convert(value)
}
