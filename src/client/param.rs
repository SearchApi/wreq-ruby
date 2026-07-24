use ::serde::{Deserialize, Serialize};
use indexmap::IndexMap;

/// HTTP parameters represented as an insertion-ordered Ruby mapping.
pub type Params = IndexMap<String, ParamValue>;

/// A scalar Ruby value accepted in query-string and form mappings.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// A Ruby `true` or `false` value.
    Boolean(bool),
    /// A Ruby Integer that fits in the native pointer-sized range.
    Number(isize),
    /// A Ruby Float.
    Float64(f64),
    /// A Ruby String or Symbol.
    String(String),
}
