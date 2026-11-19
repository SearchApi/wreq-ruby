#![deny(unsafe_code)]
#![allow(clippy::wrong_self_convention)]

#[macro_use]
mod macros;
mod arch;
mod client;
mod cookie;
mod emulate;
mod error;
mod extractor;
mod gvl;
mod header;
mod http;
mod options;
mod rt;
mod serde;
mod tls;

use magnus::{Error, Module, Ruby, Value};

use crate::{
    client::{Client, response::Response},
    http::Method,
};

const RUBY_MODULE_NAME: &str = "Wreq";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Send a HTTP request.
#[inline]
pub fn request(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once_from_args(ruby, args)
}

/// Send a GET request.
#[inline]
pub fn get(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::GET, args)
}

/// Send a POST request.
#[inline]
pub fn post(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::POST, args)
}

/// Send a PUT request.
#[inline]
pub fn put(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::PUT, args)
}

/// Send a DELETE request.
#[inline]
pub fn delete(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::DELETE, args)
}

/// Send a HEAD request.
#[inline]
pub fn head(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::HEAD, args)
}

/// Send an OPTIONS request.
#[inline]
pub fn options(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::OPTIONS, args)
}

/// Send a TRACE request.
#[inline]
pub fn trace(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::TRACE, args)
}

/// Send a PATCH request.
#[inline]
pub fn patch(ruby: &Ruby, args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request_once(ruby, Method::PATCH, args)
}

/// wreq ruby binding
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let gem_module = ruby.define_module(RUBY_MODULE_NAME)?;
    gem_module.const_set("VERSION", VERSION)?;
    error::include(ruby, &gem_module)?;
    gem_module.define_module_function("request", magnus::function!(request, -1))?;
    gem_module.define_module_function("get", magnus::function!(get, -1))?;
    gem_module.define_module_function("post", magnus::function!(post, -1))?;
    gem_module.define_module_function("put", magnus::function!(put, -1))?;
    gem_module.define_module_function("delete", magnus::function!(delete, -1))?;
    gem_module.define_module_function("head", magnus::function!(head, -1))?;
    gem_module.define_module_function("options", magnus::function!(options, -1))?;
    gem_module.define_module_function("trace", magnus::function!(trace, -1))?;
    gem_module.define_module_function("patch", magnus::function!(patch, -1))?;
    http::include(ruby, &gem_module)?;
    header::include(ruby, &gem_module)?;
    cookie::include(ruby, &gem_module)?;
    tls::include(ruby, &gem_module)?;
    client::include(ruby, &gem_module)?;
    emulate::include(ruby, &gem_module)?;
    Ok(())
}
