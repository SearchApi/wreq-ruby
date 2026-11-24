//! Presets and shared options for Chromium-based browsers.

pub mod chrome;
pub(in crate::emulate) mod edge;
pub mod opera;

mod header;
mod http2;
mod tls;
