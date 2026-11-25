//! Serde-based validation and conversion of Ruby option hashes.

use std::fmt;

use indexmap::IndexSet;
use magnus::{Error, IntoValue, Ruby, error::ErrorType};
use serde::de::DeserializeOwned;
use serde_ignored::Path;
use serde_path_to_error::{Deserializer as PathDeserializer, Path as ErrorPath, Track};

use crate::{de::Deserializer, error::Error as SerdeError};

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
pub fn validate_option_keys<Input, Output>(ruby: &Ruby, input: Input) -> Result<(), Error>
where
    Input: IntoValue,
    Output: DeserializeOwned,
{
    let value: magnus::Value = input.into_value_with(ruby);
    let mut unknown = IndexSet::new();
    {
        let mut callback = |path: Path<'_>| {
            if let Path::Map { key, .. } = path {
                unknown.insert(key);
            }
        };
        serde_ignored::deserialize::<_, _, Output>(
            Deserializer::new_option_keys(ruby, value),
            &mut callback,
        )
        .map_err(|error| option_error(ruby, error, None))?;
    }

    if unknown.is_empty() {
        return Ok(());
    }

    let label = if unknown.len() == 1 {
        "unknown option"
    } else {
        "unknown options"
    };
    let mut message = String::from(label);
    message.push(':');
    for (index, name) in unknown.into_iter().enumerate() {
        if index > 0 {
            message.push(',');
        }
        message.push(' ');
        message.push(':');
        message.push_str(&name);
    }

    Err(Error::new(ruby.exception_arg_error(), message))
}

/// Deserialize validated Ruby options and retain the failing field path.
///
/// # Errors
///
/// Returns the Ruby conversion error with the failing option path attached.
pub fn deserialize_options<Input, Output>(ruby: &Ruby, input: Input) -> Result<Output, Error>
where
    Input: IntoValue,
    Output: DeserializeOwned,
{
    let value = input.into_value_with(ruby);
    let mut track = Track::new();
    let result = Output::deserialize(PathDeserializer::new(
        Deserializer::new(ruby, value),
        &mut track,
    ));

    result.map_err(|error| {
        let path = track.path();
        let path = path.iter().next().is_some().then_some(&path);
        option_error(ruby, error, path)
    })
}

/// Prefix a Magnus error while preserving its original Ruby exception class.
pub fn contextualize_magnus_error(err: Error, context: fmt::Arguments<'_>) -> Error {
    match err.error_type() {
        ErrorType::Error(class, message) => Error::new(*class, format!("{context}: {message}")),
        ErrorType::Exception(exception) => Error::new(
            exception.exception_class(),
            format!("{context}: {exception}"),
        ),
        ErrorType::Jump(_) => err,
    }
}

/// Convert a Serde error into the Ruby exception expected for an option.
fn option_error(ruby: &Ruby, error: SerdeError, path: Option<&ErrorPath>) -> Error {
    match error {
        SerdeError::Runtime(message) => Error::new(
            ruby.exception_arg_error(),
            contextualize_message(path, message),
        ),
        SerdeError::Type(message) => Error::new(
            ruby.exception_type_error(),
            contextualize_message(path, message),
        ),
        SerdeError::DuplicateField(field) => Error::new(
            ruby.exception_arg_error(),
            format!("duplicate option: :{field}"),
        ),
        SerdeError::Ruby(error) if error.is_kind_of(ruby.exception_range_error()) => Error::new(
            ruby.exception_arg_error(),
            magnus_error_message(&error, path),
        ),
        SerdeError::Ruby(error) => match path {
            Some(path) => {
                contextualize_magnus_error(error, format_args!("invalid value for :{path}"))
            }
            None => error,
        },
    }
}

/// Prefix an error message when Serde identified the failing option path.
fn contextualize_message(path: Option<&ErrorPath>, message: String) -> String {
    match path {
        Some(path) => format!("invalid value for :{path}: {message}"),
        None => message,
    }
}

/// Build the final Ruby error message without an intermediate context String.
fn magnus_error_message(error: &Error, path: Option<&ErrorPath>) -> String {
    match (path, error.error_type()) {
        (Some(path), ErrorType::Error(_, message)) => {
            format!("invalid value for :{path}: {message}")
        }
        (Some(path), ErrorType::Exception(exception)) => {
            format!("invalid value for :{path}: {exception}")
        }
        (Some(path), ErrorType::Jump(_)) => format!("invalid value for :{path}: {error}"),
        (None, ErrorType::Error(_, message)) => message.as_ref().to_owned(),
        (None, _) => error.to_string(),
    }
}
