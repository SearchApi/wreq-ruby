use magnus::{Error, Module, RModule, Ruby, method};

define_ruby_enum!(
    /// An HTTP version.
    const,
    Version,
    "Wreq::Version",
    wreq::Version,
    HTTP_09,
    HTTP_10,
    HTTP_11,
    HTTP_2,
    HTTP_3,
);

define_ruby_enum!(
    /// An HTTP method.
    Method,
    "Wreq::Method",
    wreq::Method,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    PATCH,
);

/// HTTP status code.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[magnus::wrap(class = "Wreq::StatusCode", free_immediately, size)]
pub struct StatusCode(pub wreq::StatusCode);

impl StatusCode {
    /// Return the status code as an integer.
    #[inline]
    pub const fn as_int(&self) -> u16 {
        self.0.as_u16()
    }

    /// Check if status is within 100-199.
    #[inline]
    pub fn is_informational(&self) -> bool {
        self.0.is_informational()
    }

    /// Check if status is within 200-299.
    #[inline]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    /// Check if status is within 300-399.
    #[inline]
    pub fn is_redirection(&self) -> bool {
        self.0.is_redirection()
    }

    /// Check if status is within 400-499.
    #[inline]
    pub fn is_client_error(&self) -> bool {
        self.0.is_client_error()
    }

    /// Check if status is within 500-599.
    #[inline]
    pub fn is_server_error(&self) -> bool {
        self.0.is_server_error()
    }
}

impl From<wreq::StatusCode> for StatusCode {
    fn from(status: wreq::StatusCode) -> Self {
        Self(status)
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let method_class = gem_module.define_class("Method", ruby.class_object())?;
    method_class.const_set("GET", Method::GET)?;
    method_class.const_set("POST", Method::POST)?;
    method_class.const_set("PUT", Method::PUT)?;
    method_class.const_set("DELETE", Method::DELETE)?;
    method_class.const_set("PATCH", Method::PATCH)?;
    method_class.const_set("HEAD", Method::HEAD)?;
    method_class.const_set("TRACE", Method::TRACE)?;
    method_class.const_set("OPTIONS", Method::OPTIONS)?;

    let version_class = gem_module.define_class("Version", ruby.class_object())?;
    version_class.const_set("HTTP_09", Version::HTTP_09)?;
    version_class.const_set("HTTP_10", Version::HTTP_10)?;
    version_class.const_set("HTTP_11", Version::HTTP_11)?;
    version_class.const_set("HTTP_2", Version::HTTP_2)?;
    version_class.const_set("HTTP_3", Version::HTTP_3)?;

    let status_code_class = gem_module.define_class("StatusCode", ruby.class_object())?;
    status_code_class.define_method("as_int", method!(StatusCode::as_int, 0))?;
    status_code_class.define_method("informational?", method!(StatusCode::is_informational, 0))?;
    status_code_class.define_method("success?", method!(StatusCode::is_success, 0))?;
    status_code_class.define_method("redirection?", method!(StatusCode::is_redirection, 0))?;
    status_code_class.define_method("client_error?", method!(StatusCode::is_client_error, 0))?;
    status_code_class.define_method("server_error?", method!(StatusCode::is_server_error, 0))?;

    Ok(())
}
