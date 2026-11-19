# `serde_magnus`

`serde_magnus` converts between Rust and Ruby data structures using [Serde] and [Magnus].

[Serde]: https://github.com/serde-rs/serde
[Magnus]: https://github.com/matsadler/magnus

## Local maintenance

This crate is based on [OneSignal's serde_magnus 0.11.0 source][upstream]. It
uses the same Magnus 0.9 series and Rust 1.98 minimum as `wreq-ruby`.

### Local patches

- `src/de/` and `src/ser/` preserve arbitrary-size JSON integers in both
  directions and support typed `i128` and `u128`. JSON input validates object keys,
  finite floats, and container depth. The precision tests also cover the
  private number token shared with `serde_json`.
- `src/options.rs` validates option names before converting values and attaches
  the failing field path to Ruby exceptions.
- `src/error.rs` carries Serde errors until the conversion entry point can use
  its Ruby handle. The deserializer and map serializer check iteration and map
  state.

To sync, diff this directory against the pinned source tree, reapply the local
changes to the new upstream commit, and update the pin below. Run
`cargo test --locked -p serde_magnus` and `bundle exec rake test` from the
repository root. The Ruby tests in `test/json_precision_test.rb` and
`test/option_validation_test.rb` exercise the request, response, and option
entry points.

[upstream]: https://github.com/OneSignal/serde-magnus/tree/b84e35d03af9f84edf32e29c1fc022b4411ba9b9

## Upstream documentation

* [API documentation](https://docs.rs/serde_magnus)
* [Releases/changelog](https://github.com/OneSignal/serde-magnus/releases)
* [`serde_magnus` on crates.io](https://crates.io/crates/serde_magnus)

## Usage

The [`serde_magnus::serialize`] function converts from a Rust type implementing the
[`serde::Serialize`] trait into a Ruby equivalent.

```rust
use serde::{Serialize, Deserialize};
use magnus::{eval, Ruby, Value};
use serde_magnus::serialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Post {
    title: String,
    content: String,
    author: Author,
    tags: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Author {
    name: String,
    email_address: String
}

let post = Post {
    title: "Spring carnival planning update".into(),
    content: "Here's what's new.".into(),
    author: Author {
        name: "Martha".into(),
        email_address: "martha@example.com".into()
    },
    tags: vec![
        "carnival".into(),
        "update".into()
    ]
};

let ruby = Ruby::get().unwrap();
let post: Value = serialize(&ruby, &post)?;

assert!(eval!(
    &ruby,
    r#"
    post == {
      title: "Spring carnival planning update",
      content: "Here's what's new.",
      author: {
        name: "Martha",
        email_address: "martha@example.com"
      },
      tags: ["carnival", "update"]
    }
    "#,
    post
)?);
```

[`serde_magnus::deserialize`] converts from a Ruby value to a Rust type implementing
[`serde::Deserialize`].

```rust
use magnus::{RHash, Ruby};
use serde_magnus::deserialize;

let ruby = Ruby::get().unwrap();
let post: RHash = eval!(&ruby, r#"
  {
    title: "Spring carnival planning update",
    content: "Here's what's new.",
    author: {
      name: "Martha",
      email_address: "martha@example.com"
    },
    tags: ["carnival", "update"]
  }
"#)?;

let post: Post = deserialize(&ruby, post)?;

assert_eq!(
    Post {
        title: "Spring carnival planning update".into(),
        content: "Here's what's new.".into(),
        author: Author {
            name: "Martha".into(),
            email_address: "martha@example.com".into(),
        },
        tags: vec![
            "carnival".into(),
            "update".into()
        ]
    },
    post
);
```

[`serde_magnus::serialize`]: https://docs.rs/serde_magnus/latest/serde_magnus/fn.serialize.html
[`serde::Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`serde_magnus::deserialize`]: https://docs.rs/serde_magnus/latest/serde_magnus/fn.deserialize.html
[`serde::Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html

## Requirements

The local crate shares the Rust and Ruby requirements of `wreq-ruby`.

## License

`serde_magnus` is released under the terms of the MIT License. See `LICENSE` for details.
