use magnus::{
    Error, Module, Object, RHash, RModule, Ruby, TryConvert, Value, function, method, scan_args,
    typed_data::{Inspect, Obj},
};
use wreq::EmulationFactory;

use crate::{emulate::parse::ParserOptions, error::serde_json_error_to_magnus};

define_ruby_enum!(
    /// An emulation.
    const,
    EmulationDevice,
    "Wreq::EmulationDevice",
    wreq_util::Emulation,
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
    Opera119
);

define_ruby_enum!(
    /// An emulation operating system.
    const,
    EmulationOS,
    "Wreq::EmulationOS",
    wreq_util::EmulationOS,
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
    EmulationOption(wreq_util::EmulationOption),
}

// ===== impl EmulationDevice =====

impl EmulationDevice {
    pub fn to_s(&self) -> String {
        self.into_ffi().inspect()
    }
}

// ===== impl EmulationOS =====

impl EmulationOS {
    pub fn to_s(&self) -> String {
        self.into_ffi().inspect()
    }
}

// ===== impl Emulation =====

impl Emulation {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let mut device = None;
        let mut os = None;
        let mut skip_http2 = None;
        let mut skip_headers = None;

        if let Some(hash) = args.first().and_then(|v| RHash::from_value(*v)) {
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(device))) {
                device = Some(Obj::<EmulationDevice>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(os))) {
                os = Some(Obj::<EmulationOS>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(skip_http2))) {
                skip_http2 = Some(bool::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(skip_headers))) {
                skip_headers = Some(bool::try_convert(v)?);
            }
        }

        let emulation = wreq_util::EmulationOption::builder()
            .emulation(device.map(|obj| obj.into_ffi()).unwrap_or_default())
            .emulation_os(os.map(|os| os.into_ffi()).unwrap_or_default())
            .skip_http2(skip_http2.unwrap_or(false))
            .skip_headers(skip_headers.unwrap_or(false))
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

        if let Some((http2_options, headers)) = parse::parse_http2(&json) {
            builder = builder.http2_options(http2_options).headers(headers);
        }

        Ok(Self::Emulation(Box::new(builder.build())))
    }
}

impl EmulationFactory for Emulation {
    #[inline]
    fn emulation(self) -> wreq::Emulation {
        match self {
            Emulation::Emulation(e) => *e,
            Emulation::EmulationOption(opt) => opt.emulation(),
        }
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // EmulationDevice enum binding
    let device_class = gem_module.define_class("EmulationDevice", ruby.class_object())?;
    device_class.define_method("to_s", method!(EmulationDevice::to_s, 0))?;
    device_class.const_set("Chrome100", EmulationDevice::Chrome100)?;
    device_class.const_set("Chrome101", EmulationDevice::Chrome101)?;
    device_class.const_set("Chrome104", EmulationDevice::Chrome104)?;
    device_class.const_set("Chrome105", EmulationDevice::Chrome105)?;
    device_class.const_set("Chrome106", EmulationDevice::Chrome106)?;
    device_class.const_set("Chrome107", EmulationDevice::Chrome107)?;
    device_class.const_set("Chrome108", EmulationDevice::Chrome108)?;
    device_class.const_set("Chrome109", EmulationDevice::Chrome109)?;
    device_class.const_set("Chrome110", EmulationDevice::Chrome110)?;
    device_class.const_set("Chrome114", EmulationDevice::Chrome114)?;
    device_class.const_set("Chrome116", EmulationDevice::Chrome116)?;
    device_class.const_set("Chrome117", EmulationDevice::Chrome117)?;
    device_class.const_set("Chrome118", EmulationDevice::Chrome118)?;
    device_class.const_set("Chrome119", EmulationDevice::Chrome119)?;
    device_class.const_set("Chrome120", EmulationDevice::Chrome120)?;
    device_class.const_set("Chrome123", EmulationDevice::Chrome123)?;
    device_class.const_set("Chrome124", EmulationDevice::Chrome124)?;
    device_class.const_set("Chrome126", EmulationDevice::Chrome126)?;
    device_class.const_set("Chrome127", EmulationDevice::Chrome127)?;
    device_class.const_set("Chrome128", EmulationDevice::Chrome128)?;
    device_class.const_set("Chrome129", EmulationDevice::Chrome129)?;
    device_class.const_set("Chrome130", EmulationDevice::Chrome130)?;
    device_class.const_set("Chrome131", EmulationDevice::Chrome131)?;
    device_class.const_set("Chrome132", EmulationDevice::Chrome132)?;
    device_class.const_set("Chrome133", EmulationDevice::Chrome133)?;
    device_class.const_set("Chrome134", EmulationDevice::Chrome134)?;
    device_class.const_set("Chrome135", EmulationDevice::Chrome135)?;
    device_class.const_set("Chrome136", EmulationDevice::Chrome136)?;
    device_class.const_set("Chrome137", EmulationDevice::Chrome137)?;
    device_class.const_set("Chrome138", EmulationDevice::Chrome138)?;
    device_class.const_set("Chrome139", EmulationDevice::Chrome139)?;
    device_class.const_set("Chrome140", EmulationDevice::Chrome140)?;
    device_class.const_set("Chrome141", EmulationDevice::Chrome141)?;
    device_class.const_set("Chrome142", EmulationDevice::Chrome142)?;
    device_class.const_set("Chrome143", EmulationDevice::Chrome143)?;
    device_class.const_set("Chrome144", EmulationDevice::Chrome144)?;
    device_class.const_set("Chrome145", EmulationDevice::Chrome145)?;
    device_class.const_set("Edge101", EmulationDevice::Edge101)?;
    device_class.const_set("Edge122", EmulationDevice::Edge122)?;
    device_class.const_set("Edge127", EmulationDevice::Edge127)?;
    device_class.const_set("Edge131", EmulationDevice::Edge131)?;
    device_class.const_set("Edge134", EmulationDevice::Edge134)?;
    device_class.const_set("Edge135", EmulationDevice::Edge135)?;
    device_class.const_set("Edge136", EmulationDevice::Edge136)?;
    device_class.const_set("Edge137", EmulationDevice::Edge137)?;
    device_class.const_set("Edge138", EmulationDevice::Edge138)?;
    device_class.const_set("Edge139", EmulationDevice::Edge139)?;
    device_class.const_set("Edge140", EmulationDevice::Edge140)?;
    device_class.const_set("Edge141", EmulationDevice::Edge141)?;
    device_class.const_set("Edge142", EmulationDevice::Edge142)?;
    device_class.const_set("Edge143", EmulationDevice::Edge143)?;
    device_class.const_set("Edge144", EmulationDevice::Edge144)?;
    device_class.const_set("Edge145", EmulationDevice::Edge145)?;
    device_class.const_set("Firefox109", EmulationDevice::Firefox109)?;
    device_class.const_set("Firefox117", EmulationDevice::Firefox117)?;
    device_class.const_set("Firefox128", EmulationDevice::Firefox128)?;
    device_class.const_set("Firefox133", EmulationDevice::Firefox133)?;
    device_class.const_set("Firefox135", EmulationDevice::Firefox135)?;
    device_class.const_set("FirefoxPrivate135", EmulationDevice::FirefoxPrivate135)?;
    device_class.const_set("FirefoxAndroid135", EmulationDevice::FirefoxAndroid135)?;
    device_class.const_set("Firefox136", EmulationDevice::Firefox136)?;
    device_class.const_set("FirefoxPrivate136", EmulationDevice::FirefoxPrivate136)?;
    device_class.const_set("Firefox139", EmulationDevice::Firefox139)?;
    device_class.const_set("Firefox142", EmulationDevice::Firefox142)?;
    device_class.const_set("Firefox143", EmulationDevice::Firefox143)?;
    device_class.const_set("Firefox144", EmulationDevice::Firefox144)?;
    device_class.const_set("Firefox145", EmulationDevice::Firefox145)?;
    device_class.const_set("Firefox146", EmulationDevice::Firefox146)?;
    device_class.const_set("Firefox147", EmulationDevice::Firefox147)?;
    device_class.const_set("SafariIos17_2", EmulationDevice::SafariIos17_2)?;
    device_class.const_set("SafariIos17_4_1", EmulationDevice::SafariIos17_4_1)?;
    device_class.const_set("SafariIos16_5", EmulationDevice::SafariIos16_5)?;
    device_class.const_set("Safari15_3", EmulationDevice::Safari15_3)?;
    device_class.const_set("Safari15_5", EmulationDevice::Safari15_5)?;
    device_class.const_set("Safari15_6_1", EmulationDevice::Safari15_6_1)?;
    device_class.const_set("Safari16", EmulationDevice::Safari16)?;
    device_class.const_set("Safari16_5", EmulationDevice::Safari16_5)?;
    device_class.const_set("Safari17_0", EmulationDevice::Safari17_0)?;
    device_class.const_set("Safari17_2_1", EmulationDevice::Safari17_2_1)?;
    device_class.const_set("Safari17_4_1", EmulationDevice::Safari17_4_1)?;
    device_class.const_set("Safari17_5", EmulationDevice::Safari17_5)?;
    device_class.const_set("Safari17_6", EmulationDevice::Safari17_6)?;
    device_class.const_set("Safari18", EmulationDevice::Safari18)?;
    device_class.const_set("SafariIPad18", EmulationDevice::SafariIPad18)?;
    device_class.const_set("Safari18_2", EmulationDevice::Safari18_2)?;
    device_class.const_set("Safari18_3", EmulationDevice::Safari18_3)?;
    device_class.const_set("Safari18_3_1", EmulationDevice::Safari18_3_1)?;
    device_class.const_set("SafariIos18_1_1", EmulationDevice::SafariIos18_1_1)?;
    device_class.const_set("Safari18_5", EmulationDevice::Safari18_5)?;
    device_class.const_set("Safari26", EmulationDevice::Safari26)?;
    device_class.const_set("Safari26_1", EmulationDevice::Safari26_1)?;
    device_class.const_set("Safari26_2", EmulationDevice::Safari26_2)?;
    device_class.const_set("SafariIos26", EmulationDevice::SafariIos26)?;
    device_class.const_set("SafariIos26_2", EmulationDevice::SafariIos26_2)?;
    device_class.const_set("SafariIPad26", EmulationDevice::SafariIPad26)?;
    device_class.const_set("SafariIpad26_2", EmulationDevice::SafariIpad26_2)?;
    device_class.const_set("OkHttp3_9", EmulationDevice::OkHttp3_9)?;
    device_class.const_set("OkHttp3_11", EmulationDevice::OkHttp3_11)?;
    device_class.const_set("OkHttp3_13", EmulationDevice::OkHttp3_13)?;
    device_class.const_set("OkHttp3_14", EmulationDevice::OkHttp3_14)?;
    device_class.const_set("OkHttp4_9", EmulationDevice::OkHttp4_9)?;
    device_class.const_set("OkHttp4_10", EmulationDevice::OkHttp4_10)?;
    device_class.const_set("OkHttp4_12", EmulationDevice::OkHttp4_12)?;
    device_class.const_set("OkHttp5", EmulationDevice::OkHttp5)?;
    device_class.const_set("Opera116", EmulationDevice::Opera116)?;
    device_class.const_set("Opera117", EmulationDevice::Opera117)?;
    device_class.const_set("Opera118", EmulationDevice::Opera118)?;
    device_class.const_set("Opera119", EmulationDevice::Opera119)?;

    // EmulationOS enum binding
    let os_class = gem_module.define_class("EmulationOS", ruby.class_object())?;
    os_class.define_method("to_s", method!(EmulationOS::to_s, 0))?;
    os_class.const_set("Windows", EmulationOS::Windows)?;
    os_class.const_set("MacOS", EmulationOS::MacOS)?;
    os_class.const_set("Linux", EmulationOS::Linux)?;
    os_class.const_set("Android", EmulationOS::Android)?;
    os_class.const_set("IOS", EmulationOS::IOS)?;

    // Emulation class binding
    let class = gem_module.define_class("Emulation", ruby.class_object())?;
    class.define_singleton_method("new", function!(Emulation::new, -1))?;
    class.define_singleton_method("parse", function!(Emulation::parse, -1))?;
    Ok(())
}

mod parse {
    //! //! Currently, only newer versions of Chrome support parsing https://tls.peet.ws/api/all.

    use http::{HeaderMap, HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::Value;
    use std::str::FromStr;
    use wreq::{
        http2::{
            Http2Options, PseudoId, PseudoOrder, SettingId, SettingsOrder, StreamDependency,
            StreamId,
        },
        tls::{
            AlpnProtocol, AlpsProtocol, CertificateCompressionAlgorithm, TlsOptions, TlsVersion,
        },
    };

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
                    // todo: parse ec point formats and extended master secret
                    continue;
                }
                stringify!(extensionRenegotiationInfo) => tls_builder.renegotiation(true),
                stringify!(key_share) => {
                    // todo: parse key share groups
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
                            Some(stringify!(zlib)) => Some(CertificateCompressionAlgorithm::ZLIB),
                            Some(stringify!(brotli)) => {
                                Some(CertificateCompressionAlgorithm::BROTLI)
                            }
                            Some(stringify!(zstd)) => Some(CertificateCompressionAlgorithm::ZSTD),
                            Some(_) | None => None,
                        })
                        .collect::<Vec<_>>();

                    tls_builder.certificate_compression_algorithms(algorithms)
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

    pub fn parse_http2(json: &Value) -> Option<(Http2Options, HeaderMap)> {
        let sent_frames = get_and_then!(json, http2, as_object, sent_frames, as_array)?;

        let mut http2_builder = Http2Options::builder();
        let mut headers_map = HeaderMap::new();

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
                                headers_map.insert(header_name, header_value);
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

        Some((http2_builder.build(), headers_map))
    }
}
