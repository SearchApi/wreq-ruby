#![deny(unsafe_code)]
#![doc(test(attr(warn(unused))))]

//! `serde_magnus` converts between Rust and Ruby data structures using [Serde] and [Magnus].
//!
//! [Serde]: https://github.com/serde-rs/serde
//! [Magnus]: https://github.com/matsadler/magnus
//!
//! The [`serialize`] function converts from a Rust type implementing the [`serde::Serialize`]
//! trait into a Ruby equivalent.
//!
//! ```
//! # // SAFETY: this doctest initializes Ruby once on its current thread.
//! # let ruby = unsafe { magnus::embed::init() };
//! #
//! use serde::{Serialize, Deserialize};
//! use magnus::{eval, Value};
//! use serde_magnus::serialize;
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Post {
//!     title: String,
//!     content: String,
//!     author: Author,
//!     tags: Vec<String>
//! }
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Author {
//!     name: String,
//!     email_address: String
//! }
//!
//! let post = Post {
//!     title: "Spring carnival planning update".into(),
//!     content: "Here's what's new.".into(),
//!     author: Author {
//!         name: "Martha".into(),
//!         email_address: "martha@example.com".into()
//!     },
//!     tags: vec![
//!         "carnival".into(),
//!         "update".into()
//!     ]
//! };
//!
//! let post: Value = serialize(&ruby, &post)?;
//!
//! assert!(eval!(
//!     &ruby,
//!     r#"
//!     post == {
//!       title: "Spring carnival planning update",
//!       content: "Here's what's new.",
//!       author: {
//!         name: "Martha",
//!         email_address: "martha@example.com"
//!       },
//!       tags: ["carnival", "update"]
//!     }
//!     "#,
//!     post
//! )?);
//! #
//! # Ok::<(), magnus::Error>(())
//! ```
//!
//! [`deserialize`] converts from a Ruby value to a Rust type implementing [`serde::Deserialize`].
//!
//! ```
//! # use serde::Deserialize;
//! # use magnus::{eval, RHash};
//! #
//! # // SAFETY: this doctest initializes Ruby once on its current thread.
//! # let ruby = unsafe { magnus::embed::init() };
//! #
//! # #[derive(Deserialize, PartialEq, Debug)]
//! # struct Post {
//! #     title: String,
//! #     content: String,
//! #     author: Author,
//! #     tags: Vec<String>
//! # }
//! #
//! # #[derive(Deserialize, PartialEq, Debug)]
//! # struct Author {
//! #     name: String,
//! #     email_address: String
//! # }
//! #
//! use serde_magnus::deserialize;
//!
//! let post: RHash = eval!(&ruby, r#"
//!   {
//!     title: "Spring carnival planning update",
//!     content: "Here's what's new.",
//!     author: {
//!       name: "Martha",
//!       email_address: "martha@example.com"
//!     },
//!     tags: ["carnival", "update"]
//!   }
//! "#)?;
//!
//! let post: Post = deserialize(&ruby, post)?;
//!
//! assert_eq!(
//!     Post {
//!         title: "Spring carnival planning update".into(),
//!         content: "Here's what's new.".into(),
//!         author: Author {
//!             name: "Martha".into(),
//!             email_address: "martha@example.com".into()
//!         },
//!         tags: vec![
//!             "carnival".into(),
//!             "update".into()
//!         ]
//!     },
//!     post
//! );
//! #
//! # Ok::<(), magnus::Error>(())
//! ```

mod de;
mod error;
pub mod options;
mod ser;
#[cfg(test)]
mod tests;

pub use de::{deserialize, deserialize_json};
pub use ser::serialize;

/// Private map key used by `serde_json` for arbitrary-precision numbers.
const JSON_NUMBER_TOKEN: &str = "$serde_json::private::Number";
