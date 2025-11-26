#!/usr/bin/env ruby

require_relative "../lib/wreq"

# Example: How to use Emulation options in Wreq requests
#
# Emulation can be set globally on the client (applies to all requests),
# or per-request (overrides client default for that call).
#
# Global client emulation:
#   Set when creating the Wreq::Client instance.
#   All requests from this client will use the specified emulation unless overridden.
client = Wreq::Client.new(emulation: Wreq::Emulation.new(
  device: Wreq::EmulationDevice::Chrome142,
  os: Wreq::EmulationOS::MacOS,
  skip_http2: false,
  skip_headers: false
))

resp = client.get("https://tls.peet.ws/api/all")
puts resp.text

# Per-request emulation:
#   Pass the emulation option to a single request method.
#   This allows you to override the client default for specific calls.
resp = client.get(
  "https://tls.peet.ws/api/all",
  emulation: Wreq::Emulation.new(
    device: Wreq::EmulationDevice::Safari26,
    os: Wreq::EmulationOS::MacOS,
    skip_http2: false,
    skip_headers: false
  ),
  # Skip client default headers for this request
  default_headers: false
)
puts resp.text
