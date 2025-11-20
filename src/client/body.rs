mod stream;

pub use self::stream::{Streamer, include};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Represents a JSON value for HTTP requests.
/// Supports objects, arrays, numbers, strings, booleans, and null.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Json {
    Object(IndexMap<String, Json>),
    Boolean(bool),
    Number(isize),
    Float(f64),
    String(String),
    Null(Option<isize>),
    Array(Vec<Json>),
}

/// Represents the body of an HTTP request.
/// Supports text, bytes, form, json, synchronous and asynchronous streaming bodies.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Body {
    Text(String),
    Bytes(Vec<u8>),
}
