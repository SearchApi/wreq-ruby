#[macro_use]
mod http2;
#[macro_use]
mod tls;
mod header;

use header::*;
use tls::*;

use super::chrome::chromium_jxl_headers;
use super::*;

mod_generator!(
    opera116,
    tls_options!(CURVES),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="116", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/116.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="116", "Chromium";v="131", "Not_A Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/116.0.0.0"
        )
    ]
);

mod_generator!(
    opera117,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Opera";v="117""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36 OPR/117.0.0.0"
        ),
        (
            Windows,
            r#""Not A(Brand";v="8", "Chromium";v="132", "Opera";v="117""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36 OPR/117.0.0.0"
        )
    ]
);

mod_generator!(
    opera118,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not(A:Brand";v="99", "Opera";v="118", "Chromium";v="133""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 OPR/118.0.0.0"
        ),
        (
            Windows,
            r#""Not(A:Brand";v="99", "Opera";v="118", "Chromium";v="133""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 OPR/118.0.0.0"
        )
    ]
);

mod_generator!(
    opera119,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Opera";v="119""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 OPR/119.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="134", "Not:A-Brand";v="24", "Opera";v="119""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 OPR/119.0.0.0"
        )
    ]
);

mod_generator!(
    opera120,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Opera";v="120""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 OPR/120.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="135", "Not:A-Brand";v="24", "Opera";v="120""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 OPR/120.0.0.0"
        )
    ]
);

mod_generator!(
    opera121,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="121", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 OPR/121.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="121", "Chromium";v="137", "Not/A)Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 OPR/121.0.0.0"
        )
    ]
);

mod_generator!(
    opera122,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="138", "Not=A?Brand";v="24", "Opera";v="122""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 OPR/122.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="138", "Not=A?Brand";v="24", "Opera";v="122""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36 OPR/122.0.0.0"
        )
    ]
);

mod_generator!(
    opera123,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="139", "Not=A?Brand";v="24", "Opera";v="123""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 OPR/123.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="139", "Not=A?Brand";v="24", "Opera";v="123""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36 OPR/123.0.0.0"
        )
    ]
);

mod_generator!(
    opera124,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Opera";v="124""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 OPR/124.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="140", "Not=A?Brand";v="24", "Opera";v="124""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 OPR/124.0.0.0"
        )
    ]
);

mod_generator!(
    opera125,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="125", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 OPR/125.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="125", "Not?A_Brand";v="8", "Chromium";v="141""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 OPR/125.0.0.0"
        )
    ]
);

mod_generator!(
    opera126,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Opera";v="126", "Chromium";v="142""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 OPR/126.0.0.0"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Opera";v="126", "Chromium";v="142""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 OPR/126.0.0.0"
        )
    ]
);

mod_generator!(
    opera127,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Opera";v="127", "Chromium";v="143""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 OPR/127.0.0.0"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Opera";v="127", "Chromium";v="143""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 OPR/127.0.0.0"
        )
    ]
);

mod_generator!(
    opera128,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Opera";v="128", "Chromium";v="144""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 OPR/128.0.0.0"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Opera";v="128", "Chromium";v="144""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 OPR/128.0.0.0"
        )
    ]
);

mod_generator!(
    opera129,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Opera";v="129", "Chromium";v="145""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 OPR/129.0.0.0"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Opera";v="129", "Chromium";v="145""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 OPR/129.0.0.0"
        )
    ]
);

mod_generator!(
    opera130,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not:A-Brand";v="99", "Opera";v="130", "Chromium";v="146""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 OPR/130.0.0.0"
        ),
        (
            Windows,
            r#""Not:A-Brand";v="99", "Opera";v="130", "Chromium";v="146""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 OPR/130.0.0.0"
        )
    ]
);

mod_generator!(
    opera131,
    opera116::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="131", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 OPR/131.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="131", "Not.A/Brand";v="8", "Chromium";v="147""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36 OPR/131.0.0.0"
        )
    ]
);

// Reuse Chromium's TLS defaults with Opera's existing HTTP/2 settings.
// Brand hints use Chromium's milestone seed with Opera as the product brand.
mod_generator!(
    opera132,
    chrome::tls::tls_options!(7),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="148", "Opera";v="132", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36 OPR/132.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="148", "Opera";v="132", "Not/A)Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36 OPR/132.0.0.0"
        )
    ]
);

mod_generator!(
    opera133,
    opera132::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="133", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 OPR/133.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="133", "Chromium";v="149", "Not)A;Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 OPR/133.0.0.0"
        )
    ]
);

mod_generator!(
    opera134,
    chrome::tls::tls_options!(8),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Opera";v="134""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 OPR/134.0.0.0"
        ),
        (
            Windows,
            r#""Not;A=Brand";v="8", "Chromium";v="150", "Opera";v="134""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 OPR/134.0.0.0"
        )
    ]
);

mod_generator!(
    opera135,
    opera134::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Not=A?Brand";v="99", "Opera";v="135", "Chromium";v="151""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 OPR/135.0.0.0"
        ),
        (
            Windows,
            r#""Not=A?Brand";v="99", "Opera";v="135", "Chromium";v="151""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 OPR/135.0.0.0"
        )
    ]
);

mod_generator!(
    opera136,
    chrome::tls::tls_options!(9),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Opera";v="136""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36 OPR/136.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="152", "Not?A_Brand";v="24", "Opera";v="136""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36 OPR/136.0.0.0"
        )
    ]
);

mod_generator!(
    opera137,
    opera136::build_emulation,
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Opera";v="137", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 OPR/137.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="137", "Not_A Brand";v="8", "Chromium";v="153""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 OPR/137.0.0.0"
        )
    ]
);

mod_generator!(
    opera138,
    chrome::tls::tls_options!(10),
    http2_options!(),
    header_initializer_with_zstd_priority,
    [
        (
            MacOS,
            r#""Chromium";v="154", "Opera";v="138", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 OPR/138.0.0.0"
        ),
        (
            Windows,
            r#""Chromium";v="154", "Opera";v="138", "Not A(Brand";v="99""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 OPR/138.0.0.0"
        )
    ]
);

mod_generator!(
    opera139,
    opera138::build_emulation,
    chromium_jxl_headers,
    [
        (
            MacOS,
            r#""Opera";v="139", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 OPR/139.0.0.0"
        ),
        (
            Windows,
            r#""Opera";v="139", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 OPR/139.0.0.0"
        ),
        (
            Linux,
            r#""Opera";v="139", "Chromium";v="155", "Not(A:Brand";v="24""#,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 OPR/139.0.0.0"
        )
    ]
);

mod_generator!(
    opera140,
    opera138::build_emulation,
    chromium_jxl_headers,
    [
        (
            MacOS,
            r#""Not-A.Brand";v="99", "Opera";v="140", "Chromium";v="157""#,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 OPR/140.0.0.0"
        ),
        (
            Windows,
            r#""Not-A.Brand";v="99", "Opera";v="140", "Chromium";v="157""#,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 OPR/140.0.0.0"
        )
    ]
);
