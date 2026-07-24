use std::fmt;

use magnus::error::ErrorType;
use serde_path_to_error::Path;

use crate::error::contextualize_magnus_error;

/// Error produced by the local Ruby and Serde bridge.
#[derive(Debug)]
pub(crate) enum Error {
    Runtime(String),
    Type(String),
    DuplicateField(&'static str),
    Ruby(magnus::Error),
}

impl Error {
    /// Create a bridge error from a human-readable message.
    pub(super) fn message(message: impl Into<String>) -> Self {
        Self::Runtime(message.into())
    }

    /// Create a type mismatch error.
    pub(super) fn type_error(message: impl Into<String>) -> Self {
        Self::Type(message.into())
    }

    /// Convert this bridge error into the matching Ruby exception.
    pub(super) fn into_magnus(self, ruby: &magnus::Ruby) -> magnus::Error {
        match self {
            Self::Runtime(message) => magnus::Error::new(ruby.exception_runtime_error(), message),
            Self::Type(message) => magnus::Error::new(ruby.exception_type_error(), message),
            Self::DuplicateField(field) => magnus::Error::new(
                ruby.exception_runtime_error(),
                format!("duplicate field `{field}`"),
            ),
            Self::Ruby(error) => error,
        }
    }

    /// Convert an option value error into Ruby's normal argument categories.
    pub(super) fn into_option_magnus(
        self,
        ruby: &magnus::Ruby,
        path: Option<&Path>,
    ) -> magnus::Error {
        match self {
            Self::Runtime(message) => magnus::Error::new(
                ruby.exception_arg_error(),
                contextualize_message(path, message),
            ),
            Self::Type(message) => magnus::Error::new(
                ruby.exception_type_error(),
                contextualize_message(path, message),
            ),
            Self::DuplicateField(field) => magnus::Error::new(
                ruby.exception_arg_error(),
                format!("duplicate option: :{field}"),
            ),
            Self::Ruby(error) if error.is_kind_of(ruby.exception_range_error()) => {
                magnus::Error::new(
                    ruby.exception_arg_error(),
                    magnus_error_message(&error, path),
                )
            }
            Self::Ruby(error) => match path {
                Some(path) => {
                    contextualize_magnus_error(error, format_args!("invalid value for :{path}"))
                }
                None => error,
            },
        }
    }
}

/// Prefix an error message when Serde identified the failing option path.
fn contextualize_message(path: Option<&Path>, message: String) -> String {
    match path {
        Some(path) => format!("invalid value for :{path}: {message}"),
        None => message,
    }
}

/// Build the final Ruby error message without an intermediate context String.
fn magnus_error_message(error: &magnus::Error, path: Option<&Path>) -> String {
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

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(message) | Self::Type(message) => formatter.write_str(message),
            Self::DuplicateField(field) => write!(formatter, "duplicate field `{field}`"),
            Self::Ruby(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for Error {}

impl ::serde::ser::Error for Error {
    fn custom<T>(message: T) -> Self
    where
        T: fmt::Display,
    {
        Self::message(message.to_string())
    }
}

impl ::serde::de::Error for Error {
    fn custom<T>(message: T) -> Self
    where
        T: fmt::Display,
    {
        Self::message(message.to_string())
    }

    fn invalid_type(
        unexpected: ::serde::de::Unexpected<'_>,
        expected: &dyn ::serde::de::Expected,
    ) -> Self {
        Self::type_error(format!(
            "invalid type: expected {expected}, got {unexpected}"
        ))
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self::DuplicateField(field)
    }
}

impl From<magnus::Error> for Error {
    fn from(error: magnus::Error) -> Self {
        Self::Ruby(error)
    }
}
