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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sets the correct linker flags for the Ruby C API, which makes it possible
    // to run Cargo commands without requiring `rb_sys/mkmf`.
    //
    // This is not a requirement, but it is a convenient if you want to use
    // `cargo test`, etc.
    let _ = rb_sys_env::activate()?;
    Ok(())
}
