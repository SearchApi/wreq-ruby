use std::{
    cell::{BorrowError, BorrowMutError},
    fmt,
};

use bitflags::bitflags;
use magnus::{
    Attr, Class, Error as MagnusError, Exception, RModule, RObject, Ruby, TryConvert,
    error::ErrorType, exception::ExceptionClass, prelude::*, value::Lazy,
};
use tokio::sync::mpsc::error::SendError;

const ERROR_FLAGS_IVAR: &str = "wreq_error_flags";

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

macro_rules! define_error_predicates {
    ($($method:ident => $flag:ident = $value:expr),+ $(,)?) => {
        bitflags! {
            /// Native predicates retained after consuming a wreq error.
            struct ErrorFlags: u16 {
                $(const $flag = $value;)+
            }
        }

        $(
            fn $method(rb_self: RObject) -> Result<bool, MagnusError> {
                error_has_flag(rb_self, ErrorFlags::$flag)
            }
        )+

        /// Snapshot every native predicate before consuming the wreq error.
        fn wreq_error_flags(err: &wreq::Error) -> ErrorFlags {
            let mut flags = ErrorFlags::empty();
            $(
                if err.$method() {
                    flags.insert(ErrorFlags::$flag);
                }
            )+
            flags
        }

        /// Define the native wreq predicate methods on `Wreq::Error`.
        fn include_error_predicates(class: ExceptionClass) -> Result<(), MagnusError> {
            $(
                class.define_method(stringify!($method), magnus::method!($method, 0))?;
            )+
            Ok(())
        }
    };
}

define_error_predicates! {
    is_builder => IS_BUILDER = 1 << 0,
    is_redirect => IS_REDIRECT = 1 << 1,
    is_status => IS_STATUS = 1 << 2,
    is_timeout => IS_TIMEOUT = 1 << 3,
    is_request => IS_REQUEST = 1 << 4,
    is_connect => IS_CONNECT = 1 << 5,
    is_proxy_connect => IS_PROXY_CONNECT = 1 << 6,
    is_connection_reset => IS_CONNECTION_RESET = 1 << 7,
    is_body => IS_BODY = 1 << 8,
    is_tls => IS_TLS = 1 << 9,
    is_decode => IS_DECODE = 1 << 10,
    is_upgrade => IS_UPGRADE = 1 << 11,
}

/// Native error details retained after converting a wreq error to Ruby.
struct ErrorMetadata<'a> {
    uri: Option<&'a str>,
    status: Option<wreq::StatusCode>,
    flags: ErrorFlags,
}

// Stable roots for native errors.
define_exception!(WREQ_ERROR, "Error", exception_runtime_error);
define_exception!(INTERRUPT_ERROR, "InterruptError", exception_interrupt);

// System-level and runtime errors
define_exception!(MEMORY, "MemoryError", exception_runtime_error);
define_exception!(FORK_ERROR, "ForkError", exception_runtime_error);

// Network connection errors
define_exception!(CONNECTION_ERROR, "ConnectionError", exception_runtime_error);
define_exception!(
    PROXY_CONNECTION_ERROR,
    "ProxyConnectionError",
    exception_runtime_error
);
define_exception!(
    CONNECTION_RESET_ERROR,
    "ConnectionResetError",
    exception_runtime_error
);
define_exception!(TLS_ERROR, "TlsError", exception_runtime_error);

// HTTP protocol and request/response errors
define_exception!(REQUEST_ERROR, "RequestError", exception_runtime_error);
define_exception!(STATUS_ERROR, "StatusError", exception_runtime_error);
define_exception!(REDIRECT_ERROR, "RedirectError", exception_runtime_error);
define_exception!(TIMEOUT_ERROR, "TimeoutError", exception_runtime_error);

// Data processing and encoding errors
define_exception!(BODY_ERROR, "BodyError", exception_runtime_error);
define_exception!(DECODING_ERROR, "DecodingError", exception_runtime_error);

// Configuration and builder errors
define_exception!(BUILDER_ERROR, "BuilderError", exception_runtime_error);

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
            "wreq loaded in process {owner_pid} cannot be used after fork in process {current_pid}"
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
pub fn argument_error(ruby: &Ruby, message: impl Into<String>) -> MagnusError {
    MagnusError::new(ruby.exception_arg_error(), message.into())
}

/// Build a `RangeError` from a validation message.
pub fn range_error(ruby: &Ruby, message: impl Into<String>) -> MagnusError {
    MagnusError::new(ruby.exception_range_error(), message.into())
}

/// Build a `TypeError` from a conversion message.
pub fn type_error(ruby: &Ruby, message: impl Into<String>) -> MagnusError {
    MagnusError::new(ruby.exception_type_error(), message.into())
}

/// Select the most specific Ruby exception class for native predicate flags.
fn wreq_error_class(ruby: &Ruby, flags: &ErrorFlags) -> ExceptionClass {
    let class = if flags.contains(ErrorFlags::IS_BUILDER) {
        &BUILDER_ERROR
    } else if flags.contains(ErrorFlags::IS_BODY) {
        &BODY_ERROR
    } else if flags.contains(ErrorFlags::IS_TLS) {
        &TLS_ERROR
    } else if flags.contains(ErrorFlags::IS_CONNECTION_RESET) {
        &CONNECTION_RESET_ERROR
    } else if flags.contains(ErrorFlags::IS_CONNECT) {
        &CONNECTION_ERROR
    } else if flags.contains(ErrorFlags::IS_PROXY_CONNECT) {
        &PROXY_CONNECTION_ERROR
    } else if flags.contains(ErrorFlags::IS_DECODE) {
        &DECODING_ERROR
    } else if flags.contains(ErrorFlags::IS_REDIRECT) {
        &REDIRECT_ERROR
    } else if flags.contains(ErrorFlags::IS_TIMEOUT) {
        &TIMEOUT_ERROR
    } else if flags.contains(ErrorFlags::IS_STATUS) {
        &STATUS_ERROR
    } else if flags.contains(ErrorFlags::IS_REQUEST) {
        &REQUEST_ERROR
    } else {
        &WREQ_ERROR
    };
    ruby.get_inner(class)
}

/// Read one native predicate from a Ruby error, defaulting to `false`.
fn error_has_flag(rb_self: RObject, flag: ErrorFlags) -> Result<bool, MagnusError> {
    rb_self
        .ivar_get::<_, Option<u16>>(ERROR_FLAGS_IVAR)
        .map(|flags| flags.is_some_and(|flags| ErrorFlags::from_bits_retain(flags).contains(flag)))
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
        object.ivar_set(ERROR_FLAGS_IVAR, metadata.flags.bits())?;

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
    let flags = wreq_error_flags(&err);
    let class = wreq_error_class(ruby, &flags);
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
            flags,
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
    initialize_exception!(
        ruby,
        gem_module,
        CONNECTION_ERROR,
        "ConnectionError",
        error_class
    );
    initialize_exception!(
        ruby,
        gem_module,
        PROXY_CONNECTION_ERROR,
        "ProxyConnectionError",
        error_class
    );
    initialize_exception!(
        ruby,
        gem_module,
        CONNECTION_RESET_ERROR,
        "ConnectionResetError",
        error_class
    );
    initialize_exception!(ruby, gem_module, TLS_ERROR, "TlsError", error_class);
    initialize_exception!(ruby, gem_module, REQUEST_ERROR, "RequestError", error_class);

    initialize_exception!(ruby, gem_module, STATUS_ERROR, "StatusError", error_class);
    initialize_exception!(
        ruby,
        gem_module,
        REDIRECT_ERROR,
        "RedirectError",
        error_class
    );
    initialize_exception!(ruby, gem_module, TIMEOUT_ERROR, "TimeoutError", error_class);
    initialize_exception!(ruby, gem_module, BODY_ERROR, "BodyError", error_class);
    initialize_exception!(
        ruby,
        gem_module,
        DECODING_ERROR,
        "DecodingError",
        error_class
    );
    initialize_exception!(ruby, gem_module, BUILDER_ERROR, "BuilderError", error_class);
    Ok(())
}
