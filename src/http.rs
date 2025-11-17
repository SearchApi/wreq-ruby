use magnus::{Error, Module, RModule, Ruby};

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
    Ok(())
}
