use magnus::{
    Error as MagnusError, RModule, Ruby, exception::ExceptionClass, prelude::*, value::Lazy,
};

static WREQ: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module(crate::RUBY_MODULE_NAME).unwrap());

macro_rules! define_exception {
    ($name:ident, $ruby_name:literal, $parent:expr) => {
        static $name: Lazy<ExceptionClass> = Lazy::new(|ruby| {
            ruby.get_inner(&WREQ)
                .define_error($ruby_name, $parent(ruby))
                .unwrap()
        });
    };
}

// Network connection errors
define_exception!(
    CONNECTION_ERROR,
    "ConnectionError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    CONNECTION_RESET_ERROR,
    "ConnectionResetError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    TLS_ERROR,
    "TlsError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);

// HTTP protocol and request/response errors
define_exception!(
    REQUEST_ERROR,
    "RequestError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    STATUS_ERROR,
    "StatusError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    REDIRECT_ERROR,
    "RedirectError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    TIMEOUT_ERROR,
    "TimeoutError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);

// Data processing and encoding errors
define_exception!(
    BODY_ERROR,
    "BodyError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);
define_exception!(
    DECODING_ERROR,
    "DecodingError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);

// Configuration and builder errors
define_exception!(
    BUILDER_ERROR,
    "BuilderError",
    |ruby: &Ruby| ruby.exception_runtime_error()
);

pub fn wreq_error_to_magnus(ruby: &Ruby, err: wreq::Error) -> MagnusError {
    let error_msg = err.to_string();

    if err.is_builder() {
        MagnusError::new(ruby.get_inner(&BUILDER_ERROR), error_msg)
    } else if err.is_body() {
        MagnusError::new(ruby.get_inner(&BODY_ERROR), error_msg)
    } else if err.is_tls() {
        MagnusError::new(ruby.get_inner(&TLS_ERROR), error_msg)
    } else if err.is_connection_reset() {
        MagnusError::new(ruby.get_inner(&CONNECTION_RESET_ERROR), error_msg)
    } else if err.is_connect() {
        MagnusError::new(ruby.get_inner(&CONNECTION_ERROR), error_msg)
    } else if err.is_decode() {
        MagnusError::new(ruby.get_inner(&DECODING_ERROR), error_msg)
    } else if err.is_redirect() {
        MagnusError::new(ruby.get_inner(&REDIRECT_ERROR), error_msg)
    } else if err.is_timeout() {
        MagnusError::new(ruby.get_inner(&TIMEOUT_ERROR), error_msg)
    } else if err.is_status() {
        MagnusError::new(ruby.get_inner(&STATUS_ERROR), error_msg)
    } else if err.is_request() {
        MagnusError::new(ruby.get_inner(&REQUEST_ERROR), error_msg)
    } else {
        MagnusError::new(ruby.exception_runtime_error(), error_msg)
    }
}

pub fn include(ruby: &Ruby) {
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
