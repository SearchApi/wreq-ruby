# Local Rust dependencies

- [`wreq-util`](wreq-util/) is a local copy of [0x676e67/wreq-util](https://github.com/0x676e67/wreq-util). The root [`Cargo.toml`](../Cargo.toml) resolves it from `crates/wreq-util` through `[patch.crates-io]`.

When building or testing `wreq-util` on its own, use the Git dependency patches from the root manifest of the same branch. Cargo does not inherit those patches from the parent package.
