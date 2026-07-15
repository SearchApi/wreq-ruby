use magnus::{IntoValue, Ruby, Value};

use crate::serde::Error;

/// Wrap serialized enum data in the one-entry hash used by `serde_magnus`.
pub(super) fn nest(
    ruby: &Ruby,
    variant: &'static str,
    data: impl IntoValue,
) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    hash.aset(variant, data)?;
    Ok(hash.into_value_with(ruby))
}
