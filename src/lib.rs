#![allow(clippy::wrong_self_convention)]

#[macro_use]
mod macros;
mod client;
mod cookie;
mod emulation;
mod error;
mod extractor;
mod gvl;
mod header;
mod http;
mod rt;

use magnus::{Error, Module, Ruby, Value};

use crate::client::{Client, resp::Response};

const RUBY_MODULE_NAME: &str = "Wreq";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Send a HTTP request.
#[inline]
pub fn request(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::request(&Client::default(), args)
}

/// Send a GET request.
#[inline]
pub fn get(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::get(&Client::default(), args)
}

/// Send a POST request.
#[inline]
pub fn post(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::post(&Client::default(), args)
}

/// Send a PUT request.
#[inline]
pub fn put(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::put(&Client::default(), args)
}

/// Send a DELETE request.
#[inline]
pub fn delete(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::delete(&Client::default(), args)
}

/// Send a HEAD request.
#[inline]
pub fn head(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::head(&Client::default(), args)
}

/// Send an OPTIONS request.
#[inline]
pub fn options(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::options(&Client::default(), args)
}

/// Send a TRACE request.
#[inline]
pub fn trace(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::trace(&Client::default(), args)
}

/// Send a PATCH request.
#[inline]
pub fn patch(args: &[Value]) -> Result<Response, magnus::Error> {
    Client::patch(&Client::default(), args)
}

/// wreq ruby binding
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let gem_module = ruby.define_module(RUBY_MODULE_NAME)?;
    gem_module.const_set("VERSION", VERSION)?;
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
    client::include(ruby, &gem_module)?;
    emulation::include(ruby, &gem_module)?;
    error::include(ruby);
    Ok(())
}
