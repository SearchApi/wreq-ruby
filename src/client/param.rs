use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Represents HTTP parameters from Python as either a mapping or a sequence of key-value pairs.
pub type Params = IndexMap<String, ParamValue>;

/// Represents a single parameter value that can be automatically converted from Python types.
#[derive(Serialize, Deserialize)]
pub enum ParamValue {
    /// A boolean value from Python `bool`.
    Boolean(bool),
    /// An integer value from Python `int`.
    Number(isize),
    /// A floating-point value from Python `float`.
    Float64(f64),
    /// A string value from Python `str`.
    String(String),
}
