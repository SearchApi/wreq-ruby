use std::{
    borrow::Cow,
    cell::{BorrowError, BorrowMutError},
    fmt,
};

use magnus::{
    Attr, Class, Error as MagnusError, Exception, RModule, RObject, Ruby, TryConvert,
    error::ErrorType, exception::ExceptionClass, prelude::*, value::Lazy,
};
use tokio::sync::mpsc::error::SendError;

const ERROR_PREDICATES_IVAR: &str = "wreq_error_predicates";

const RACE_CONDITION_ERROR_MSG: &str = r#"Due to Rust's memory management with borrowing,
you cannot use certain instances multiple times as they may be consumed.

This error can occur in the following cases:
1) You passed a non-clonable instance to a function that requires ownership.
2) You attempted to use a method that consumes ownership more than once (e.g., reading a response body twice).
3) You tried to reference an instance after it was borrowed.

Potential solutions:
1) Avoid sharing instances; create a new instance each time you use it.
2) Refrain from performing actions that consume ownership multiple times.
3) Change the order of operations to reference the instance before borrowing it.
"#;

macro_rules! define_exception {
    ($name:ident, $ruby_name:literal, $parent_method:ident) => {
        static $name: Lazy<ExceptionClass> = Lazy::new(|ruby| {
            ruby.class_object()
                .const_get::<_, RModule>(crate::RUBY_MODULE_NAME)
                .and_then(|module| module.const_get::<_, ExceptionClass>($ruby_name))
                .unwrap_or_else(|_| ruby.$parent_method())
        });
    };
}

macro_rules! initialize_exception {
    ($ruby:expr, $module:expr, $name:ident, $ruby_name:literal, $parent:expr) => {{
        $module.define_error($ruby_name, $parent)?;
        Lazy::force(&$name, $ruby);
    }};
}

macro_rules! define_error_mapping {
    ($($predicate:ident: $method:ident => $class:ident $(($ruby_name:literal))?),+ $(,)?) => {
        /// Native predicates ordered by Ruby exception class priority.
        #[derive(Clone, Copy)]
        #[repr(u8)]
        enum ErrorPredicate {
            $($predicate),+
        }

        impl ErrorPredicate {
            const CLASSIFICATION_ORDER: &'static [Self] = &[$(Self::$predicate),+];

            /// Return this predicate's position in the compact Ruby metadata.
            const fn mask(self) -> u16 {
                1_u16 << (self as u8)
            }

            /// Evaluate this predicate before the native error is consumed.
            fn matches_wreq(self, error: &wreq::Error) -> bool {
                match self {
                    $(Self::$predicate => error.$method(),)+
                }
            }

            /// Return the Ruby class selected when this predicate has priority.
            fn error_class(self) -> &'static Lazy<ExceptionClass> {
                match self {
                    $(Self::$predicate => &$class,)+
                }
            }
        }

        const _: () =
            assert!(ErrorPredicate::CLASSIFICATION_ORDER.len() <= u16::BITS as usize);

        $(
            $(define_exception!($class, $ruby_name, exception_runtime_error);)?
        )+

        $(
            fn $method(rb_self: RObject) -> Result<bool, MagnusError> {
                error_has_predicate(rb_self, ErrorPredicate::$predicate)
            }
        )+

        /// Define the native wreq predicate methods on Wreq::Error.
        fn include_error_predicates(class: ExceptionClass) -> Result<(), MagnusError> {
            $(
                class.define_method(stringify!($method), magnus::method!($method, 0))?;
            )+
            Ok(())
        }

        /// Define and retain every mapped Ruby exception class.
        fn initialize_mapped_errors(
            ruby: &Ruby,
            gem_module: &RModule,
            parent: ExceptionClass,
        ) -> Result<(), MagnusError> {
            $(
                $(
                    initialize_exception!(ruby, gem_module, $class, $ruby_name, parent);
                )?
            )+
            Ok(())
        }
    };
}

// The first matching entry determines the Ruby exception class.
define_error_mapping! {
    Builder: is_builder => BUILDER_ERROR("BuilderError"),
    Body: is_body => BODY_ERROR("BodyError"),
    Tls: is_tls => TLS_ERROR("TlsError"),
    ConnectionReset: is_connection_reset => CONNECTION_RESET_ERROR("ConnectionResetError"),
    Connect: is_connect => CONNECTION_ERROR("ConnectionError"),
    ProxyConnect: is_proxy_connect => PROXY_CONNECTION_ERROR("ProxyConnectionError"),
    Decode: is_decode => DECODING_ERROR("DecodingError"),
    Redirect: is_redirect => REDIRECT_ERROR("RedirectError"),
    Timeout: is_timeout => TIMEOUT_ERROR("TimeoutError"),
    Status: is_status => STATUS_ERROR("StatusError"),
    Request: is_request => REQUEST_ERROR("RequestError"),
    Upgrade: is_upgrade => WREQ_ERROR,
}

/// Native predicates retained after consuming a wreq error.
#[derive(Clone, Copy, Default)]
struct ErrorPredicates(u16);

impl ErrorPredicates {
    /// Restore predicates from compact Ruby metadata.
    const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Return the compact representation stored on the Ruby exception.
    const fn bits(self) -> u16 {
        self.0
    }

    /// Return whether the set contains a native predicate.
    const fn contains(self, predicate: ErrorPredicate) -> bool {
        self.0 & predicate.mask() != 0
    }

    /// Include a predicate when its native check succeeds.
    #[must_use]
    const fn include_if(mut self, predicate: ErrorPredicate, include: bool) -> Self {
        if include {
            self.0 |= predicate.mask();
        }
        self
    }
}

impl From<&wreq::Error> for ErrorPredicates {
    /// Snapshot every native predicate before consuming the wreq error.
    fn from(error: &wreq::Error) -> Self {
        ErrorPredicate::CLASSIFICATION_ORDER
            .iter()
            .copied()
            .fold(Self::default(), |predicates, predicate| {
                predicates.include_if(predicate, predicate.matches_wreq(error))
            })
    }
}

/// Native error details retained after converting a wreq error to Ruby.
struct ErrorMetadata<'a> {
    uri: Option<&'a str>,
    status: Option<wreq::StatusCode>,
    predicates: ErrorPredicates,
}

// Stable roots for native errors.
define_exception!(WREQ_ERROR, "Error", exception_runtime_error);
define_exception!(INTERRUPT_ERROR, "InterruptError", exception_interrupt);

// System-level and runtime errors
define_exception!(MEMORY, "MemoryError", exception_runtime_error);
define_exception!(FORK_ERROR, "ForkError", exception_runtime_error);

/// Memory error constant
pub fn memory_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.get_inner(&MEMORY), RACE_CONDITION_ERROR_MSG)
}

/// Create a `Wreq::InterruptError` outside the `StandardError` hierarchy.
pub fn interrupt_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.get_inner(&INTERRUPT_ERROR), "request interrupted")
}

/// Build `Wreq::ForkError` without touching inherited native state.
#[cfg(unix)]
pub fn fork_error(ruby: &Ruby, owner_pid: u32, current_pid: u32) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&FORK_ERROR),
        format!(
            "wreq-ruby was loaded in process {owner_pid} and cannot be used after fork in process {current_pid}"
        ),
    )
}

/// Map a failed process-fork handler registration to `Wreq::ForkError`.
#[cfg(unix)]
pub fn fork_handler_error(ruby: &Ruby, err: &std::io::Error) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&FORK_ERROR),
        format!("failed to initialize process fork tracking: {err}"),
    )
}

/// Map a Tokio runtime initialization failure to `Wreq::BuilderError`.
pub fn runtime_initialization_error(ruby: &Ruby, err: &std::io::Error) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("failed to initialize Tokio runtime: {err}"),
    )
}

/// LocalJumpError for methods that require a Ruby block.
pub fn no_block_given_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.exception_local_jump_error(), "no block given (yield)")
}

/// Build an `IOError` for writes to a closed request-body sender.
pub fn closed_body_sender_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.exception_io_error(), "closed body sender")
}

/// Map a failed body-channel send to `IOError`.
pub fn body_sender_send_error<T>(ruby: &Ruby, err: SendError<T>) -> MagnusError {
    MagnusError::new(
        ruby.exception_io_error(),
        format!("closed body sender: {err}"),
    )
}

/// Map an immutable sender-state borrow failure to `Wreq::BodyError`.
pub fn body_sender_borrow_error(ruby: &Ruby, err: BorrowError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BODY_ERROR),
        format!("body sender state is unavailable: {err}"),
    )
}

/// Map a mutable sender-state borrow failure to `Wreq::BodyError`.
pub fn body_sender_borrow_mut_error(ruby: &Ruby, err: BorrowMutError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BODY_ERROR),
        format!("body sender state is unavailable: {err}"),
    )
}

/// Map [`wreq::header::InvalidHeaderName`] to corresponding [`magnus::Error`]
pub fn header_name_error(ruby: &Ruby, err: wreq::header::InvalidHeaderName) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("invalid header name: {err}"),
    )
}

/// Map [`wreq::header::InvalidHeaderValue`] to corresponding [`magnus::Error`]
pub fn header_value_error(ruby: &Ruby, err: wreq::header::InvalidHeaderValue) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("invalid header value: {err}"),
    )
}

/// Build a `Wreq::BuilderError` for an invalid Ruby header structure.
pub fn header_type_error(ruby: &Ruby, err: &str) -> MagnusError {
    MagnusError::new(ruby.get_inner(&BUILDER_ERROR), format!("type error: {err}"))
}

/// Build a `Wreq::BuilderError` for unsupported request JSON values.
pub fn json_serialization_error(ruby: &Ruby, err: MagnusError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("JSON serialization error: {err}"),
    )
}

/// Prefix a Magnus error while preserving its Ruby exception class and cause.
pub fn contextualize_magnus_error(err: MagnusError, context: fmt::Arguments<'_>) -> MagnusError {
    match err.error_type() {
        ErrorType::Error(class, message) => {
            MagnusError::new(*class, format!("{context}: {message}"))
        }
        ErrorType::Exception(exception) => {
            let contextualized: Result<Exception, MagnusError> =
                exception.funcall("exception", (format!("{context}: {exception}"),));
            contextualized.map_or_else(|error| error, MagnusError::from)
        }
        ErrorType::Jump(_) => err,
    }
}

/// Add an option name while preserving the original Ruby exception class.
pub fn option_value_error(option: &str, err: MagnusError) -> MagnusError {
    contextualize_magnus_error(err, format_args!("invalid value for :{option}"))
}

/// Build an `ArgumentError` from a validation message.
pub fn argument_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_arg_error(), message)
}

/// Builds a Ruby `RangeError` from a validation message.
pub fn range_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_range_error(), message)
}

/// Build a `TypeError` from a conversion message.
pub fn type_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_type_error(), message)
}

/// Select the most specific Ruby exception class for native predicates.
fn wreq_error_class(ruby: &Ruby, predicates: ErrorPredicates) -> ExceptionClass {
    for &predicate in ErrorPredicate::CLASSIFICATION_ORDER {
        if predicates.contains(predicate) {
            return ruby.get_inner(predicate.error_class());
        }
    }

    ruby.get_inner(&WREQ_ERROR)
}

/// Read one native predicate from a Ruby error, defaulting to false.
fn error_has_predicate(rb_self: RObject, predicate: ErrorPredicate) -> Result<bool, MagnusError> {
    rb_self
        .ivar_get::<_, Option<u16>>(ERROR_PREDICATES_IVAR)
        .map(|bits| bits.is_some_and(|bits| ErrorPredicates::from_bits(bits).contains(predicate)))
}

/// Construct a Ruby exception and attach immutable native error metadata.
fn error_with_metadata(
    ruby: &Ruby,
    class: ExceptionClass,
    message: String,
    metadata: ErrorMetadata<'_>,
) -> MagnusError {
    match class.new_instance((message,)).and_then(|exception| {
        let object = RObject::try_convert(exception.as_value())?;
        object.ivar_set(ERROR_PREDICATES_IVAR, metadata.predicates.bits())?;

        if let Some(uri) = metadata.uri {
            let uri = ruby.str_new(uri);
            uri.freeze();
            object.ivar_set("@uri", uri)?;
        }

        if let Some(status) = metadata.status {
            object.ivar_set("@status", status.as_u16())?;
        }

        Ok(exception)
    }) {
        Ok(exception) => exception.into(),
        Err(error) => error,
    }
}

/// Map [`wreq::Error`] to corresponding [`magnus::Error`].
pub fn wreq_error(ruby: &Ruby, err: wreq::Error) -> MagnusError {
    let predicates = ErrorPredicates::from(&err);
    let class = wreq_error_class(ruby, predicates);
    let uri = err.uri().map(ToString::to_string);
    let status = err.status();
    let message = err.without_uri().to_string();

    error_with_metadata(
        ruby,
        class,
        message,
        ErrorMetadata {
            uri: uri.as_deref(),
            status,
            predicates,
        },
    )
}

/// Define and retain the Ruby exception classes used by the binding.
///
/// # Errors
///
/// Returns the Ruby exception raised while defining an error class.
pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), MagnusError> {
    let error_class = gem_module.define_error("Error", ruby.exception_runtime_error())?;
    error_class.define_attr("uri", Attr::Read)?;
    error_class.define_attr("status", Attr::Read)?;
    include_error_predicates(error_class)?;
    Lazy::force(&WREQ_ERROR, ruby);

    gem_module.define_error("InterruptError", ruby.exception_interrupt())?;
    Lazy::force(&INTERRUPT_ERROR, ruby);

    initialize_exception!(ruby, gem_module, MEMORY, "MemoryError", error_class);
    initialize_exception!(ruby, gem_module, FORK_ERROR, "ForkError", error_class);
    initialize_mapped_errors(ruby, gem_module, error_class)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ErrorPredicate, ErrorPredicates};

    #[test]
    fn error_predicate_bits_are_unique_and_round_trip() {
        let predicates = ErrorPredicate::CLASSIFICATION_ORDER.iter().copied().fold(
            ErrorPredicates::default(),
            |predicates, predicate| {
                assert!(!predicates.contains(predicate));
                predicates.include_if(predicate, true)
            },
        );

        assert_eq!(
            ErrorPredicate::CLASSIFICATION_ORDER.len(),
            predicates.bits().count_ones() as usize
        );

        let restored = ErrorPredicates::from_bits(predicates.bits());
        for &predicate in ErrorPredicate::CLASSIFICATION_ORDER {
            assert!(restored.contains(predicate));
        }
    }
}
