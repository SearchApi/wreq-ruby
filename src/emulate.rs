use magnus::{
    Error, Module, Object, RHash, RModule, Ruby, TryConvert, Value, function, method,
    typed_data::{Inspect, Obj},
};

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
pub struct Emulation(pub wreq_util::Emulation);

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
        let mut device = None;
        let mut os = None;
        let mut http2 = None;
        let mut headers = None;

        if let Some(hash) = args.first().and_then(|v| RHash::from_value(*v)) {
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(profile))) {
                device = Some(Obj::<Profile>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(platform))) {
                os = Some(Obj::<Platform>::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(http2))) {
                http2 = Some(bool::try_convert(v)?);
            }
            if let Some(v) = hash.get(ruby.to_symbol(stringify!(headers))) {
                headers = Some(bool::try_convert(v)?);
            }
        }

        let emulation = wreq_util::Emulation::builder()
            .profile(device.map(|obj| obj.into_ffi()).unwrap_or_default())
            .platform(os.map(|os| os.into_ffi()).unwrap_or_default())
            .http2(http2.unwrap_or(true))
            .headers(headers.unwrap_or(true))
            .build();

        Ok(Self(emulation))
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
    Ok(())
}
