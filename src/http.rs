use magnus::{Error, Module, RModule, Ruby, TryConvert, Value, method, typed_data::Inspect};

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

impl Method {
    /// HTTP method token as a string.
    #[inline]
    pub fn to_s(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }

    /// HTTP method as a lowercase Ruby symbol.
    #[inline]
    pub fn to_sym(ruby: &Ruby, rb_self: &Self) -> magnus::Symbol {
        let name = match *rb_self {
            Method::GET => "get",
            Method::HEAD => "head",
            Method::POST => "post",
            Method::PUT => "put",
            Method::DELETE => "delete",
            Method::OPTIONS => "options",
            Method::TRACE => "trace",
            Method::PATCH => "patch",
        };
        ruby.to_symbol(name)
    }
}

/// HTTP status code.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[magnus::wrap(class = "Wreq::StatusCode", free_immediately, size)]
pub struct StatusCode(pub wreq::StatusCode);

// ===== impl Version =====

impl Version {
    /// Convert version to string.
    #[inline]
    pub fn to_s(&self) -> String {
        self.into_ffi().inspect()
    }
}

impl TryConvert for Version {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        <&Version>::try_convert(value).cloned()
    }
}

// ===== impl StatusCode =====

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

    /// Convert status code to string.
    #[inline]
    pub fn to_s(&self) -> String {
        self.0.to_string()
    }

    /// Value-based equality for Ruby (`==`).
    #[inline]
    pub fn equals(&self, other: Value) -> bool {
        <&StatusCode>::try_convert(other)
            .map(|other| *self == *other)
            .unwrap_or(false)
    }

    /// Strict equality for Ruby (`eql?`).
    #[inline]
    pub fn is_eql(&self, other: Value) -> bool {
        self.equals(other)
    }

    /// Hash value for Ruby (`hash`).
    #[inline]
    pub fn hash_value(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    /// Return the status code as an integer (Ruby `to_i`).
    #[inline]
    pub const fn to_i(&self) -> u16 {
        self.0.as_u16()
    }
}

impl From<wreq::StatusCode> for StatusCode {
    fn from(status: wreq::StatusCode) -> Self {
        Self(status)
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let method_class = gem_module.define_class("Method", ruby.class_object())?;
    Method::define_constants(method_class)?;
    method_class.define_method("to_s", method!(Method::to_s, 0))?;
    method_class.define_method("to_sym", method!(Method::to_sym, 0))?;
    method_class.define_method("==", method!(Method::equals, 1))?;
    method_class.define_method("eql?", method!(Method::is_eql, 1))?;
    method_class.define_method("hash", method!(Method::hash_value, 0))?;

    let version_class = gem_module.define_class("Version", ruby.class_object())?;
    Version::define_constants(version_class)?;
    version_class.define_method("to_s", method!(Version::to_s, 0))?;
    version_class.define_method("==", method!(Version::equals, 1))?;
    version_class.define_method("eql?", method!(Version::is_eql, 1))?;
    version_class.define_method("hash", method!(Version::hash_value, 0))?;

    let status_code_class = gem_module.define_class("StatusCode", ruby.class_object())?;
    status_code_class.define_method("as_int", method!(StatusCode::as_int, 0))?;
    status_code_class.define_method("informational?", method!(StatusCode::is_informational, 0))?;
    status_code_class.define_method("success?", method!(StatusCode::is_success, 0))?;
    status_code_class.define_method("redirection?", method!(StatusCode::is_redirection, 0))?;
    status_code_class.define_method("client_error?", method!(StatusCode::is_client_error, 0))?;
    status_code_class.define_method("server_error?", method!(StatusCode::is_server_error, 0))?;
    status_code_class.define_method("to_s", method!(StatusCode::to_s, 0))?;
    status_code_class.define_method("==", method!(StatusCode::equals, 1))?;
    status_code_class.define_method("eql?", method!(StatusCode::is_eql, 1))?;
    status_code_class.define_method("hash", method!(StatusCode::hash_value, 0))?;
    status_code_class.define_method("to_i", method!(StatusCode::to_i, 0))?;

    Ok(())
}
