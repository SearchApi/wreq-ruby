#[cfg(feature = "emulation-compression")]
use super::super::ACCEPT_ENCODING;
use super::super::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, Platform, USER_AGENT};

const DEFAULT_ACCEPT: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7";
const JXL_ACCEPT: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/jxl,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7";

#[derive(Clone, Copy)]
pub(super) enum Preset {
    Legacy,
    Zstd,
    ZstdWithPriority,
    Jxl,
    EarlyLanguage,
}

impl Preset {
    pub(super) fn build(
        self,
        brands: &'static str,
        ua: &'static str,
        platform: Platform,
    ) -> HeaderMap {
        // Chrome 152 keeps the regular Accept-Language position on Linux.
        let early_language =
            matches!(self, Self::EarlyLanguage) && !matches!(platform, Platform::Linux);
        let language = "en-US,en;q=0.9";
        let mobile = if platform.is_mobile() { "?1" } else { "?0" };

        // Chrome 155+ enables JPEG XL in Blink, not in the iOS WebKit profile.
        // https://chromium.googlesource.com/chromium/src/+/cfaadc5a132d78e1828635aa8405a499f3e14864/content/common/content_constants_internal.h
        let accept = if matches!(self, Self::Jxl) && !matches!(platform, Platform::IOS) {
            JXL_ACCEPT
        } else {
            DEFAULT_ACCEPT
        };

        let mut headers = HeaderMap::new();
        headers.insert("sec-ch-ua", HeaderValue::from_static(brands));
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_static(mobile));
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_static(platform.platform()),
        );
        if early_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static(language));
        }
        headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
        headers.insert(USER_AGENT, HeaderValue::from_static(ua));
        headers.insert(ACCEPT, HeaderValue::from_static(accept));
        headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
        headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
        headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
        #[cfg(feature = "emulation-compression")]
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static(match self {
                Self::Legacy => "gzip, deflate, br",
                _ => "gzip, deflate, br, zstd",
            }),
        );
        if !early_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static(language));
        }
        if matches!(
            self,
            Self::ZstdWithPriority | Self::Jxl | Self::EarlyLanguage
        ) {
            headers.insert("priority", HeaderValue::from_static("u=0, i"));
        }
        headers
    }
}
