//! Parsing and validation for Ruby option hashes.

use std::{fmt, marker::PhantomData};

use ::serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Visitor},
};
use magnus::{Error, RHash, Ruby, TryConvert, Value, value::ReprValue};

use crate::{
    error::{argument_error, option_value_error, type_error},
    serde,
};

/// Private Serde newtype used to recognize values converted by Magnus.
pub(crate) const NATIVE_OPTION_TOKEN: &str = "$wreq::private::NativeOption";

/// An option represented in the Serde schema but converted through Magnus.
///
/// Serde records the field as accepted without traversing its Ruby value. Once
/// the complete option hash has been validated, [`Options::convert`] stores the
/// converted value here.
pub(crate) struct NativeOption<T>(Option<T>);

impl<T> NativeOption<T> {
    /// Replace the value after converting it through Magnus.
    pub(crate) fn set(&mut self, value: Option<T>) {
        self.0 = value;
    }

    /// Take the converted value.
    pub(crate) fn take(&mut self) -> Option<T> {
        self.0.take()
    }
}

impl<T> Default for NativeOption<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<'de, T> Deserialize<'de> for NativeOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NativeOptionVisitor<T>(PhantomData<fn() -> T>);

        impl<T> Visitor<'_> for NativeOptionVisitor<T> {
            type Value = NativeOption<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a Ruby-native option")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(NativeOption::default())
            }
        }

        deserializer
            .deserialize_newtype_struct(NATIVE_OPTION_TOKEN, NativeOptionVisitor(PhantomData))
    }
}

/// A Ruby option hash parsed through its derived Serde schema.
pub(crate) struct Options<'ruby> {
    ruby: &'ruby Ruby,
    hash: RHash,
}

impl<'ruby> Options<'ruby> {
    /// Parse zero or one Ruby options Hash without copying it.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for extra positional arguments and `TypeError`
    /// when the single argument is not a Hash.
    pub(crate) fn from_args(
        ruby: &'ruby Ruby,
        args: &[Value],
        owner: &str,
    ) -> Result<Option<Self>, Error> {
        magnus::scan_args::scan_args::<(), (Option<Value>,), (), (), (), ()>(args)?
            .optional
            .0
            .map(|value| {
                RHash::from_value(value)
                    .map(|hash| Self::new(ruby, hash))
                    .ok_or_else(|| type_error(ruby, format!("{owner} options must be a Hash")))
            })
            .transpose()
    }

    /// Wrap a Ruby Hash without copying its keys or values.
    pub(crate) fn new(ruby: &'ruby Ruby, hash: RHash) -> Self {
        Self { ruby, hash }
    }

    /// Return the original Ruby Hash as a generic value.
    pub(crate) fn as_value(&self) -> Value {
        self.hash.as_value()
    }

    /// Validate option keys without reading values, then borrow the options for chaining.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for unknown or duplicate keys and `TypeError`
    /// when a key is neither a Ruby Symbol nor String.
    pub(crate) fn validate_keys<T>(&self) -> Result<&Self, Error>
    where
        T: DeserializeOwned,
    {
        serde::validate_option_keys::<_, T>(self.ruby, self.hash)?;
        Ok(self)
    }

    /// Deserialize validated option values with field-path error context.
    ///
    /// # Errors
    ///
    /// Returns the Ruby exception produced while converting a known value and
    /// includes its option path in the message.
    pub(crate) fn deserialize<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        serde::deserialize_options(self.ruby, self.hash)
    }

    /// Return whether an option key is present, including a `nil` value.
    pub(crate) fn is_present(&self, name: &str) -> bool {
        get(self.ruby, self.hash, name).is_some()
    }

    /// Return whether an option is present with a non-nil value.
    pub(crate) fn is_non_nil(&self, name: &str) -> bool {
        get(self.ruby, self.hash, name).is_some_and(|value| !value.is_nil())
    }

    /// Create a fluent validator for rules involving this option hash.
    pub(crate) fn validator(&self) -> Validator<'_, 'ruby> {
        Validator::new(self)
    }

    /// Convert a present, non-nil option while retaining its name in errors.
    ///
    /// # Errors
    ///
    /// Returns the Ruby conversion error with the option name added.
    pub(crate) fn convert<T>(&self, name: &str) -> Result<Option<T>, Error>
    where
        T: TryConvert,
    {
        get(self.ruby, self.hash, name)
            .filter(|value| !value.is_nil())
            .map(T::try_convert)
            .transpose()
            .map_err(|error| option_value_error(name, error))
    }

    /// Convert a present option including `nil`, which may be meaningful to `T`.
    ///
    /// # Errors
    ///
    /// Returns the Ruby conversion error with the option name added.
    pub(crate) fn convert_present<T>(&self, name: &str) -> Result<Option<T>, Error>
    where
        T: TryConvert,
    {
        get(self.ruby, self.hash, name)
            .map(T::try_convert)
            .transpose()
            .map_err(|error| option_value_error(name, error))
    }
}

/// A fluent collection of validation rules for one option hash.
///
/// Rules run in declaration order and later rules become no-ops after the
/// first failure, preserving a deterministic error priority without building
/// errors for successful rules.
#[must_use = "call Validator::finish to observe validation errors"]
pub(crate) struct Validator<'options, 'ruby> {
    options: &'options Options<'ruby>,
    error: Option<Error>,
}

impl<'options, 'ruby> Validator<'options, 'ruby> {
    /// Create an empty validation chain.
    fn new(options: &'options Options<'ruby>) -> Self {
        Self {
            options,
            error: None,
        }
    }

    /// Run one rule unless an earlier rule has already failed.
    fn check<F>(mut self, validate: F) -> Self
    where
        F: FnOnce(&Options<'ruby>) -> Result<(), Error>,
    {
        if self.error.is_none() {
            self.error = validate(self.options).err();
        }
        self
    }

    /// Reject a non-nil option when the current target does not support it.
    pub(crate) fn reject_unsupported(self, name: &str, supported: bool) -> Self {
        self.check(|options| {
            if supported || !options.is_non_nil(name) {
                Ok(())
            } else {
                Err(argument_error(
                    options.ruby,
                    format!("option :{name} is not supported on this platform"),
                ))
            }
        })
    }

    /// Reject a group when more than one option has an effective value.
    pub(crate) fn reject_conflicts<const N: usize>(self, options: [(&str, bool); N]) -> Self {
        self.check(|state| {
            if options.iter().filter(|(_, present)| *present).count() < 2 {
                return Ok(());
            }

            let mut message = String::from("mutually exclusive options: ");
            let mut separator = "";
            for (name, present) in options {
                if present {
                    message.push_str(separator);
                    message.push(':');
                    message.push_str(name);
                    separator = ", ";
                }
            }

            Err(argument_error(state.ruby, message))
        })
    }

    /// Require a companion setting when an option is present.
    pub(crate) fn require_when_present(
        self,
        option: &str,
        present: bool,
        effective: bool,
        requirement: &str,
    ) -> Self {
        self.check(|state| {
            if !present || effective {
                Ok(())
            } else {
                Err(argument_error(
                    state.ruby,
                    format!("option :{option} requires {requirement}"),
                ))
            }
        })
    }

    /// Complete validation and return the source options for further processing.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the first failed rule.
    pub(crate) fn finish(self) -> Result<&'options Options<'ruby>, Error> {
        self.error.map_or(Ok(self.options), Err)
    }
}

/// Return an option by either its Symbol key or equivalent String key.
///
/// The Serde schema rejects a hash containing both forms as a duplicate before
/// native values are converted.
pub(crate) fn get(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    hash.get(ruby.to_symbol(name)).or_else(|| hash.get(name))
}
