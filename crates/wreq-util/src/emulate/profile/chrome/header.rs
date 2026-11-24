use super::*;

pub(super) fn navigation_headers(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: Platform,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );
    header_chrome_ua!(headers, ua);
    header_chrome_accept!(headers);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept_encoding!(headers);
    headers
}

pub(super) fn navigation_headers_with_zstd(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: Platform,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );
    header_chrome_ua!(headers, ua);
    header_chrome_accept!(headers);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept_encoding!(zstd, headers);
    headers
}

pub(super) fn navigation_headers_with_zstd_and_priority(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: Platform,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );
    header_chrome_ua!(headers, ua);
    header_chrome_accept!(headers);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept_encoding!(zstd, headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    headers
}

pub(in crate::emulate::profile) fn navigation_headers_with_jxl(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: Platform,
) -> HeaderMap {
    let mut headers = navigation_headers_with_zstd_and_priority(sec_ch_ua, ua, emulation_os);
    // Chrome 155 enables JPEG XL in Blink; the iOS profile keeps its WebKit headers.
    // https://chromium.googlesource.com/chromium/src/+/cfaadc5a132d78e1828635aa8405a499f3e14864/content/common/content_constants_internal.h
    if !matches!(emulation_os, Platform::IOS) {
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/jxl,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    }
    headers
}

pub(super) fn navigation_headers_with_language_and_zstd_only(
    sec_ch_ua: &'static str,
    ua: &'static str,
    emulation_os: Platform,
) -> HeaderMap {
    // Linux sends Accept-Language after Accept-Encoding for this profile.
    if matches!(emulation_os, Platform::Linux) {
        return navigation_headers_with_zstd_and_priority(sec_ch_ua, ua, emulation_os);
    }

    let mut headers = HeaderMap::new();
    header_chrome_sec_ch_ua!(
        headers,
        sec_ch_ua,
        emulation_os.platform(),
        emulation_os.is_mobile()
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );
    header_chrome_ua!(headers, ua);
    header_chrome_accept!(headers);
    header_chrome_sec_fetch!(headers);
    header_chrome_accept_encoding!(zstd_only, headers);
    headers.insert(
        HeaderName::from_static("priority"),
        HeaderValue::from_static("u=0, i"),
    );
    headers
}
