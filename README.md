# wreq-ruby

[![CI](https://github.com/SearchApi/wreq-ruby/actions/workflows/ci.yml/badge.svg)](https://github.com/SearchApi/wreq-ruby/actions/workflows/ci.yml)

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

## Behavior

1. **HTTP/2 over TLS**

Due to the complexity of TLS encryption and the widespread adoption of HTTP/2, browser fingerprints such as **JA3**, **JA4**, and **Akamai** cannot be reliably emulated using simple fingerprint strings. Instead of parsing and emulating these string-based fingerprints, `rnet` provides fine-grained control over TLS and HTTP/2 extensions and settings for precise browser behavior emulation.

2. **Device Emulation**

Most browser device models share identical TLS and HTTP/2 configurations, differing only in the `User-Agent` string.

- <details>
  <summary>Available OS emulations</summary>

  | **OS**      | **Description**                |
  | ----------- | ------------------------------ |
  | **Windows** | Windows (any version)          |
  | **MacOS**   | macOS (any version)            |
  | **Linux**   | Linux (any distribution)       |
  | **Android** | Android (mobile)               |
  | **iOS**     | iOS (iPhone/iPad)              |

  </details>
- <details>
  <summary>Available browser emulations</summary>

  | **Browser** | **Versions**                                                                                                                                                                                                                                                                                                                                                                            |
  | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | **Chrome**  | `Chrome100`, `Chrome101`, `Chrome104`, `Chrome105`, `Chrome106`, `Chrome107`, `Chrome108`, `Chrome109`, `Chrome110`, `Chrome114`, `Chrome116`, `Chrome117`, `Chrome118`, `Chrome119`, `Chrome120`, `Chrome123`, `Chrome124`, `Chrome126`, `Chrome127`, `Chrome128`, `Chrome129`, `Chrome130`, `Chrome131`, `Chrome132`, `Chrome133`, `Chrome134`, `Chrome135`, `Chrome136`, `Chrome137`, `Chrome138`, `Chrome139`, `Chrome140`, `Chrome141`, `Chrome142`, `Chrome143` |
  | **Safari**  | `SafariIos17_2`, `SafariIos17_4_1`, `SafariIos16_5`, `Safari15_3`, `Safari15_5`, `Safari15_6_1`, `Safari16`, `Safari16_5`, `Safari17_0`, `Safari17_2_1`, `Safari17_4_1`, `Safari17_5`, `Safari18`, `SafariIPad18`, `Safari18_2`, `SafariIos18_1_1`, `Safari18_3`, `Safari18_3_1`, `Safari18_5`,  `Safari26`, `Safari26_1`, `Safari26_2`, `SafariIos26`, `SafariIos26_2`, `SafariIPad26`, `SafariIpad26_2`  |
  | **Firefox** | `Firefox109`, `Firefox117`, `Firefox128`, `Firefox133`, `Firefox135`, `FirefoxPrivate135`, `FirefoxAndroid135`, `Firefox136`, `FirefoxPrivate136`, `Firefox139`, `Firefox142`, `Firefox143`, `Firefox144`, `Firefox145`, `Firefox146` |
  | **OkHttp**  | `OkHttp3_9`, `OkHttp3_11`, `OkHttp3_13`, `OkHttp3_14`, `OkHttp4_9`, `OkHttp4_10`, `OkHttp4_12`, `OkHttp5`                                                                                                                                                                                                                                                                               |
  | **Edge**    | `Edge101`, `Edge122`, `Edge127`, `Edge131`, `Edge134`, `Edge135`, `Edge136`, `Edge137`, `Edge138`, `Edge139`, `Edge140`, `Edge141`, `Edge142`|
  | **Opera**   | `Opera116`, `Opera117`, `Opera118`, `Opera119`                                                                                                                                                                 |

  </details>

## Building

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
