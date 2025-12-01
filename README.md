# wreq-ruby

 An easy and powerful Ruby HTTP client with advanced browser fingerprinting that accurately emulates **Chrome**, **Firefox**, **Safari**, **Opera**, and **OkHttp**, with precise **TLS/HTTP2** signatures, and powered by [wreq](https://github.com/0x676e67/wreq).


## Features

- Plain bodies, JSON, urlencoded,
- Cookie Store
- Redirect Policy
- Original Headers
- Rotating Proxies
- Connection Pooling
- Streaming Transfers
- HTTPS via BoringSSL
- Free-Threaded Safety
- Automatic Decompression

## Example

This example demonstrates how to make a simple GET request using the `wreq` library. So you need install `wreq` and run the following code:

```bash
gem install wreq
```

And then the code:

```ruby
require "wreq"

 # Build a client
client = Wreq::Client.new(emulation: Wreq::Emulation.new(
  device: Wreq::EmulationDevice::Chrome142,
  os: Wreq::EmulationOS::MacOS,
  skip_http2: false,
  skip_headers: false
))

# Use the API you're already familiar with
resp = client.get("https://tls.peet.ws/api/all")
puts resp.text
```

Additional learning resources include:

- [API Documentation](https://github.com/SearchApi/wreq-ruby/tree/main/lib)
- [Repository Tests](https://github.com/SearchApi/wreq-ruby/tree/main/test)
- [Repository Examples](https://github.com/SearchApi/wreq-ruby/tree/main/examples)

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
gem install pkg/wreq-*.gem

# Development workflow
bundle exec rake compile    # Compile for development/testing
bundle exec rake test       # Run tests  
bundle exec ruby examples/body.rb  # Run examples without installing
```

## License

Licensed under either of Apache License, Version 2.0 ([LICENSE](./LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the [Apache-2.0](./LICENSE) license, shall be licensed as above, without any additional terms or conditions.
