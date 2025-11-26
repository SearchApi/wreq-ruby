#![allow(clippy::wrong_self_convention)]
use magnus::{
    Error, Module, Object, RHash, RModule, Ruby, TryConvert, Value, function, method,
    typed_data::{Inspect, Obj},
};

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
    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Edge134,
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
    Safari18,
    SafariIPad18,
    Safari18_2,
    Safari18_3,
    Safari18_3_1,
    SafariIos18_1_1,
    Safari18_5,
    Safari26,
    SafariIos26,
    SafariIPad26,
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
pub struct Emulation(pub wreq_util::EmulationOption);

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
            if let Some(v) = hash.get(ruby.to_symbol("device")) {
                device = Some(Obj::<EmulationDevice>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol("os")) {
                os = Some(Obj::<EmulationOS>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol("skip_http2")) {
                skip_http2 = Some(bool::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol("skip_headers")) {
                skip_headers = Some(bool::try_convert(v)?);
            }
        }

        let emulation = wreq_util::EmulationOption::builder()
            .emulation(device.map(|obj| obj.into_ffi()).unwrap_or_default())
            .emulation_os(os.map(|os| os.into_ffi()).unwrap_or_default())
            .skip_http2(skip_http2.unwrap_or(false))
            .skip_headers(skip_headers.unwrap_or(false))
            .build();

        Ok(Self(emulation))
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // EmulationDevice enum binding
    let emulation_class = gem_module.define_class("EmulationDevice", ruby.class_object())?;
    emulation_class.define_method("to_s", method!(EmulationDevice::to_s, 0))?;
    emulation_class.const_set("Chrome100", EmulationDevice::Chrome100)?;
    emulation_class.const_set("Chrome101", EmulationDevice::Chrome101)?;
    emulation_class.const_set("Chrome104", EmulationDevice::Chrome104)?;
    emulation_class.const_set("Chrome105", EmulationDevice::Chrome105)?;
    emulation_class.const_set("Chrome106", EmulationDevice::Chrome106)?;
    emulation_class.const_set("Chrome107", EmulationDevice::Chrome107)?;
    emulation_class.const_set("Chrome108", EmulationDevice::Chrome108)?;
    emulation_class.const_set("Chrome109", EmulationDevice::Chrome109)?;
    emulation_class.const_set("Chrome110", EmulationDevice::Chrome110)?;
    emulation_class.const_set("Chrome114", EmulationDevice::Chrome114)?;
    emulation_class.const_set("Chrome116", EmulationDevice::Chrome116)?;
    emulation_class.const_set("Chrome117", EmulationDevice::Chrome117)?;
    emulation_class.const_set("Chrome118", EmulationDevice::Chrome118)?;
    emulation_class.const_set("Chrome119", EmulationDevice::Chrome119)?;
    emulation_class.const_set("Chrome120", EmulationDevice::Chrome120)?;
    emulation_class.const_set("Chrome123", EmulationDevice::Chrome123)?;
    emulation_class.const_set("Chrome124", EmulationDevice::Chrome124)?;
    emulation_class.const_set("Chrome126", EmulationDevice::Chrome126)?;
    emulation_class.const_set("Chrome127", EmulationDevice::Chrome127)?;
    emulation_class.const_set("Chrome128", EmulationDevice::Chrome128)?;
    emulation_class.const_set("Chrome129", EmulationDevice::Chrome129)?;
    emulation_class.const_set("Chrome130", EmulationDevice::Chrome130)?;
    emulation_class.const_set("Chrome131", EmulationDevice::Chrome131)?;
    emulation_class.const_set("Chrome132", EmulationDevice::Chrome132)?;
    emulation_class.const_set("Chrome133", EmulationDevice::Chrome133)?;
    emulation_class.const_set("Chrome134", EmulationDevice::Chrome134)?;
    emulation_class.const_set("Chrome135", EmulationDevice::Chrome135)?;
    emulation_class.const_set("Chrome136", EmulationDevice::Chrome136)?;
    emulation_class.const_set("Chrome137", EmulationDevice::Chrome137)?;
    emulation_class.const_set("Chrome138", EmulationDevice::Chrome138)?;
    emulation_class.const_set("Chrome139", EmulationDevice::Chrome139)?;
    emulation_class.const_set("Chrome140", EmulationDevice::Chrome140)?;
    emulation_class.const_set("Chrome141", EmulationDevice::Chrome141)?;
    emulation_class.const_set("Chrome142", EmulationDevice::Chrome142)?;
    emulation_class.const_set("Edge101", EmulationDevice::Edge101)?;
    emulation_class.const_set("Edge122", EmulationDevice::Edge122)?;
    emulation_class.const_set("Edge127", EmulationDevice::Edge127)?;
    emulation_class.const_set("Edge131", EmulationDevice::Edge131)?;
    emulation_class.const_set("Edge134", EmulationDevice::Edge134)?;
    emulation_class.const_set("Firefox109", EmulationDevice::Firefox109)?;
    emulation_class.const_set("Firefox117", EmulationDevice::Firefox117)?;
    emulation_class.const_set("Firefox128", EmulationDevice::Firefox128)?;
    emulation_class.const_set("Firefox133", EmulationDevice::Firefox133)?;
    emulation_class.const_set("Firefox135", EmulationDevice::Firefox135)?;
    emulation_class.const_set("FirefoxPrivate135", EmulationDevice::FirefoxPrivate135)?;
    emulation_class.const_set("FirefoxAndroid135", EmulationDevice::FirefoxAndroid135)?;
    emulation_class.const_set("Firefox136", EmulationDevice::Firefox136)?;
    emulation_class.const_set("FirefoxPrivate136", EmulationDevice::FirefoxPrivate136)?;
    emulation_class.const_set("Firefox139", EmulationDevice::Firefox139)?;
    emulation_class.const_set("Firefox142", EmulationDevice::Firefox142)?;
    emulation_class.const_set("Firefox143", EmulationDevice::Firefox143)?;
    emulation_class.const_set("SafariIos17_2", EmulationDevice::SafariIos17_2)?;
    emulation_class.const_set("SafariIos17_4_1", EmulationDevice::SafariIos17_4_1)?;
    emulation_class.const_set("SafariIos16_5", EmulationDevice::SafariIos16_5)?;
    emulation_class.const_set("Safari15_3", EmulationDevice::Safari15_3)?;
    emulation_class.const_set("Safari15_5", EmulationDevice::Safari15_5)?;
    emulation_class.const_set("Safari15_6_1", EmulationDevice::Safari15_6_1)?;
    emulation_class.const_set("Safari16", EmulationDevice::Safari16)?;
    emulation_class.const_set("Safari16_5", EmulationDevice::Safari16_5)?;
    emulation_class.const_set("Safari17_0", EmulationDevice::Safari17_0)?;
    emulation_class.const_set("Safari17_2_1", EmulationDevice::Safari17_2_1)?;
    emulation_class.const_set("Safari17_4_1", EmulationDevice::Safari17_4_1)?;
    emulation_class.const_set("Safari17_5", EmulationDevice::Safari17_5)?;
    emulation_class.const_set("Safari18", EmulationDevice::Safari18)?;
    emulation_class.const_set("SafariIPad18", EmulationDevice::SafariIPad18)?;
    emulation_class.const_set("Safari18_2", EmulationDevice::Safari18_2)?;
    emulation_class.const_set("Safari18_3", EmulationDevice::Safari18_3)?;
    emulation_class.const_set("Safari18_3_1", EmulationDevice::Safari18_3_1)?;
    emulation_class.const_set("SafariIos18_1_1", EmulationDevice::SafariIos18_1_1)?;
    emulation_class.const_set("Safari18_5", EmulationDevice::Safari18_5)?;
    emulation_class.const_set("Safari26", EmulationDevice::Safari26)?;
    emulation_class.const_set("SafariIos26", EmulationDevice::SafariIos26)?;
    emulation_class.const_set("SafariIPad26", EmulationDevice::SafariIPad26)?;
    emulation_class.const_set("OkHttp3_13", EmulationDevice::OkHttp3_13)?;
    emulation_class.const_set("OkHttp3_14", EmulationDevice::OkHttp3_14)?;
    emulation_class.const_set("OkHttp4_9", EmulationDevice::OkHttp4_9)?;
    emulation_class.const_set("OkHttp4_10", EmulationDevice::OkHttp4_10)?;
    emulation_class.const_set("OkHttp4_12", EmulationDevice::OkHttp4_12)?;
    emulation_class.const_set("OkHttp5", EmulationDevice::OkHttp5)?;
    emulation_class.const_set("Opera116", EmulationDevice::Opera116)?;
    emulation_class.const_set("Opera117", EmulationDevice::Opera117)?;
    emulation_class.const_set("Opera118", EmulationDevice::Opera118)?;
    emulation_class.const_set("Opera119", EmulationDevice::Opera119)?;

    // EmulationOS enum binding
    let emulation_os_class = gem_module.define_class("EmulationOS", ruby.class_object())?;
    emulation_os_class.define_method("to_s", method!(EmulationOS::to_s, 0))?;
    emulation_os_class.const_set("Windows", EmulationOS::Windows)?;
    emulation_os_class.const_set("MacOS", EmulationOS::MacOS)?;
    emulation_os_class.const_set("Linux", EmulationOS::Linux)?;
    emulation_os_class.const_set("Android", EmulationOS::Android)?;
    emulation_os_class.const_set("IOS", EmulationOS::IOS)?;

    // Emulation class binding
    let emulation_option_class = gem_module.define_class("Emulation", ruby.class_object())?;
    emulation_option_class.define_singleton_method("new", function!(Emulation::new, -1))?;
    Ok(())
}
