use super::super::{Emulation, HeaderMap, Platform, build_standard_emulation};
use super::{header, http2, tls};

macro_rules! opera_desktop_profile {
    (
        $name:ident,
        opera: $opera:literal,
        chromium: $chromium:literal,
        brands: $brands:literal,
        headers: $headers:path,
        $($transport:tt)+
    ) => {
        chromium_profile!(
            $name,
            $($transport)+,
            headers: $headers,
            brands: $brands,
            [
                (MacOS, chromium_desktop_ua!("Macintosh; Intel Mac OS X 10_15_7", $chromium, " OPR/", $opera, ".0.0.0")),
                (Windows, chromium_desktop_ua!("Windows NT 10.0; Win64; x64", $chromium, " OPR/", $opera, ".0.0.0")),
                (Linux, chromium_desktop_ua!("X11; Linux x86_64", $chromium, " OPR/", $opera, ".0.0.0"))
            ]
        );
    };
}

opera_desktop_profile!(
    opera116,
    opera: "116",
    chromium: "131",
    brands: r#""Opera";v="116", "Chromium";v="131", "Not_A Brand";v="24""#,
    headers: header::Preset::ZstdWithPriority,
    tls: tls::Preset::ResumptionWithMlkem,
    http2: http2::Preset::NoPush
);

opera_desktop_profile!(
    opera117,
    opera: "117",
    chromium: "132",
    brands: r#""Not A(Brand";v="8", "Chromium";v="132", "Opera";v="117""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera118,
    opera: "118",
    chromium: "133",
    brands: r#""Not(A:Brand";v="99", "Opera";v="118", "Chromium";v="133""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera119,
    opera: "119",
    chromium: "134",
    brands: r#""Chromium";v="134", "Not:A-Brand";v="24", "Opera";v="119""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera120,
    opera: "120",
    chromium: "135",
    brands: r#""Chromium";v="135", "Not:A-Brand";v="24", "Opera";v="120""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera121,
    opera: "121",
    chromium: "137",
    brands: r#""Opera";v="121", "Chromium";v="137", "Not/A)Brand";v="24""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera122,
    opera: "122",
    chromium: "138",
    brands: r#""Chromium";v="138", "Not=A?Brand";v="24", "Opera";v="122""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera123,
    opera: "123",
    chromium: "139",
    brands: r#""Chromium";v="139", "Not=A?Brand";v="24", "Opera";v="123""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera124,
    opera: "124",
    chromium: "140",
    brands: r#""Chromium";v="140", "Not=A?Brand";v="24", "Opera";v="124""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera125,
    opera: "125",
    chromium: "141",
    brands: r#""Opera";v="125", "Not?A_Brand";v="8", "Chromium";v="141""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera126,
    opera: "126",
    chromium: "142",
    brands: r#""Not:A-Brand";v="99", "Opera";v="126", "Chromium";v="142""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera127,
    opera: "127",
    chromium: "143",
    brands: r#""Not:A-Brand";v="99", "Opera";v="127", "Chromium";v="143""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera128,
    opera: "128",
    chromium: "144",
    brands: r#""Not:A-Brand";v="99", "Opera";v="128", "Chromium";v="144""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera129,
    opera: "129",
    chromium: "145",
    brands: r#""Not:A-Brand";v="99", "Opera";v="129", "Chromium";v="145""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera130,
    opera: "130",
    chromium: "146",
    brands: r#""Not:A-Brand";v="99", "Opera";v="130", "Chromium";v="146""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

opera_desktop_profile!(
    opera131,
    opera: "131",
    chromium: "147",
    brands: r#""Opera";v="131", "Not.A/Brand";v="8", "Chromium";v="147""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera116::build_emulation
);

// Reuse Chromium's TLS defaults with Opera's existing HTTP/2 settings.
// Brand hints use Chromium's milestone seed with Opera as the product brand.
opera_desktop_profile!(
    opera132,
    opera: "132",
    chromium: "148",
    brands: r#""Chromium";v="148", "Opera";v="132", "Not/A)Brand";v="99""#,
    headers: header::Preset::ZstdWithPriority,
    tls: tls::Preset::MlkemNewAlps,
    http2: http2::Preset::NoPush
);

opera_desktop_profile!(
    opera133,
    opera: "133",
    chromium: "149",
    brands: r#""Opera";v="133", "Chromium";v="149", "Not)A;Brand";v="24""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera132::build_emulation
);

opera_desktop_profile!(
    opera134,
    opera: "134",
    chromium: "150",
    brands: r#""Not;A=Brand";v="8", "Chromium";v="150", "Opera";v="134""#,
    headers: header::Preset::ZstdWithPriority,
    tls: tls::Preset::MlkemNewAlpsWithMldsa,
    http2: http2::Preset::NoPush
);

opera_desktop_profile!(
    opera135,
    opera: "135",
    chromium: "151",
    brands: r#""Not=A?Brand";v="99", "Opera";v="135", "Chromium";v="151""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera134::build_emulation
);

// Use Chromium's shuffled Trust Anchor IDs with Opera's existing HTTP/2 settings.
opera_desktop_profile!(
    opera136,
    opera: "136",
    chromium: "152",
    brands: r#""Chromium";v="152", "Not?A_Brand";v="24", "Opera";v="136""#,
    headers: header::Preset::ZstdWithPriority,
    tls: tls::Preset::ShuffledTrustAnchorIds,
    http2: http2::Preset::NoPush
);

opera_desktop_profile!(
    opera137,
    opera: "137",
    chromium: "153",
    brands: r#""Opera";v="137", "Not_A Brand";v="8", "Chromium";v="153""#,
    headers: header::Preset::ZstdWithPriority,
    transport: opera136::build_emulation
);

// Switch from shuffled to sorted Trust Anchor IDs.
opera_desktop_profile!(
    opera138,
    opera: "138",
    chromium: "154",
    brands: r#""Chromium";v="154", "Opera";v="138", "Not A(Brand";v="99""#,
    headers: header::Preset::ZstdWithPriority,
    tls: tls::Preset::SortedTrustAnchorIds,
    http2: http2::Preset::NoPush
);

// Reuse Chromium's JPEG XL navigation headers without changing the Opera transport.
opera_desktop_profile!(
    opera139,
    opera: "139",
    chromium: "155",
    brands: r#""Opera";v="139", "Chromium";v="155", "Not(A:Brand";v="24""#,
    headers: header::Preset::Jxl,
    transport: opera138::build_emulation
);

opera_desktop_profile!(
    opera140,
    opera: "140",
    chromium: "157",
    brands: r#""Not-A.Brand";v="99", "Opera";v="140", "Chromium";v="157""#,
    headers: header::Preset::Jxl,
    transport: opera138::build_emulation
);
