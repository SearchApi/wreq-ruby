use super::super::{Emulation, HeaderMap, Platform, build_standard_emulation};
use super::{
    chrome::{v100, v117, v124, v131, v132, v150},
    header, http2, tls,
};

macro_rules! edge_profile {
    (
        $name:ident,
        transport: $transport:path,
        brands: $brands:literal,
        $platforms:tt $(,)?
    ) => {
        edge_profile!(
            $name,
            transport: $transport,
            headers: header::Preset::ZstdWithPriority,
            brands: $brands,
            $platforms
        );
    };
    (
        $name:ident,
        transport: $transport:path,
        headers: $headers:path,
        brands: $brands:literal,
        $platforms:tt $(,)?
    ) => {
        standard_mod_generator!($name, $transport, |emulation: &Emulation| {
            platform_headers!(emulation, $headers, $brands, $platforms)
        });
    };
    (
        $name:ident,
        tls: $tls:path,
        http2: $http2:path,
        brands: $brands:literal,
        $platforms:tt $(,)?
    ) => {
        standard_mod_generator!($name, ($tls).build(), ($http2).build(), |emulation: &Emulation| {
            platform_headers!(emulation, header::Preset::ZstdWithPriority, $brands, $platforms)
        });
    };
}

edge_profile!(
    edge101,
    transport: v100::build_emulation,
    headers: header::Preset::Legacy,
    brands: r#""Not A;Brand";v="99", "Chromium";v="101", "Microsoft Edge";v="101""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.47"),
        (Android, "Mozilla/5.0 (Linux; Android 10; ONEPLUS A6003) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.41 Safari/537.36 Edg/101.0.1210.31"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.53"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.53"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/101.0.4951.64 Mobile Safari/537.36 Edg/101.0.1210.53")
    ]
);

edge_profile!(
    edge122,
    transport: v117::build_emulation,
    headers: header::Preset::Legacy,
    brands: r#""Chromium";v="122", "Not(A:Brand";v="24", "Microsoft Edge";v="122""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; Pixel 3 XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6268.219 Safari/537.36 Edg/122.0.2238.82"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0")
    ]
);

edge_profile!(
    edge127,
    transport: v124::build_emulation,
    brands: r#""Not)A;Brand";v="99", "Microsoft Edge";v="127", "Chromium";v="127""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.6332.205 Safari/537.36 Edg/127.0.2322.67"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0")
    ]
);

edge_profile!(
    edge131,
    transport: v131::build_emulation,
    brands: r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; HD1913) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.200 Mobile Safari/537.36 EdgA/131.0.2903.87"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")
    ]
);

// Unlike Chrome 134, this Edge preset retains the earlier ALPS codepoint.
edge_profile!(
    edge134,
    transport: v131::build_emulation,
    brands: r#""Chromium";v="134", "Not:A-Brand";v="24", "Microsoft Edge";v="134"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/134.0.0.0")
    ]
);

edge_profile!(
    edge135,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="135", "Not:A-Brand";v="24", "Microsoft Edge";v="135"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/135.0.0.0")
    ]
);

edge_profile!(
    edge136,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="136", "Not:A-Brand";v="24", "Microsoft Edge";v="136"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/136.0.0.0")
    ]
);

edge_profile!(
    edge137,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="137", "Not:A-Brand";v="24", "Microsoft Edge";v="137"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/137.0.0.0")
    ]
);

edge_profile!(
    edge138,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="138", "Not=A?Brand";v="24", "Microsoft Edge";v="138"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/138.0.0.0")
    ]
);

edge_profile!(
    edge139,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="139", "Not=A?Brand";v="24", "Microsoft Edge";v="139"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/139.0.0.0")
    ]
);

edge_profile!(
    edge140,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="140", "Not=A?Brand";v="24", "Microsoft Edge";v="140"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/140.0.0.0")
    ]
);

edge_profile!(
    edge141,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="141", "Not=A?Brand";v="24", "Microsoft Edge";v="141"#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/141.0.0.0")
    ]
);

edge_profile!(
    edge142,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="142", "Microsoft Edge";v="142", "Not_A Brand";v="99""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36 Edg/142.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0")
    ]
);

edge_profile!(
    edge143,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="143", "Microsoft Edge";v="143", "Not_A Brand";v="99""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Mobile Safari/537.36 Edg/143.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.0.0")
    ]
);

edge_profile!(
    edge144,
    transport: v132::build_emulation,
    brands: r#""Not(A:Brand";v="8", "Chromium";v="144", "Microsoft Edge";v="144""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Mobile Safari/537.36 Edg/144.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0")
    ]
);

edge_profile!(
    edge145,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="145", "Not;A=Brand";v="24", "Microsoft Edge";v="145""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Mobile Safari/537.36 Edg/145.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0")
    ]
);

edge_profile!(
    edge146,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="146", "Not(A:Brand";v="24", "Microsoft Edge";v="146""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 Edg/146.0.3856.109"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.7680.178 Mobile Safari/537.36 EdgA/146.0.3856.97"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 Edg/146.0.3856.109"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 Edg/146.0.3856.109"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/146.0.3856.102 Mobile/15E148 Safari/605.1.15")
    ]
);

edge_profile!(
    edge147,
    transport: v132::build_emulation,
    brands: r#""Microsoft Edge";v="147", "Not.A/Brand";v="8", "Chromium";v="147""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 Edg/147.0.3912.51"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.7727.55 Mobile Safari/537.36 EdgA/147.0.3912.51"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 Edg/147.0.3912.51"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 Edg/147.0.3912.51"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/147.0.3912.51 Mobile/15E148 Safari/605.1.15")
    ]
);

edge_profile!(
    edge148,
    transport: v132::build_emulation,
    brands: r#""Chromium";v="148", "Microsoft Edge";v="148", "Not/A)Brand";v="99""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36 Edg/148.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36 Edg/148.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36 Edg/148.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Mobile Safari/537.36 EdgA/148.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/148.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);

// Unlike newer Chrome, Edge does not advertise JPEG XL in navigation requests.
edge_profile!(
    edge149,
    transport: v132::build_emulation,
    brands: r#""Microsoft Edge";v="149", "Chromium";v="149", "Not)A;Brand";v="24""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 Edg/149.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 Edg/149.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 Edg/149.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Mobile Safari/537.36 EdgA/149.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/149.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge150,
    transport: v150::build_emulation,
    brands: r#""Not;A=Brand";v="8", "Chromium";v="150", "Microsoft Edge";v="150""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 Edg/150.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 Edg/150.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 Edg/150.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Mobile Safari/537.36 EdgA/150.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/150.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge151,
    transport: v150::build_emulation,
    brands: r#""Not=A?Brand";v="99", "Microsoft Edge";v="151", "Chromium";v="151""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 Edg/151.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 Edg/151.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 Edg/151.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Mobile Safari/537.36 EdgA/151.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/151.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
// Edge enables TLS signature-algorithm GREASE without sending Trust Anchor IDs.
edge_profile!(
    edge152,
    tls: tls::Preset::EdgeSignatureGrease,
    http2: http2::Preset::NoPush,
    brands: r#""Chromium";v="152", "Not?A_Brand";v="24", "Microsoft Edge";v="152""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36 Edg/152.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36 Edg/152.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36 Edg/152.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Mobile Safari/537.36 EdgA/152.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/152.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge153,
    transport: edge152::build_emulation,
    brands: r#""Microsoft Edge";v="153", "Not_A Brand";v="8", "Chromium";v="153""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 Edg/153.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 Edg/153.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 Edg/153.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Mobile Safari/537.36 EdgA/153.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/153.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge154,
    transport: edge152::build_emulation,
    brands: r#""Chromium";v="154", "Microsoft Edge";v="154", "Not A(Brand";v="99""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 Edg/154.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 Edg/154.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 Edg/154.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Mobile Safari/537.36 EdgA/154.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/154.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge155,
    transport: edge152::build_emulation,
    brands: r#""Microsoft Edge";v="155", "Chromium";v="155", "Not(A:Brand";v="24""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 Edg/155.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 Edg/155.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 Edg/155.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Mobile Safari/537.36 EdgA/155.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/155.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge156,
    transport: edge152::build_emulation,
    brands: r#""Not:A-Brand";v="8", "Chromium";v="156", "Microsoft Edge";v="156""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36 Edg/156.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36 Edg/156.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36 Edg/156.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Mobile Safari/537.36 EdgA/156.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/156.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge157,
    transport: edge152::build_emulation,
    brands: r#""Not-A.Brand";v="99", "Microsoft Edge";v="157", "Chromium";v="157""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 Edg/157.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 Edg/157.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 Edg/157.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Mobile Safari/537.36 EdgA/157.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/157.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
edge_profile!(
    edge158,
    transport: edge152::build_emulation,
    brands: r#""Chromium";v="158", "Not.A/Brand";v="24", "Microsoft Edge";v="158""#,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36 Edg/158.0.0.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36 Edg/158.0.0.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36 Edg/158.0.0.0"),
        (Android, "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Mobile Safari/537.36 EdgA/158.0.0.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/158.0.0.0 Mobile/15E148 Safari/605.1.15")
    ]
);
