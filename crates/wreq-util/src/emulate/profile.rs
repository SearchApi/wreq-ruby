//! Emulation for different browsers.

pub mod chrome;
pub mod firefox;
pub mod okhttp;
pub mod opera;
pub mod safari;

use typed_builder::TypedBuilder;
#[cfg(feature = "emulation-compression")]
use wreq::header::ACCEPT_ENCODING;
use wreq::{
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    http2::{
        HeadersDependency, HeadersPriority, Http2Options, Priorities, Priority, PseudoId,
        PseudoOrder, SettingId, SettingsOrder, StreamDependency, StreamId,
    },
    tls::{
        AlpnProtocol, AlpsProtocol, EchGrease, EchGreaseAead, ExtensionType, KeyShare, Tls13Cipher,
        TlsOptions, TlsVersion, compress::CertificateCompressor,
    },
};

use super::{
    Emulation, Platform,
    compress::{BrotliCompressor, ZlibCompressor, ZstdCompressor},
};

fn build_standard_emulation(
    tls_options: TlsOptions,
    http2_options: Option<Http2Options>,
    default_headers: Option<HeaderMap>,
) -> wreq::Emulation {
    let mut builder = wreq::Emulation::builder().config(tls_options);

    if let Some(http2_options) = http2_options {
        builder = builder.config(http2_options);
    }

    if let Some(headers) = default_headers {
        builder = builder.headers(headers);
    }

    builder.build()
}
