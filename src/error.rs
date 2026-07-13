use std::cell::{BorrowError, BorrowMutError};

use magnus::{
    Error as MagnusError, RModule, Ruby, error::ErrorType, exception::ExceptionClass, prelude::*,
    value::Lazy,
};
use tokio::sync::mpsc::error::SendError;

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
    ($ruby:expr, $module:expr, $name:ident, $ruby_name:literal, $parent_method:ident) => {{
        $module.define_error($ruby_name, $ruby.$parent_method())?;
        Lazy::force(&$name, $ruby);
    }};
}

macro_rules! map_wreq_error {
    ($ruby:expr, $err:expr, $msg:expr, $($check_method:ident => $exception:ident),* $(,)?) => {
        {
            $(
                if $err.$check_method() {
                    return MagnusError::new($ruby.get_inner(&$exception), $msg);
                }
            )*
            MagnusError::new($ruby.exception_runtime_error(), $msg)
        }
    };
}

// System-level and runtime errors
define_exception!(MEMORY, "MemoryError", exception_runtime_error);

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

/// Create Ruby's standard thread interruption error.
pub fn interrupt_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.exception_interrupt(), "request interrupted")
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

/// Add an option name while preserving common Ruby configuration error classes.
pub fn option_value_error(ruby: &Ruby, option: &str, err: MagnusError) -> MagnusError {
    let class = if err.is_kind_of(ruby.exception_type_error()) {
        ruby.exception_type_error()
    } else if err.is_kind_of(ruby.exception_arg_error()) {
        ruby.exception_arg_error()
    } else if err.is_kind_of(ruby.get_inner(&BUILDER_ERROR)) {
        ruby.get_inner(&BUILDER_ERROR)
    } else {
        return err;
    };
    let message = match err.error_type() {
        ErrorType::Error(_, message) => message.as_ref().to_owned(),
        _ => err.to_string(),
    };
    MagnusError::new(class, format!("invalid value for :{option}: {message}"))
}

/// Build an `ArgumentError` from a validation message.
pub fn argument_error(ruby: &Ruby, message: impl Into<String>) -> MagnusError {
    MagnusError::new(ruby.exception_arg_error(), message.into())
}

/// Build a `TypeError` from a conversion message.
pub fn type_error(ruby: &Ruby, message: impl Into<String>) -> MagnusError {
    MagnusError::new(ruby.exception_type_error(), message.into())
}
/// Map [`wreq::Error`] to corresponding [`magnus::Error`]
pub fn wreq_error(ruby: &Ruby, err: wreq::Error) -> MagnusError {
    let error_msg = err.to_string();
    map_wreq_error!(
        ruby,
        err,
        error_msg,
        is_builder => BUILDER_ERROR,
        is_body => BODY_ERROR,
        is_tls => TLS_ERROR,
        is_connection_reset => CONNECTION_RESET_ERROR,
        is_connect => CONNECTION_ERROR,
        is_proxy_connect => PROXY_CONNECTION_ERROR,
        is_decode => DECODING_ERROR,
        is_redirect => REDIRECT_ERROR,
        is_timeout => TIMEOUT_ERROR,
        is_status => STATUS_ERROR,
        is_request => REQUEST_ERROR,
    )
}

/// Define and retain the Ruby exception classes used by the binding.
///
/// # Errors
///
/// Returns the Ruby exception raised while defining an error class.
pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), MagnusError> {
    initialize_exception!(
        ruby,
        gem_module,
        MEMORY,
        "MemoryError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        CONNECTION_ERROR,
        "ConnectionError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        PROXY_CONNECTION_ERROR,
        "ProxyConnectionError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        CONNECTION_RESET_ERROR,
        "ConnectionResetError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        TLS_ERROR,
        "TlsError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        REQUEST_ERROR,
        "RequestError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        STATUS_ERROR,
        "StatusError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        REDIRECT_ERROR,
        "RedirectError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        TIMEOUT_ERROR,
        "TimeoutError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        BODY_ERROR,
        "BodyError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        DECODING_ERROR,
        "DecodingError",
        exception_runtime_error
    );
    initialize_exception!(
        ruby,
        gem_module,
        BUILDER_ERROR,
        "BuilderError",
        exception_runtime_error
    );
    Ok(())
}
