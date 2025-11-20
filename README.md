# wreq-ruby

## Documentation

More detailed documentation is a work in progress. Meanwhile, you can refer to the [wreq](https://docs.rs/wreq/latest/wreq/) documentation for details on the underlying Rust library.

## Development

Install the BoringSSL build environment by referring to [boring](https://github.com/cloudflare/boring/blob/master/.github/workflows/ci.yml) and [boringssl](https://github.com/google/boringssl/blob/master/BUILDING.md#build-prerequisites).

```bash
# Prerequisites: Install build dependencies
# on ubuntu or debian
sudo apt install -y build-essential cmake perl pkg-config libclang-dev musl-tools git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
bundle install

# Option 1: Build source gem (requires compilation during install)
# This approach requires users to have Rust toolchain and build environment.
# Installation will show "Building native extensions. This could take a while..."
gem build wreq.gemspec
gem install wreq-*.gem

# Option 2: Build pre-compiled platform gem (recommended for distribution)
# This creates a platform-specific gem (e.g., wreq-0.1.0-arm64-darwin.gem) 
# with pre-compiled binaries. Users can install quickly without build environment.
bundle exec rake compile
bundle exec rake native gem
gem install pkg/wreq-*-$(ruby -e 'puts RUBY_PLATFORM').gem

# Development workflow
bundle exec rake compile    # Compile for development/testing
bundle exec rake test       # Run tests  
bundle exec ruby examples/body.rb  # Run examples without installing
```

## License

Licensed under either of Apache License, Version 2.0 ([LICENSE](./LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the [Apache-2.0](./LICENSE) license, shall be licensed as above, without any additional terms or conditions.
