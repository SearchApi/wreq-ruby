use magnus::{
    Error as MagnusError, RModule, Ruby, exception::ExceptionClass, prelude::*, value::Lazy,
};

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

static WREQ: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module(crate::RUBY_MODULE_NAME).unwrap());

macro_rules! define_exception {
    ($name:ident, $ruby_name:literal, $parent_method:ident) => {
        static $name: Lazy<ExceptionClass> = Lazy::new(|ruby| {
            ruby.get_inner(&WREQ)
                .define_error($ruby_name, ruby.$parent_method())
                .unwrap()
        });
    };
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
pub fn memory_error() -> MagnusError {
    MagnusError::new(ruby!().get_inner(&MEMORY), RACE_CONDITION_ERROR_MSG)
}

/// Map [`wreq::header::InvalidHeaderName`] to corresponding [`magnus::Error`]
pub fn header_name_error_to_magnus(err: wreq::header::InvalidHeaderName) -> MagnusError {
    MagnusError::new(
        ruby!().get_inner(&BUILDER_ERROR),
        format!("invalid header name: {err}"),
    )
}

/// Map [`wreq::header::InvalidHeaderValue`] to corresponding [`magnus::Error`]
pub fn header_value_error_to_magnus(err: wreq::header::InvalidHeaderValue) -> MagnusError {
    MagnusError::new(
        ruby!().get_inner(&BUILDER_ERROR),
        format!("invalid header value: {err}"),
    )
}

/// Map [`wreq::Error`] to corresponding [`magnus::Error`]
pub fn wreq_error_to_magnus(err: wreq::Error) -> MagnusError {
    let error_msg = err.to_string();
    map_wreq_error!(
        ruby!(),
        err,
        error_msg,
        is_builder => BUILDER_ERROR,
        is_body => BODY_ERROR,
        is_tls => TLS_ERROR,
        is_connection_reset => CONNECTION_RESET_ERROR,
        is_connect => CONNECTION_ERROR,
        is_decode => DECODING_ERROR,
        is_redirect => REDIRECT_ERROR,
        is_timeout => TIMEOUT_ERROR,
        is_status => STATUS_ERROR,
        is_request => REQUEST_ERROR,
    )
}

pub fn include(ruby: &Ruby) {
    Lazy::force(&MEMORY, ruby);
    Lazy::force(&CONNECTION_ERROR, ruby);
    Lazy::force(&CONNECTION_RESET_ERROR, ruby);
    Lazy::force(&TLS_ERROR, ruby);
    Lazy::force(&REQUEST_ERROR, ruby);
    Lazy::force(&STATUS_ERROR, ruby);
    Lazy::force(&REDIRECT_ERROR, ruby);
    Lazy::force(&TIMEOUT_ERROR, ruby);
    Lazy::force(&BODY_ERROR, ruby);
    Lazy::force(&DECODING_ERROR, ruby);
    Lazy::force(&BUILDER_ERROR, ruby);
}
