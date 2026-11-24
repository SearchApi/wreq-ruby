use super::*;

#[inline]
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
