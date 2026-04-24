use magnus::{
    Error, Module, Object, RHash, RModule, Ruby, TryConvert, Value, function, method,
    typed_data::{Inspect, Obj},
};
use wreq::EmulationFactory;

use crate::error::serde_json_error_to_magnus;

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
    Emulation(wreq::Emulation),
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

    fn parse(s: String) -> Result<Self, Error> {
        let json: serde_json::Value =
            serde_json::from_str(&s).map_err(serde_json_error_to_magnus)?;
        let builder = parse::parse_tls(&json, wreq::Emulation::builder())?;
        let builder = parse::parse_http2(&json, builder)?;
        Ok(Self::Emulation(builder.build()))
    }
}

impl EmulationFactory for Emulation {
    #[inline]
    fn emulation(self) -> wreq::Emulation {
        match self {
            Emulation::Emulation(e) => e,
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
    class.define_singleton_method("parse", function!(Emulation::parse, 1))?;
    Ok(())
}

mod parse {
    use std::str::FromStr;

    use http::{HeaderMap, HeaderName, HeaderValue};
    use magnus::Error;
    use serde_json::Value;
    use wreq::{
        EmulationBuilder,
        http2::{
            Http2Options, PseudoId, PseudoOrder, SettingId, SettingsOrder, StreamDependency,
            StreamId,
        },
        tls::TlsOptions,
    };

    // Only support parse https://tls.peet.ws/api/all
    use crate::error::{header_name_error_to_magnus, header_value_error_to_magnus};

    macro_rules! value {
        ($json:expr, $key:ident, $method:ident) => {
            $json.get(stringify!($key)).and_then(|v| v.$method())
        };
    }

    pub fn parse_tls(json: &Value, builder: EmulationBuilder) -> Result<EmulationBuilder, Error> {
        let Some(tls) = value!(json, tls, as_object) else {
            return Ok(builder);
        };

        let mut tls_builder = TlsOptions::builder();

        // parse ciphers
        if let Some(ciphers) = value!(tls, ciphers, as_array) {
            let ciphers_list = ciphers
                .iter()
                .flat_map(|v| v.as_str())
                .filter(|s| !s.is_empty() && !s.starts_with(stringify!(TLS_GREASE)))
                .into_iter()
                .collect::<Vec<_>>()
                .join(",");
            tls_builder = tls_builder.cipher_list(ciphers_list)
        }

        Ok(builder.tls_options(tls_builder.build()))
    }
    pub fn parse_http2(
        json: &Value,
        mut builder: EmulationBuilder,
    ) -> Result<EmulationBuilder, Error> {
        let Some(sent_frames) = value!(json, http2, as_object)
            .and_then(|http2_frames| value!(http2_frames, sent_frames, as_array))
        else {
            return Ok(builder);
        };

        let mut http2_builder = Http2Options::builder();

        // parse settings frame
        if let Some(settings) = sent_frames
            .iter()
            .find(|frame| value!(frame, frame_type, as_str) == Some(stringify!(SETTINGS)))
            .and_then(|frame| value!(frame, settings, as_array))
        {
            let mut settings_order = SettingsOrder::builder();
            for setting in settings.iter().filter_map(|v| v.as_str()) {
                let mut parts = setting.split('=');
                if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                    let value = value.trim();
                    match name.trim() {
                        "HEADER_TABLE_SIZE" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.header_table_size(value);
                                settings_order = settings_order.push(SettingId::HeaderTableSize);
                            }
                        }
                        "ENABLE_PUSH" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.enable_push(value != 0);
                                settings_order = settings_order.push(SettingId::EnablePush);
                            }
                        }
                        "INITIAL_WINDOW_SIZE" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.initial_window_size(value);
                                settings_order = settings_order.push(SettingId::InitialWindowSize);
                            }
                        }
                        "MAX_FRAME_SIZE" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.max_frame_size(value);
                                settings_order = settings_order.push(SettingId::MaxFrameSize);
                            }
                        }
                        "MAX_HEADER_LIST_SIZE" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.max_header_list_size(value);
                                settings_order = settings_order.push(SettingId::MaxHeaderListSize);
                            }
                        }
                        "MAX_CONCURRENT_STREAMS" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.max_concurrent_streams(value);
                                settings_order =
                                    settings_order.push(SettingId::MaxConcurrentStreams);
                            }
                        }
                        "UNKNOWN_SETTING_8" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.enable_connect_protocol(value != 0);
                                settings_order =
                                    settings_order.push(SettingId::EnableConnectProtocol);
                            }
                        }
                        "NO_RFC7540_PRIORITIES" => {
                            if let Ok(value) = value.parse::<u32>() {
                                http2_builder = http2_builder.no_rfc7540_priorities(value != 0);
                                settings_order =
                                    settings_order.push(SettingId::NoRfc7540Priorities);
                            }
                        }
                        _ => {}
                    }
                }
            }

            http2_builder = http2_builder.settings_order(settings_order.build());
        }

        // parse window update frame
        if let Some(window_update) = sent_frames
            .iter()
            .find(|frame| value!(frame, frame_type, as_str) == Some(stringify!(WINDOW_UPDATE)))
            .and_then(|frame| value!(frame, increment, as_u64))
        {
            http2_builder =
                http2_builder.initial_connection_window_size((window_update + 65535) as u32);
        }

        // parse headers frame
        if let Some(headers_frame) = sent_frames
            .iter()
            .find(|frame| value!(frame, frame_type, as_str) == Some(stringify!(HEADERS)))
        {
            // parse initial stream id
            if let Some(init_stream_id) =
                value!(headers_frame, stream_id, as_u64).filter(|v| *v != 0)
            {
                http2_builder = http2_builder.initial_stream_id(init_stream_id as u32);
            }

            // parse headers
            if let Some(headers) = value!(headers_frame, headers, as_array) {
                let mut pseudo_builder = PseudoOrder::builder();
                let mut headers_map = HeaderMap::with_capacity(headers.len());

                for (name, value) in headers
                    .iter()
                    .filter_map(|h| h.as_str())
                    .map(|h| h.split_once(": "))
                    .flatten()
                {
                    match name {
                        ":method" => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Method);
                        }
                        ":path" => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Path);
                        }
                        ":scheme" => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Scheme);
                        }
                        ":authority" => {
                            pseudo_builder = pseudo_builder.push(PseudoId::Authority);
                        }
                        _ => {
                            let header_name =
                                HeaderName::from_str(name).map_err(header_name_error_to_magnus)?;
                            let header_value = HeaderValue::from_str(value)
                                .map_err(header_value_error_to_magnus)?;
                            headers_map.insert(header_name, header_value);
                        }
                    }
                }

                http2_builder = http2_builder.headers_pseudo_order(pseudo_builder.build());
                builder = builder.headers(headers_map);
            };

            // parse header priority
            if let Some(priority) = value!(headers_frame, priority, as_object) {
                if let (Some(depends_on), Some(weight), Some(exclusive)) = (
                    value!(priority, depends_on, as_u64),
                    value!(priority, weight, as_u64),
                    value!(priority, exclusive, as_u64),
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

        Ok(builder.http2_options(http2_builder.build()))
    }
}
