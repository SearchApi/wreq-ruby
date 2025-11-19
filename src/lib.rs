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

#![allow(unused)]

#[macro_use]
mod macros;
mod client;
mod error;
mod extractor;
mod http;
mod nogvl;

use std::sync::LazyLock;

use magnus::{Error, RModule, Ruby, value::Lazy};

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime")
});

const RUBY_MODULE_NAME: &str = "Wreq";

/// wreq ruby binding
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let gem_module = ruby.define_module(RUBY_MODULE_NAME)?;
    http::include(ruby, &gem_module)?;
    client::include(ruby, &gem_module)?;
    error::include(ruby);
    Ok(())
}
