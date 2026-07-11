use std::fmt;

/// Error produced by the local Ruby and Serde bridge.
#[derive(Debug)]
pub(crate) enum Error {
    Runtime(String),
    Type(String),
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
            Self::Ruby(error) => error,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(message) | Self::Type(message) => formatter.write_str(message),
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
}

impl From<magnus::Error> for Error {
    fn from(error: magnus::Error) -> Self {
        Self::Ruby(error)
    }
}
