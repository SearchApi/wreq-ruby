use crate::error::Error;
use magnus::{IntoValue, Ruby, Value};

pub fn nest<Data>(ruby: &Ruby, variant: &'static str, data: Data) -> Result<Value, Error>
where
    Data: IntoValue,
{
    let hash = ruby.hash_new();
    hash.aset(variant, data)?;
    Ok(hash.into_value_with(ruby))
}
