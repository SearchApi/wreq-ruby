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
    Chrome148,
    Chrome149,

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

        Ok(Self(emulation))
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // Profile enum binding
    let profile = gem_module.define_class("Profile", ruby.class_object())?;
    profile.define_method("to_s", method!(Profile::to_s, 0))?;
    Profile::define_constants(profile)?;

    // Platform enum binding
    let platform = gem_module.define_class("Platform", ruby.class_object())?;
    platform.define_method("to_s", method!(Platform::to_s, 0))?;
    Platform::define_constants(platform)?;

    // Emulation class binding
    let emulation = gem_module.define_class("Emulation", ruby.class_object())?;
    emulation.define_singleton_method("new", function!(Emulation::new, -1))?;
    Ok(())
}
