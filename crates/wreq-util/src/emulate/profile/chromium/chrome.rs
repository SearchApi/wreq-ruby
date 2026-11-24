use super::super::{Emulation, HeaderMap, Platform, build_standard_emulation};
use super::{
    header::{
        navigation_headers, navigation_headers_with_jxl,
        navigation_headers_with_language_and_zstd_only, navigation_headers_with_zstd,
        navigation_headers_with_zstd_and_priority,
    },
    http2, tls,
};

chromium_profile!(
    v100,
    tls: tls::classic_groups,
    http2: http2::Preset::StreamLimit,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Android,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            Windows,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 15_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/100.0.4896.85 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v101,
    transport: v100::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Android,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            Windows,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A;Brand";v="99", "Chromium";v="101", "Google Chrome";v="101""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/101.0.4951.58 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v104,
    transport: v100::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="104", " Not A;Brand";v="99", "Google Chrome";v="104""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/104.0.5112.99 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v105,
    tls: tls::ech_grease,
    http2: http2::Preset::StreamLimit,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (X11; U; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="105", "Not)A;Brand";v="8", "Chromium";v="105""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/105.0.5195.100 Mobile/15E148 Safari/604.1"
        )
    ]
);

// This preset explicitly disables HTTP/2 push; earlier presets leave ENABLE_PUSH unset.
chromium_profile!(
    v106,
    tls: tls::permuted_extensions,
    http2: http2::Preset::StreamLimitNoPush,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/106.0.5249.92 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v107,
    transport: v106::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Google Chrome";v="107", "Chromium";v="107", "Not=A?Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="107", "Chromium";v="107", "Not=A?Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="107", "Chromium";v="107", "Not=A?Brand";v="24""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="107", "Chromium";v="107", "Not=A?Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="107", "Chromium";v="107", "Not=A?Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/107.0.5304.66 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v108,
    transport: v106::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/108.0.5359.112 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v109,
    transport: v106::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/109.0.5414.112 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Chrome 110 uses the earlier transport, including no explicit HTTP/2 ENABLE_PUSH.
chromium_profile!(
    v110,
    transport: v100::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.104 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v114,
    transport: v106::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        )
    ]
);

chromium_profile!(
    v116,
    tls: tls::permuted_extensions_with_ech_grease,
    http2: http2::Preset::StreamLimitNoPush,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="116", "Not)A;Brand";v="24", "Google Chrome";v="116""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/116.0.5845.103 Mobile/15E148 Safari/604.1"
        )
    ]
);

// The HTTP/2 stream limit is omitted here, while ENABLE_PUSH stays explicitly disabled.
chromium_profile!(
    v117,
    tls: tls::resumption,
    http2: http2::Preset::NoPush,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="117", "Not;A=Brand";v="8", "Chromium";v="117""#,
            "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
        )
    ]
);

chromium_profile!(
    v120,
    transport: v117::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not_A Brand";v="8", "Chromium";v="120", "Google Chrome";v="120""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/120.0.6099.119 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v123,
    transport: v117::build_emulation,
    headers: navigation_headers_with_zstd,
    [
        (
            MacOS,
            r#""Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        )
    ]
);

// Chrome 118 keeps the no-push HTTP/2 settings but leaves TLS resumption off.
chromium_profile!(
    v118,
    tls: tls::permuted_extensions_with_ech_grease,
    http2: http2::Preset::NoPush,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="118", "Google Chrome";v="118", "Not=A?Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/118.0.5993.92 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v119,
    transport: v118::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/119.0.6045.109 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v124,
    tls: tls::resumption_with_kyber,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_zstd,
    [
        (
            MacOS,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="124", "Google Chrome";v="124", "Not-A.Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/124.0.6312.52 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v126,
    transport: v124::build_emulation,
    headers: navigation_headers_with_zstd,
    [
        (
            MacOS,
            r#""Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/126.0.6478.153 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v127,
    transport: v124::build_emulation,
    headers: navigation_headers_with_zstd,
    [
        (
            MacOS,
            r#""Not)A;Brand";v="99", "Google Chrome";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not)A;Brand";v="99", "Google Chrome";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not)A;Brand";v="99", "Google Chrome";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Not)A;Brand";v="99", "Google Chrome";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not)A;Brand";v="99", "Google Chrome";v="127", "Chromium";v="127""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/127.0.6533.77 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Chrome 128 uses the earlier navigation headers despite sharing Chrome 124's transport.
chromium_profile!(
    v128,
    transport: v124::build_emulation,
    headers: navigation_headers,
    [
        (
            MacOS,
            r#""Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/128.0.6613.98 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v129,
    transport: v124::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/129.0.6668.46 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v130,
    transport: v124::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""#,
            "Mozilla/5.0 (iPad; CPU OS 18_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/130.0.6723.90 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v131,
    tls: tls::resumption_with_mlkem,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Linux: Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/131.0.6778.134 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v132,
    tls: tls::mlkem_new_alps,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Google Chrome";v="132""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/132.0.6834.78 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v133,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/133.0.6943.33 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v134,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/134.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v135,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="135", "Not-A.Brand";v="8", "Chromium";v="135""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/135.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v136,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="136", "Google Chrome";v="136", "Not.A/Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="136", "Google Chrome";v="136", "Not.A/Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="136", "Google Chrome";v="136", "Not.A/Brand";v="99""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="136", "Google Chrome";v="136", "Not.A/Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="136", "Google Chrome";v="136", "Not.A/Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/136.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v137,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/137.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v138,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/138.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v139,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not;A=Brand";v="99", "Google Chrome";v="139", "Chromium";v="139""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not;A=Brand";v="99", "Google Chrome";v="139", "Chromium";v="139""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not;A=Brand";v="99", "Google Chrome";v="139", "Chromium";v="139""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not;A=Brand";v="99", "Google Chrome";v="139", "Chromium";v="139""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not;A=Brand";v="99", "Google Chrome";v="139", "Chromium";v="139""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/139.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v140,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/140.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v141,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/141.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v142,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/142.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v143,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/143.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v144,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not(A:Brand";v="8", "Chromium";v="144", "Google Chrome";v="144""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not(A:Brand";v="8", "Chromium";v="144", "Google Chrome";v="144""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not(A:Brand";v="8", "Chromium";v="144", "Google Chrome";v="144""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not(A:Brand";v="8", "Chromium";v="144", "Google Chrome";v="144""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not(A:Brand";v="8", "Chromium";v="144", "Google Chrome";v="144""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/144.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v145,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Google Chrome";v="145", "Chromium";v="145""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not:A-Brand";v="99", "Google Chrome";v="145", "Chromium";v="145""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not:A-Brand";v="99", "Google Chrome";v="145", "Chromium";v="145""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Google Chrome";v="145", "Chromium";v="145""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not:A-Brand";v="99", "Google Chrome";v="145", "Chromium";v="145""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/145.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v146,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.7680.164 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/146.0.7680.164 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v147,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.7712.122 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/147.0.7712.122 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v148,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="148", "Google Chrome";v="148", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="148", "Google Chrome";v="148", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="148", "Google Chrome";v="148", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="148", "Google Chrome";v="148", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="148", "Google Chrome";v="148", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/148.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v149,
    transport: v132::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/149.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Add ML-DSA to the TLS signature list while leaving signature GREASE unset.
chromium_profile!(
    v150,
    tls: tls::mlkem_new_alps_with_mldsa,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Google Chrome";v="150""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Google Chrome";v="150""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Google Chrome";v="150""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Google Chrome";v="150""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Google Chrome";v="150""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/150.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

chromium_profile!(
    v151,
    transport: v150::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/151.0.7922.43 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Shuffle Trust Anchor IDs per native configuration, and send Accept-Language earlier outside Linux.
chromium_profile!(
    v152,
    tls: tls::shuffled_trust_anchor_ids,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_language_and_zstd_only,
    [
        (
            MacOS,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Google Chrome";v="152""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Google Chrome";v="152""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Google Chrome";v="152""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Google Chrome";v="152""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Google Chrome";v="152""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/152.0.0.0 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Keep the shuffled Trust Anchor IDs, but use the regular zstd/priority headers on all platforms.
chromium_profile!(
    v153,
    transport: v152::build_emulation,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Google Chrome";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/153.0.8010.24 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Sort the raw Trust Anchor IDs before encoding them, instead of shuffling.
chromium_profile!(
    v154,
    tls: tls::sorted_trust_anchor_ids,
    http2: http2::Preset::NoPush,
    headers: navigation_headers_with_zstd_and_priority,
    [
        (
            MacOS,
            r#""Chromium";v="154", "Google Chrome";v="154", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="154", "Google Chrome";v="154", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="154", "Google Chrome";v="154", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="154", "Google Chrome";v="154", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="154", "Google Chrome";v="154", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/154.0.8037.41 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Advertise JPEG XL outside iOS while retaining Chrome 154's TLS and HTTP/2 settings.
// The UA-CH permutation and GREASE brand are seeded by major version 155.
// https://chromium.googlesource.com/chromium/src/+/cfaadc5a132d78e1828635aa8405a499f3e14864/components/embedder_support/user_agent_utils.cc
chromium_profile!(
    v155,
    transport: v154::build_emulation,
    headers: navigation_headers_with_jxl,
    [
        (
            MacOS,
            r#""Google Chrome";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Google Chrome";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Google Chrome";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Google Chrome";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Google Chrome";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/155.0.8059.37 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Chrome 156 retains Chrome 155's TLS/HTTP2 and Accept configuration.
// https://chromium.googlesource.com/chromium/src/+/29eb03535d5f2f4f899b95f22a92413ffde5284e/components/embedder_support/user_agent_utils.cc
chromium_profile!(
    v156,
    transport: v154::build_emulation,
    headers: navigation_headers_with_jxl,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="8", "Chromium";v="156", "Google Chrome";v="156""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not:A-Brand";v="8", "Chromium";v="156", "Google Chrome";v="156""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not:A-Brand";v="8", "Chromium";v="156", "Google Chrome";v="156""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="8", "Chromium";v="156", "Google Chrome";v="156""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not:A-Brand";v="8", "Chromium";v="156", "Google Chrome";v="156""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/156.0.8078.54 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Chrome 157 retains Chrome 156's TLS/HTTP2 and Accept configuration.
// Its UA-CH order and GREASE brand are seeded by major version 157.
// https://chromium.googlesource.com/chromium/src/+/653674df293788cf4932c40d914e9e15b41a2d31/components/embedder_support/user_agent_utils.cc
chromium_profile!(
    v157,
    transport: v154::build_emulation,
    headers: navigation_headers_with_jxl,
    [
        (
            MacOS,
            r#""Not-A.Brand";v="99", "Google Chrome";v="157", "Chromium";v="157""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Not-A.Brand";v="99", "Google Chrome";v="157", "Chromium";v="157""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Not-A.Brand";v="99", "Google Chrome";v="157", "Chromium";v="157""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Not-A.Brand";v="99", "Google Chrome";v="157", "Chromium";v="157""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Not-A.Brand";v="99", "Google Chrome";v="157", "Chromium";v="157""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/157.0.8103.37 Mobile/15E148 Safari/604.1"
        )
    ]
);

// Chrome 158 retains the Chrome 157 TLS, HTTP/2 and Accept presets.
// https://chromium.googlesource.com/chromium/src/+/b8c2be851f62f6a394ec57955d7f31335d7cb6d4/components/embedder_support/user_agent_utils.cc
chromium_profile!(
    v158,
    transport: v154::build_emulation,
    headers: navigation_headers_with_jxl,
    [
        (
            MacOS,
            r#""Chromium";v="158", "Not.A/Brand";v="24", "Google Chrome";v="158""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36"
        ),
        (
            Linux,
            r#""Chromium";v="158", "Not.A/Brand";v="24", "Google Chrome";v="158""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36"
        ),
        (
            Android,
            r#""Chromium";v="158", "Not.A/Brand";v="24", "Google Chrome";v="158""#,
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Mobile Safari/537.36"
        ),
        (
            Windows,
            r#""Chromium";v="158", "Not.A/Brand";v="24", "Google Chrome";v="158""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36"
        ),
        (
            IOS,
            r#""Chromium";v="158", "Not.A/Brand";v="24", "Google Chrome";v="158""#,
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/158.0.8127.35 Mobile/15E148 Safari/604.1"
        )
    ]
);
