#![cfg(not(target_arch = "wasm32"))]
mod support;

use std::future::Future;

use support::server;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use wreq::Client;
use wreq_util::{Emulation, Platform, Profile};

const CHROMIUM_HEADER_ORDER: &[&str] = &[
    "sec-ch-ua",
    "sec-ch-ua-mobile",
    "sec-ch-ua-platform",
    "upgrade-insecure-requests",
    "user-agent",
    "accept",
    "sec-fetch-site",
    "sec-fetch-mode",
    "sec-fetch-user",
    "sec-fetch-dest",
    #[cfg(feature = "emulation-compression")]
    "accept-encoding",
    "accept-language",
    "priority",
];

const FIREFOX_HEADER_ORDER: &[&str] = &[
    "user-agent",
    "accept",
    "accept-language",
    #[cfg(feature = "emulation-compression")]
    "accept-encoding",
    "upgrade-insecure-requests",
    "sec-fetch-dest",
    "sec-fetch-mode",
    "sec-fetch-site",
    "sec-fetch-user",
    "te",
];

fn check_header_order<'a>(
    request: &'a [u8],
    stream: &'a mut TcpStream,
    expected: &'static [&'static str],
) -> Box<dyn Future<Output = ()> + Send + 'a> {
    Box::new(async move {
        let request = std::str::from_utf8(request)
            .expect("request should be valid UTF-8")
            .to_ascii_lowercase();
        let mut previous = None;

        for name in expected {
            let marker = format!("\r\n{name}:");
            let position = request
                .find(&marker)
                .unwrap_or_else(|| panic!("missing `{name}` header:\n{request}"));

            if let Some(previous) = previous {
                assert!(
                    position > previous,
                    "`{name}` header is out of order:\n{request}"
                );
            }

            previous = Some(position);
        }

        stream
            .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            .await
            .expect("response");
    })
}

async fn assert_emulation_headers(
    emulation: impl wreq::IntoEmulation,
    expected: &'static [&'static str],
) {
    let server = server::low_level_with_response(move |request, stream| {
        check_header_order(request, stream, expected)
    });
    Client::builder()
        .emulation(emulation)
        .no_proxy()
        .build()
        .expect("client")
        .get(format!("http://{}/headers", server.addr()))
        .send()
        .await
        .expect("request");
}

#[tokio::test]
async fn test_edge158_mobile_headers() {
    for (platform, platform_hint, user_agent) in [
        (
            Platform::Android,
            "\"Android\"",
            "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Mobile Safari/537.36 EdgA/158.0.0.0",
        ),
        (
            Platform::IOS,
            "\"iOS\"",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 EdgiOS/158.0.0.0 Mobile/15E148 Safari/605.1.15",
        ),
    ] {
        let server = server::http(move |request| async move {
            let headers = request.headers();
            assert_eq!(headers["user-agent"], user_agent);
            assert_eq!(headers["sec-ch-ua-platform"], platform_hint);
            assert_eq!(headers["sec-ch-ua-mobile"], "?1");
            if platform == Platform::Android {
                assert!(!headers["accept"].to_str().unwrap().contains("image/jxl"));
            }
            http::Response::default()
        });

        Client::builder()
            .emulation(
                Emulation::builder()
                    .profile(Emulation::Edge158)
                    .platform(platform)
                    .build(),
            )
            .no_proxy()
            .build()
            .unwrap()
            .get(format!("http://{}/", server.addr()))
            .send()
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn test_linux_navigation_emulation_headers() {
    let linux = |profile| {
        Emulation::builder()
            .profile(profile)
            .platform(Platform::Linux)
            .build()
    };
    assert_emulation_headers(linux(Profile::Chrome152), CHROMIUM_HEADER_ORDER).await;

    let server = server::http(|request| async move {
        assert_eq!(request.version(), http::Version::HTTP_2);
        let mut response = http::Response::default();
        response
            .headers_mut()
            .insert("user-agent", request.headers()["user-agent"].clone());
        response
    });
    let client = Client::builder()
        .http2_only()
        .no_proxy()
        .build()
        .expect("client");
    for (profile, major) in [
        (Profile::Firefox150, 150),
        (Profile::Firefox151, 151),
        (Profile::Firefox152, 152),
    ] {
        let response = client
            .get(format!("http://{}/headers", server.addr()))
            .emulation(linux(profile))
            .send()
            .await
            .expect("request");
        let expected = format!(
            "Mozilla/5.0 (X11; Linux x86_64; rv:{major}.0) Gecko/20100101 Firefox/{major}.0"
        );
        assert_eq!(response.headers()["user-agent"], expected, "{profile:?}");
    }
}

#[tokio::test]
async fn test_firefox_default_header_order() {
    assert_emulation_headers(Emulation::Firefox109, FIREFOX_HEADER_ORDER).await;
}

#[tokio::test]
async fn test_opera_linux_headers() {
    for (profile, expected_user_agent) in [
        (
            Emulation::Opera138,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 OPR/138.0.0.0",
        ),
        (
            Emulation::Opera139,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36 OPR/139.0.0.0",
        ),
        (
            Emulation::Opera140,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 OPR/140.0.0.0",
        ),
    ] {
        let server = server::http(move |request| async move {
            assert_eq!(request.version(), http::Version::HTTP_2);
            assert_eq!(
                request.headers()["user-agent"],
                expected_user_agent,
                "{profile:?}"
            );
            assert_eq!(
                request.headers()["sec-ch-ua-platform"],
                "\"Linux\"",
                "{profile:?}"
            );
            if profile == Profile::Opera140 {
                assert!(
                    request.headers()["accept"]
                        .to_str()
                        .unwrap()
                        .contains("image/jxl,image/avif")
                );
            }
            http::Response::default()
        });

        Client::builder()
            .emulation(
                Emulation::builder()
                    .profile(profile)
                    .platform(Platform::Linux)
                    .build(),
            )
            .http2_only()
            .no_proxy()
            .build()
            .expect("client")
            .get(format!("http://{}/", server.addr()))
            .send()
            .await
            .expect("request");
    }
}
