#[cfg(feature = "emulation-compression")]
use wreq::header::ACCEPT_ENCODING;
use wreq::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};

use super::Platform;

#[derive(Clone, Copy)]
pub(super) enum Preset {
    Legacy,
    ZstdWithPriority,
    LanguageQ09,
    LanguageQ09WithAndroidLocale,
}

impl Preset {
    pub(super) fn build(self, platform: Platform, ua: &'static str) -> HeaderMap {
        // Firefox 147 switched to decrementing the q-value by 0.1 per entry (Bugzilla 2000765).
        let language = match (self, platform) {
            (Self::Legacy | Self::ZstdWithPriority, _) => "en-US,en;q=0.5",
            // GeckoView uses Android's locale list without adding a fallback (Bugzilla 2042376).
            (Self::LanguageQ09WithAndroidLocale, Platform::Android) => "en-US",
            (Self::LanguageQ09 | Self::LanguageQ09WithAndroidLocale, _) => "en-US,en;q=0.9",
        };

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(ua));
        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static(language));
        #[cfg(feature = "emulation-compression")]
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static(match self {
                Self::Legacy => "gzip, deflate, br",
                _ => "gzip, deflate, br, zstd",
            }),
        );
        headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
        headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
        headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
        if !matches!(self, Self::Legacy) {
            headers.insert("priority", HeaderValue::from_static("u=0, i"));
        }
        headers.insert("te", HeaderValue::from_static("trailers"));
        headers
    }
}
