#[macro_use]
mod http2;
#[macro_use]
mod tls;
mod header;

use header::*;
use tls::*;

use super::*;

// The first platform entry supplies the UA when a platform is not listed.
macro_rules! firefox_profile {
    (
        $name:ident,
        tls: $tls:expr,
        http2: $http2:expr,
        headers: $headers:ident,
        [$(($platform:ident, $ua:literal)),+ $(,)?] $(,)?
    ) => {
        mod_generator!($name, $tls, $http2, $headers, [$(($platform, $ua)),+]);
    };
    (
        $name:ident,
        transport: $transport:path,
        headers: $headers:ident,
        [$(($platform:ident, $ua:literal)),+ $(,)?] $(,)?
    ) => {
        mod_generator!($name, $transport, $headers, [$(($platform, $ua)),+]);
    };
}

firefox_profile!(
    ff109,
    tls: tls_options!(2, CIPHER_LIST_1, CURVES_1),
    http2: http2_options!(2),
    headers: header_initializer,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_17; rv:109.0) Gecko/20000101 Firefox/109.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/109.0 Firefox/109.0"),
        (Linux, "Mozilla/5.0 (X11; Linux i686; rv:109.0) Gecko/20100101 Firefox/109.0"),
        (IOS, "Mozilla/5.0 (iPad; CPU OS 13_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/109.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff117,
    transport: ff109::build_emulation,
    headers: header_initializer,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:117.0) Gecko/20100101 Firefox/117.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_16_1; rv:117.0) Gecko/20010101 Firefox/117.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:117.0) Gecko/117.0 Firefox/117.0"),
        (Linux, "Mozilla/5.0 (X11; Linux i686; rv:117.0) Gecko/20100101 Firefox/117.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 14_7_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/117.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff128,
    tls: tls_options!(3, CIPHER_LIST_2, CURVES_1, KEY_SHARES_1),
    http2: http2_options!(3),
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:128.0) Gecko/128.0 Firefox/128.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/128.0 Mobile/15E148 Safari/605.1.15"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
    ]
);

firefox_profile!(
    ff133,
    tls: tls_options!(1, CIPHER_LIST_1, CURVES_2, KEY_SHARES_2),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:133.0) Gecko/133.0 Firefox/133.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:133.0) Gecko/20100101 Firefox/133.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/133.4 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff135,
    tls: tls_options!(
        4,
        CIPHER_LIST_1,
        CURVES_2,
        KEY_SHARES_2,
        EchGrease::NssPre3127
    ),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:135.0) Gecko/20100101 Firefox/135.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:135.0) Gecko/20100101 Firefox/135.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0")
    ]
);

firefox_profile!(
    ff_private_135,
    tls: tls_options!(6, CIPHER_LIST_1, CURVES_2, KEY_SHARES_2),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:135.0) Gecko/20100101 Firefox/135.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:135.0) Gecko/20100101 Firefox/135.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0")
    ]
);

firefox_profile!(
    ff_android_135,
    tls: tls_options!(5, CIPHER_LIST_1, CURVES_1, KEY_SHARES_1),
    http2: http2_options!(4),
    headers: header_initializer_with_zstd,
    [
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:135.0) Gecko/135.0 Firefox/135.0")
    ]
);

firefox_profile!(
    ff136,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:136.0) Gecko/20100101 Firefox/136.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:136.0) Gecko/20100101 Firefox/136.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:136.0) Gecko/20100101 Firefox/136.0")
    ]
);

firefox_profile!(
    ff_private_136,
    transport: ff_private_135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:136.0) Gecko/20100101 Firefox/136.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:136.0) Gecko/20100101 Firefox/136.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:136.0) Gecko/20100101 Firefox/136.0")
    ]
);

firefox_profile!(
    ff139,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0"),
        (Windows, "Mozilla/5.0 (Windows NT 10.0; rv:136.0) Gecko/20100101 Firefox/139.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:136.0) Gecko/20100101 Firefox/139.0")
    ]
);

firefox_profile!(
    ff142,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:142.0) Gecko/20100101 Firefox/142.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:142.0) Gecko/20100101 Firefox/142.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:142.0) Gecko/142.0 Firefox/142.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/142.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff143,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:143.0) Gecko/20100101 Firefox/143.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:143.0) Gecko/20100101 Firefox/143.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:143.0) Gecko/143.0 Firefox/143.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/143.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff144,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:144.0) Gecko/20100101 Firefox/144.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:144.0) Gecko/144.0 Firefox/144.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/144.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff145,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:145.0) Gecko/20100101 Firefox/145.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:145.0) Gecko/20100101 Firefox/145.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:145.0) Gecko/145.0 Firefox/145.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/145.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff146,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:146.0) Gecko/20100101 Firefox/146.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:146.0) Gecko/20100101 Firefox/146.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:146.0) Gecko/20100101 Firefox/146.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:146.0) Gecko/146.0 Firefox/146.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/146.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff147,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:147.0) Gecko/20100101 Firefox/147.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:147.0) Gecko/20100101 Firefox/147.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:147.0) Gecko/20100101 Firefox/147.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:147.0) Gecko/147.0 Firefox/147.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/147.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff148,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:148.0) Gecko/20100101 Firefox/148.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:148.0) Gecko/20100101 Firefox/148.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:148.0) Gecko/20100101 Firefox/148.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:148.0) Gecko/148.0 Firefox/148.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/148.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff149,
    transport: ff135::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:149.0) Gecko/20100101 Firefox/149.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:149.0) Gecko/20100101 Firefox/149.0"),
        (Linux, "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:149.0) Gecko/20100101 Firefox/149.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:149.0) Gecko/149.0 Firefox/149.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/149.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff150,
    tls: tls_options!(
        4,
        CIPHER_LIST_3,
        CURVES_2,
        KEY_SHARES_2,
        EchGrease::NssPre3127
    ),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:150.0) Gecko/20100101 Firefox/150.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:150.0) Gecko/20100101 Firefox/150.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:150.0) Gecko/150.0 Firefox/150.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/150.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff151,
    transport: ff150::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:151.0) Gecko/20100101 Firefox/151.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:151.0) Gecko/20100101 Firefox/151.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:151.0) Gecko/20100101 Firefox/151.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:151.0) Gecko/151.0 Firefox/151.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/151.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff152,
    transport: ff150::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:152.0) Gecko/20100101 Firefox/152.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:152.0) Gecko/20100101 Firefox/152.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0"),
        (Android, "Mozilla/5.0 (Android 13; Mobile; rv:152.0) Gecko/152.0 Firefox/152.0"),
        (IOS, "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/152.0 Mobile/15E148 Safari/605.1.15")
    ]
);

firefox_profile!(
    ff153,
    transport: ff150::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:153.0) Gecko/20100101 Firefox/153.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:153.0) Gecko/20100101 Firefox/153.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:153.0) Gecko/20100101 Firefox/153.0")
    ]
);

firefox_profile!(
    ff154,
    tls: tls_options!(
        4,
        CIPHER_LIST_2,
        CURVES_2,
        KEY_SHARES_2,
        EchGrease::NssPre3127
    ),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:154.0) Gecko/20100101 Firefox/154.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:154.0) Gecko/20100101 Firefox/154.0")
    ]
);

firefox_profile!(
    ff155,
    tls: tls_options!(4, CIPHER_LIST_2, CURVES_2, KEY_SHARES_2, EchGrease::Nss3127),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:155.0) Gecko/20100101 Firefox/155.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:155.0) Gecko/20100101 Firefox/155.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:155.0) Gecko/20100101 Firefox/155.0")
    ]
);

firefox_profile!(
    ff156,
    tls: tls_options!(4, CIPHER_LIST_2, CURVES_3, KEY_SHARES_2, EchGrease::Nss3127),
    http2: http2_options!(1),
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:156.0) Gecko/20100101 Firefox/156.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:156.0) Gecko/20100101 Firefox/156.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:156.0) Gecko/20100101 Firefox/156.0")
    ]
);

firefox_profile!(
    ff157,
    transport: ff156::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:157.0) Gecko/20100101 Firefox/157.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:157.0) Gecko/20100101 Firefox/157.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:157.0) Gecko/20100101 Firefox/157.0")
    ]
);

firefox_profile!(
    ff158,
    transport: ff156::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:158.0) Gecko/20100101 Firefox/158.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:158.0) Gecko/20100101 Firefox/158.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:158.0) Gecko/20100101 Firefox/158.0")
    ]
);

firefox_profile!(
    ff159,
    transport: ff156::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:159.0) Gecko/20100101 Firefox/159.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0) Gecko/20100101 Firefox/159.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:159.0) Gecko/20100101 Firefox/159.0")
    ]
);

firefox_profile!(
    ff160,
    transport: ff156::build_emulation,
    headers: header_initializer_with_zstd_q09,
    [
        (Windows, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:160.0) Gecko/20100101 Firefox/160.0"),
        (MacOS, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:160.0) Gecko/20100101 Firefox/160.0"),
        (Linux, "Mozilla/5.0 (X11; Linux x86_64; rv:160.0) Gecko/20100101 Firefox/160.0")
    ]
);
