use magnus::{
    Error, Module, Object, RHash, RModule, Ruby, TryConvert, Value, function, method, scan_args,
    typed_data::{Inspect, Obj},
};
use wreq::{Group, IntoEmulation};

use crate::{emulate::parse::ParserOptions, error::serde_json_error_to_magnus};

define_ruby_enum!(
    /// An emulation profile.
    const,
    Profile,
    "Wreq::Profile",
    wreq_util::Profile,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome110,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome126,
    Chrome127,
    Chrome128,
    Chrome129,
    Chrome130,
    Chrome131,
    Chrome132,
    Chrome133,
    Chrome134,
    Chrome135,
    Chrome136,
    Chrome137,
    Chrome138,
    Chrome139,
    Chrome140,
    Chrome141,
    Chrome142,
    Chrome143,
    Chrome144,
    Chrome145,
    Chrome146,
    Chrome147,
    Chrome148,

    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Edge134,
    Edge135,
    Edge136,
    Edge137,
    Edge138,
    Edge139,
    Edge140,
    Edge141,
    Edge142,
    Edge143,
    Edge144,
    Edge145,
    Edge146,
    Edge147,
    Edge148,

    Firefox109,
    Firefox117,
    Firefox128,
    Firefox133,
    Firefox135,
    FirefoxPrivate135,
    FirefoxAndroid135,
    Firefox136,
    FirefoxPrivate136,
    Firefox139,
    Firefox142,
    Firefox143,
    Firefox144,
    Firefox145,
    Firefox146,
    Firefox147,
    Firefox148,
    Firefox149,
    Firefox150,
    Firefox151,

    SafariIos17_2,
    SafariIos17_4_1,
    SafariIos16_5,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_0,
    Safari17_2_1,
    Safari17_4_1,
    Safari17_5,
    Safari17_6,
    Safari18,
    SafariIPad18,
    Safari18_2,
    Safari18_3,
    Safari18_3_1,
    SafariIos18_1_1,
    Safari18_5,
    Safari26,
    Safari26_1,
    Safari26_2,
    Safari26_3,
    Safari26_4,
    SafariIos26,
    SafariIos26_2,
    SafariIPad26,
    SafariIpad26_2,

    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp4_12,
    OkHttp5,

    Opera116,
    Opera117,
    Opera118,
    Opera119,
    Opera120,
    Opera121,
    Opera122,
    Opera123,
    Opera124,
    Opera125,
    Opera126,
    Opera127,
    Opera128,
    Opera129,
    Opera130,
    Opera131,
);

define_ruby_enum!(
    /// An emulation profile for OS.
    const,
    Platform,
    "Wreq::Platform",
    wreq_util::Platform,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
);

/// A struct to represent the `EmulationOption` class.
#[derive(Clone)]
#[magnus::wrap(class = "Wreq::Emulation", free_immediately, size)]
pub enum Emulation {
    Emulation(Box<wreq::Emulation>),
    EmulationOption(wreq_util::Emulation),
}

// ===== impl Profile =====

impl Profile {
    pub fn to_s(&self) -> String {
        self.into_ffi().inspect()
    }
}

// ===== impl Platform =====

impl Platform {
    pub fn to_s(&self) -> String {
        self.into_ffi().inspect()
    }
}

// ===== impl Emulation =====

impl Emulation {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let mut profile = None;
        let mut platform = None;
        let mut http2 = None;
        let mut headers = None;

        if let Some(hash) = args.first().and_then(|v| RHash::from_value(*v)) {
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(profile))) {
                profile = Some(Obj::<Profile>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(platform))) {
                platform = Some(Obj::<Platform>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(http2))) {
                http2 = Some(bool::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(headers))) {
                headers = Some(bool::try_convert(v)?);
            }
        }

        let emulation = wreq_util::Emulation::builder()
            .profile(profile.map(|obj| obj.into_ffi()).unwrap_or_default())
            .platform(platform.map(|os| os.into_ffi()).unwrap_or_default())
            .http2(http2.unwrap_or(true))
            .headers(headers.unwrap_or(true))
            .build();

        Ok(Self::EmulationOption(emulation))
    }

    fn parse(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::scan_args::<(String,), (), (), (), RHash, ()>(args)?;

        let json: serde_json::Value =
            serde_json::from_str(args.required.0.as_str()).map_err(serde_json_error_to_magnus)?;
        let opts: ParserOptions = serde_magnus::deserialize(ruby, args.keywords)?;

        let mut builder = wreq::Emulation::builder();

        if let Some(tls_options) = parse::parse_tls(&json, opts) {
            builder = builder.tls_options(tls_options);
        }

        if let Some((http2_options, headers, orig_headers)) = parse::parse_http2(&json) {
            builder = builder
                .http2_options(http2_options)
                .headers(headers)
                .orig_headers(orig_headers);
        }

        Ok(Self::Emulation(Box::new(builder.build(Group::default()))))
    }
}

impl IntoEmulation for Emulation {
    #[inline]
    fn into_emulation(self) -> wreq::Emulation {
        match self {
            Emulation::Emulation(e) => *e,
            Emulation::EmulationOption(opt) => opt.into_emulation(),
        }
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // Profile enum binding
    let profile = gem_module.define_class("Profile", ruby.class_object())?;
    profile.define_method("to_s", method!(Profile::to_s, 0))?;
    profile.const_set("Chrome100", Profile::Chrome100)?;
    profile.const_set("Chrome101", Profile::Chrome101)?;
    profile.const_set("Chrome104", Profile::Chrome104)?;
    profile.const_set("Chrome105", Profile::Chrome105)?;
    profile.const_set("Chrome106", Profile::Chrome106)?;
    profile.const_set("Chrome107", Profile::Chrome107)?;
    profile.const_set("Chrome108", Profile::Chrome108)?;
    profile.const_set("Chrome109", Profile::Chrome109)?;
    profile.const_set("Chrome110", Profile::Chrome110)?;
    profile.const_set("Chrome114", Profile::Chrome114)?;
    profile.const_set("Chrome116", Profile::Chrome116)?;
    profile.const_set("Chrome117", Profile::Chrome117)?;
    profile.const_set("Chrome118", Profile::Chrome118)?;
    profile.const_set("Chrome119", Profile::Chrome119)?;
    profile.const_set("Chrome120", Profile::Chrome120)?;
    profile.const_set("Chrome123", Profile::Chrome123)?;
    profile.const_set("Chrome124", Profile::Chrome124)?;
    profile.const_set("Chrome126", Profile::Chrome126)?;
    profile.const_set("Chrome127", Profile::Chrome127)?;
    profile.const_set("Chrome128", Profile::Chrome128)?;
    profile.const_set("Chrome129", Profile::Chrome129)?;
    profile.const_set("Chrome130", Profile::Chrome130)?;
    profile.const_set("Chrome131", Profile::Chrome131)?;
    profile.const_set("Chrome132", Profile::Chrome132)?;
    profile.const_set("Chrome133", Profile::Chrome133)?;
    profile.const_set("Chrome134", Profile::Chrome134)?;
    profile.const_set("Chrome135", Profile::Chrome135)?;
    profile.const_set("Chrome136", Profile::Chrome136)?;
    profile.const_set("Chrome137", Profile::Chrome137)?;
    profile.const_set("Chrome138", Profile::Chrome138)?;
    profile.const_set("Chrome139", Profile::Chrome139)?;
    profile.const_set("Chrome140", Profile::Chrome140)?;
    profile.const_set("Chrome141", Profile::Chrome141)?;
    profile.const_set("Chrome142", Profile::Chrome142)?;
    profile.const_set("Chrome143", Profile::Chrome143)?;
    profile.const_set("Chrome144", Profile::Chrome144)?;
    profile.const_set("Chrome145", Profile::Chrome145)?;
    profile.const_set("Chrome146", Profile::Chrome146)?;
    profile.const_set("Chrome147", Profile::Chrome147)?;
    profile.const_set("Chrome148", Profile::Chrome148)?;

    profile.const_set("Edge101", Profile::Edge101)?;
    profile.const_set("Edge122", Profile::Edge122)?;
    profile.const_set("Edge127", Profile::Edge127)?;
    profile.const_set("Edge131", Profile::Edge131)?;
    profile.const_set("Edge134", Profile::Edge134)?;
    profile.const_set("Edge135", Profile::Edge135)?;
    profile.const_set("Edge136", Profile::Edge136)?;
    profile.const_set("Edge137", Profile::Edge137)?;
    profile.const_set("Edge138", Profile::Edge138)?;
    profile.const_set("Edge139", Profile::Edge139)?;
    profile.const_set("Edge140", Profile::Edge140)?;
    profile.const_set("Edge141", Profile::Edge141)?;
    profile.const_set("Edge142", Profile::Edge142)?;
    profile.const_set("Edge143", Profile::Edge143)?;
    profile.const_set("Edge144", Profile::Edge144)?;
    profile.const_set("Edge145", Profile::Edge145)?;
    profile.const_set("Edge146", Profile::Edge146)?;
    profile.const_set("Edge147", Profile::Edge147)?;
    profile.const_set("Edge148", Profile::Edge148)?;

    profile.const_set("Firefox109", Profile::Firefox109)?;
    profile.const_set("Firefox117", Profile::Firefox117)?;
    profile.const_set("Firefox128", Profile::Firefox128)?;
    profile.const_set("Firefox133", Profile::Firefox133)?;
    profile.const_set("Firefox135", Profile::Firefox135)?;
    profile.const_set("FirefoxPrivate135", Profile::FirefoxPrivate135)?;
    profile.const_set("FirefoxAndroid135", Profile::FirefoxAndroid135)?;
    profile.const_set("Firefox136", Profile::Firefox136)?;
    profile.const_set("FirefoxPrivate136", Profile::FirefoxPrivate136)?;
    profile.const_set("Firefox139", Profile::Firefox139)?;
    profile.const_set("Firefox142", Profile::Firefox142)?;
    profile.const_set("Firefox143", Profile::Firefox143)?;
    profile.const_set("Firefox144", Profile::Firefox144)?;
    profile.const_set("Firefox145", Profile::Firefox145)?;
    profile.const_set("Firefox146", Profile::Firefox146)?;
    profile.const_set("Firefox147", Profile::Firefox147)?;
    profile.const_set("Firefox148", Profile::Firefox148)?;
    profile.const_set("Firefox149", Profile::Firefox149)?;
    profile.const_set("Firefox150", Profile::Firefox150)?;
    profile.const_set("Firefox151", Profile::Firefox151)?;

    profile.const_set("SafariIos17_2", Profile::SafariIos17_2)?;
    profile.const_set("SafariIos17_4_1", Profile::SafariIos17_4_1)?;
    profile.const_set("SafariIos16_5", Profile::SafariIos16_5)?;
    profile.const_set("Safari15_3", Profile::Safari15_3)?;
    profile.const_set("Safari15_5", Profile::Safari15_5)?;
    profile.const_set("Safari15_6_1", Profile::Safari15_6_1)?;
    profile.const_set("Safari16", Profile::Safari16)?;
    profile.const_set("Safari16_5", Profile::Safari16_5)?;
    profile.const_set("Safari17_0", Profile::Safari17_0)?;
    profile.const_set("Safari17_2_1", Profile::Safari17_2_1)?;
    profile.const_set("Safari17_4_1", Profile::Safari17_4_1)?;
    profile.const_set("Safari17_5", Profile::Safari17_5)?;
    profile.const_set("Safari17_6", Profile::Safari17_6)?;
    profile.const_set("Safari18", Profile::Safari18)?;
    profile.const_set("SafariIPad18", Profile::SafariIPad18)?;
    profile.const_set("Safari18_2", Profile::Safari18_2)?;
    profile.const_set("Safari18_3", Profile::Safari18_3)?;
    profile.const_set("Safari18_3_1", Profile::Safari18_3_1)?;
    profile.const_set("SafariIos18_1_1", Profile::SafariIos18_1_1)?;
    profile.const_set("Safari18_5", Profile::Safari18_5)?;
    profile.const_set("Safari26", Profile::Safari26)?;
    profile.const_set("Safari26_1", Profile::Safari26_1)?;
    profile.const_set("Safari26_2", Profile::Safari26_2)?;
    profile.const_set("Safari26_3", Profile::Safari26_3)?;
    profile.const_set("Safari26_4", Profile::Safari26_4)?;
    profile.const_set("SafariIos26", Profile::SafariIos26)?;
    profile.const_set("SafariIos26_2", Profile::SafariIos26_2)?;
    profile.const_set("SafariIPad26", Profile::SafariIPad26)?;
    profile.const_set("SafariIpad26_2", Profile::SafariIpad26_2)?;

    profile.const_set("OkHttp3_9", Profile::OkHttp3_9)?;
    profile.const_set("OkHttp3_11", Profile::OkHttp3_11)?;
    profile.const_set("OkHttp3_13", Profile::OkHttp3_13)?;
    profile.const_set("OkHttp3_14", Profile::OkHttp3_14)?;
    profile.const_set("OkHttp4_9", Profile::OkHttp4_9)?;
    profile.const_set("OkHttp4_10", Profile::OkHttp4_10)?;
    profile.const_set("OkHttp4_12", Profile::OkHttp4_12)?;
    profile.const_set("OkHttp5", Profile::OkHttp5)?;

    profile.const_set("Opera116", Profile::Opera116)?;
    profile.const_set("Opera117", Profile::Opera117)?;
    profile.const_set("Opera118", Profile::Opera118)?;
    profile.const_set("Opera119", Profile::Opera119)?;
    profile.const_set("Opera120", Profile::Opera120)?;
    profile.const_set("Opera121", Profile::Opera121)?;
    profile.const_set("Opera122", Profile::Opera122)?;
    profile.const_set("Opera123", Profile::Opera123)?;
    profile.const_set("Opera124", Profile::Opera124)?;
    profile.const_set("Opera125", Profile::Opera125)?;
    profile.const_set("Opera126", Profile::Opera126)?;
    profile.const_set("Opera127", Profile::Opera127)?;
    profile.const_set("Opera128", Profile::Opera128)?;
    profile.const_set("Opera129", Profile::Opera129)?;
    profile.const_set("Opera130", Profile::Opera130)?;
    profile.const_set("Opera131", Profile::Opera131)?;

    // Platform enum binding
    let platform = gem_module.define_class("Platform", ruby.class_object())?;
    platform.define_method("to_s", method!(Platform::to_s, 0))?;
    platform.const_set("Windows", Platform::Windows)?;
    platform.const_set("MacOS", Platform::MacOS)?;
    platform.const_set("Linux", Platform::Linux)?;
    platform.const_set("Android", Platform::Android)?;
    platform.const_set("IOS", Platform::IOS)?;

    // Emulation class binding
    let emulation = gem_module.define_class("Emulation", ruby.class_object())?;
    emulation.define_singleton_method("new", function!(Emulation::new, -1))?;
    emulation.define_singleton_method("parse", function!(Emulation::parse, -1))?;
    Ok(())
}

mod parse {
    //! //! Currently, only newer versions of Chrome support parsing https://tls.peet.ws/api/all.

    use http::{HeaderMap, HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::Value;
    use std::str::FromStr;
    use wreq::{
        header::OrigHeaderMap,
        http2::{
            Http2Options, PseudoId, PseudoOrder, SettingId, SettingsOrder, StreamDependency,
            StreamId,
        },
        tls::{
            AlpnProtocol, AlpsProtocol, TlsOptions, TlsVersion, compress::CertificateCompressor,
        },
    };
    use wreq_util::emulate::compress;

    macro_rules! get {
        ($json:expr, $key:ident, $method:ident) => {
            $json.get(stringify!($key)).and_then(|v| v.$method())
        };
    }

    macro_rules! find {
        ($array:expr, $key1:ident, $method1:ident, $key2:ident) => {
            $array
                .iter()
                .find(|v| get!(v, $key1, $method1) == Some(stringify!($key2)))
        };
    }

    macro_rules! get_and_then {
        ($json:expr, $key1:ident, $method1:ident, $key2:ident, $method2:ident) => {
            get!($json, $key1, $method1).and_then(|v| get!(v, $key2, $method2))
        };
    }

    macro_rules! find_and_then {
        ($array:expr, $key1:ident, $method1:ident, $key2:ident, $key3:ident, $method2:ident) => {
            $array
                .iter()
                .find(|v| get!(v, $key1, $method1) == Some(stringify!($key2)))
                .and_then(|v| get!(v, $key3, $method2))
        };
    }

    #[derive(Default, Deserialize)]
    #[non_exhaustive]
    pub struct ParserOptions {
        /// Whether to skip session tickets when using PSK.
        #[serde(default)]
        psk_skip_session_ticket: bool,

        /// Controls whether ClientHello extensions should be permuted.
        #[serde(default)]
        permute_extensions: Option<bool>,

        /// Overrides AES hardware acceleration.
        #[serde(default)]
        aes_hw_override: Option<bool>,

        /// Overrides the random AES hardware acceleration.
        #[serde(default)]
        random_aes_hw_override: bool,
    }

    pub fn parse_tls(json: &Value, opts: ParserOptions) -> Option<TlsOptions> {
        let tls = get!(json, tls, as_object)?;
        let mut tls_builder = TlsOptions::builder()
            .aes_hw_override(opts.aes_hw_override)
            .random_aes_hw_override(opts.random_aes_hw_override)
            .permute_extensions(opts.permute_extensions)
            .psk_skip_session_ticket(opts.psk_skip_session_ticket);

        // parse ciphers
        if let Some(ciphers) = get!(tls, ciphers, as_array) {
            let ciphers_list = ciphers
                .iter()
                .flat_map(|v| v.as_str())
                .filter(|s| !s.is_empty() && !s.starts_with(stringify!(TLS_GREASE)))
                .collect::<Vec<_>>()
                .join(":");
            tls_builder = tls_builder
                .cipher_list(ciphers_list)
                .preserve_tls13_cipher_list(true)
        }

        for extension in get!(tls, extensions, as_array)? {
            let Some(name) =
                get!(extension, name, as_str).and_then(|s| s.split_whitespace().next())
            else {
                continue;
            };

            tls_builder = match name {
                stringify!(session_ticket) => tls_builder.session_ticket(true),
                stringify!(extensionEncryptedClientHello) => tls_builder.enable_ech_grease(true),
                stringify!(signed_certificate_timestamp) => {
                    tls_builder.enable_signed_cert_timestamps(true)
                }
                stringify!(ec_point_formats) | stringify!(extended_master_secret) => {
                    continue;
                }
                stringify!(extensionRenegotiationInfo) => tls_builder.renegotiation(true),
                stringify!(key_share) => {
                    continue;
                }
                stringify!(supported_versions) => {
                    let Some(versions) = get!(extension, versions, as_array) else {
                        continue;
                    };

                    for version in versions
                        .iter()
                        .filter_map(|s| s.as_str())
                        .map(|s| s.split_whitespace().nth(1))
                    {
                        tls_builder = match version {
                            Some(stringify!(1.0)) => {
                                tls_builder.min_tls_version(TlsVersion::TLS_1_0)
                            }
                            Some(stringify!(1.1)) => {
                                tls_builder.min_tls_version(TlsVersion::TLS_1_1)
                            }
                            Some(stringify!(1.2)) => {
                                tls_builder.min_tls_version(TlsVersion::TLS_1_2)
                            }
                            Some(stringify!(1.3)) => {
                                tls_builder.max_tls_version(TlsVersion::TLS_1_3)
                            }
                            Some(_) | None => {
                                continue;
                            }
                        }
                    }

                    tls_builder
                }
                stringify!(application_settings) | stringify!(application_settings_old) => {
                    let Some(protocols) = get!(extension, protocols, as_array) else {
                        continue;
                    };

                    let protocols = protocols
                        .iter()
                        .filter_map(|v| v.as_str())
                        .flat_map(|s| match s {
                            "http/1.1" => Some(AlpsProtocol::HTTP1),
                            stringify!(h2) => Some(AlpsProtocol::HTTP2),
                            stringify!(h3) => Some(AlpsProtocol::HTTP3),
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    tls_builder
                        .alps_protocols(protocols)
                        .alps_use_new_codepoint(name == stringify!(application_settings))
                }
                stringify!(application_layer_protocol_negotiation) => {
                    let Some(protocols) = get!(extension, protocols, as_array) else {
                        continue;
                    };

                    let protocols = protocols
                        .iter()
                        .filter_map(|v| v.as_str())
                        .flat_map(|s| match s {
                            "http/1.1" => Some(AlpnProtocol::HTTP1),
                            stringify!(h2) => Some(AlpnProtocol::HTTP2),
                            stringify!(h3) => Some(AlpnProtocol::HTTP3),
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    tls_builder.alpn_protocols(protocols)
                }
                stringify!(status_request) => tls_builder.enable_ocsp_stapling(true),
                stringify!(psk_key_exchange_modes) => tls_builder.psk_dhe_ke(true),
                stringify!(supported_groups) => {
                    let Some(groups) = get!(extension, supported_groups, as_array) else {
                        continue;
                    };

                    let groups = groups
                        .iter()
                        .filter_map(|s| s.as_str())
                        .filter(|s| !s.is_empty() && !s.starts_with(stringify!(TLS_GREASE)))
                        .flat_map(|s| s.split_whitespace().next())
                        .collect::<Vec<&str>>()
                        .join(":");

                    tls_builder.curves_list(groups)
                }
                stringify!(compress_certificate) => {
                    let Some(algorithms) = get!(extension, algorithms, as_array) else {
                        continue;
                    };

                    let algorithms = algorithms
                        .iter()
                        .filter_map(|s| s.as_str())
                        .filter(|s| !s.is_empty())
                        .flat_map(|s| match s.split_whitespace().next() {
                            Some(stringify!(zlib)) => Some(&compress::ZlibCompressor as _),
                            Some(stringify!(brotli)) => Some(&compress::BrotliCompressor as _),
                            Some(stringify!(zstd)) => Some(&compress::ZstdCompressor as _),
                            Some(_) | None => None,
                        })
                        .collect::<Vec<&'static dyn CertificateCompressor>>();

                    tls_builder.certificate_compressors(algorithms)
                }
                stringify!(signature_algorithms) => {
                    let Some(algorithms) = get!(extension, signature_algorithms, as_array) else {
                        continue;
                    };

                    let algorithms = algorithms
                        .iter()
                        .filter_map(|s| s.as_str())
                        .filter(|s| !s.is_empty())
                        .flat_map(|s| s.split_whitespace().next())
                        .collect::<Vec<&str>>()
                        .join(":");

                    tls_builder.sigalgs_list(algorithms)
                }
                stringify!(pre_shared_key) => tls_builder.pre_shared_key(true),
                name if name.starts_with(stringify!(TLS_GREASE)) => {
                    tls_builder.grease_enabled(true)
                }
                _ => continue,
            };
        }

        Some(tls_builder.build())
    }

    pub fn parse_http2(json: &Value) -> Option<(Http2Options, HeaderMap, OrigHeaderMap)> {
        let sent_frames = get_and_then!(json, http2, as_object, sent_frames, as_array)?;

        let mut http2_builder = Http2Options::builder();

        // parse settings frame
        if let Some(settings) = find_and_then!(
            sent_frames,
            frame_type,
            as_str,
            SETTINGS,
            settings,
            as_array
        ) {
            let mut settings_order = SettingsOrder::builder();
            for setting in settings.iter().filter_map(|v| v.as_str()) {
                let mut parts = setting.split('=');
                let (Some(name), Some(value)) = (parts.next(), parts.next()) else {
                    continue;
                };
                let value = value.trim();

                match name.trim() {
                    stringify!(HEADER_TABLE_SIZE) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.header_table_size(value);
                            settings_order = settings_order.push(SettingId::HeaderTableSize);
                        }
                    }
                    stringify!(ENABLE_PUSH) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.enable_push(value != 0);
                            settings_order = settings_order.push(SettingId::EnablePush);
                        }
                    }
                    stringify!(INITIAL_WINDOW_SIZE) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.initial_window_size(value);
                            settings_order = settings_order.push(SettingId::InitialWindowSize);
                        }
                    }
                    stringify!(MAX_FRAME_SIZE) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.max_frame_size(value);
                            settings_order = settings_order.push(SettingId::MaxFrameSize);
                        }
                    }
                    stringify!(MAX_HEADER_LIST_SIZE) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.max_header_list_size(value);
                            settings_order = settings_order.push(SettingId::MaxHeaderListSize);
                        }
                    }
                    stringify!(MAX_CONCURRENT_STREAMS) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.max_concurrent_streams(value);
                            settings_order = settings_order.push(SettingId::MaxConcurrentStreams);
                        }
                    }
                    stringify!(ENABLE_CONNECT_PROTOCOL) | stringify!(UNKNOWN_SETTING_8) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.enable_connect_protocol(value != 0);
                            settings_order = settings_order.push(SettingId::EnableConnectProtocol);
                        }
                    }
                    stringify!(NO_RFC7540_PRIORITIES) => {
                        if let Ok(value) = value.parse::<u32>() {
                            http2_builder = http2_builder.no_rfc7540_priorities(value != 0);
                            settings_order = settings_order.push(SettingId::NoRfc7540Priorities);
                        }
                    }
                    _ => {}
                }
            }

            http2_builder = http2_builder.settings_order(settings_order.build());
        }

        // parse window update frame
        if let Some(window_update) = find_and_then!(
            sent_frames,
            frame_type,
            as_str,
            WINDOW_UPDATE,
            increment,
            as_u64
        ) {
            http2_builder =
                http2_builder.initial_connection_window_size((window_update + 65535) as u32);
        }

        let mut headers_map = HeaderMap::new();
        let mut orig_headers_map = OrigHeaderMap::new();

        // parse headers frame
        if let Some(headers_frame) = find!(sent_frames, frame_type, as_str, HEADERS) {
            // parse initial stream id
            if let Some(init_stream_id) = get!(headers_frame, stream_id, as_u64).filter(|v| *v != 0)
            {
                http2_builder = http2_builder.initial_stream_id(init_stream_id as u32);
            }

            // parse headers
            if let Some(headers) = get!(headers_frame, headers, as_array) {
                let mut pseudo_builder = PseudoOrder::builder();

                for (name, value) in headers
                    .iter()
                    .filter_map(|h| h.as_str())
                    .filter_map(|h| h.split_once(": "))
                {
                    match name {
                        stringify!(:method) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Method);
                        }
                        stringify!(:path) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Path);
                        }
                        stringify!(:scheme) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Scheme);
                        }
                        stringify!(:authority) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Authority);
                        }
                        stringify!(:status) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Status);
                        }
                        stringify!(:protocol) => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Protocol);
                        }
                        _ => {
                            if let (Ok(header_name), Ok(header_value)) =
                                (HeaderName::from_str(name), HeaderValue::from_str(value))
                            {
                                headers_map.insert(&header_name, header_value);
                                orig_headers_map.insert(header_name);
                            }
                        }
                    }
                }

                http2_builder = http2_builder.headers_pseudo_order(pseudo_builder.build());
            };

            // parse header priority
            if let Some(priority) = get!(headers_frame, priority, as_object) {
                if let (Some(depends_on), Some(weight), Some(exclusive)) = (
                    get!(priority, depends_on, as_u64),
                    get!(priority, weight, as_u64),
                    get!(priority, exclusive, as_u64),
                ) {
                    http2_builder = http2_builder.headers_stream_dependency(StreamDependency::new(
                        if depends_on == 0 {
                            StreamId::zero()
                        } else {
                            StreamId::from(depends_on as u32)
                        },
                        (weight - 1) as u8,
                        exclusive != 0,
                    ));
                }
            }
        }

        Some((http2_builder.build(), headers_map, orig_headers_map))
    }
}
