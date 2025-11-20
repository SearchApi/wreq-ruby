// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[macro_use]
mod macros;
mod client;
mod error;
mod extractor;
mod header;
mod http;
mod nogvl;

use std::sync::LazyLock;

use magnus::{Error, Ruby, Value};

use crate::{
    client::{Client, resp::Response},
    http::Method,
};

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime")
});

const RUBY_MODULE_NAME: &str = "Wreq";

/// Send a HTTP request.
#[inline]
pub fn request(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((method, url), request) = extract_args!(args, (&Method, String));
    Client::default().execute_request(*method, url, request)
}

/// Send a GET request.
#[inline]
pub fn get(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::GET, url, request)
}

/// Send a POST request.
#[inline]
pub fn post(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::POST, url, request)
}

/// Send a PUT request.
#[inline]
pub fn put(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::PUT, url, request)
}

/// Send a DELETE request.
#[inline]
pub fn delete(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::DELETE, url, request)
}

/// Send a HEAD request.
#[inline]
pub fn head(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::HEAD, url, request)
}

/// Send an OPTIONS request.
#[inline]
pub fn options(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::OPTIONS, url, request)
}

/// Send a TRACE request.
#[inline]
pub fn trace(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::TRACE, url, request)
}

/// Send a PATCH request.
#[inline]
pub fn patch(args: &[Value]) -> Result<Response, magnus::Error> {
    let ((url,), request) = extract_args!(args, (String,));
    Client::default().execute_request(Method::PATCH, url, request)
}

/// wreq ruby binding
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let gem_module = ruby.define_module(RUBY_MODULE_NAME)?;
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
    client::include(ruby, &gem_module)?;
    error::include(ruby);
    Ok(())
}
