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

        assert!(request.contains("\r\nupgrade-insecure-requests: 1\r\n"));
        assert!(request.contains("\r\nsec-fetch-user: ?1\r\n"));

        stream
            .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            .await
            .expect("response");
    })
}

async fn assert_emulation_headers(profile: Profile, expected: &'static [&'static str]) {
    let server = server::low_level_with_response(move |request, stream| {
        check_header_order(request, stream, expected)
    });
    let response = Client::builder()
        .emulation(profile)
        .build()
        .expect("client")
        .get(format!("http://{}/headers", server.addr()))
        .send()
        .await
        .expect("request");

    assert_eq!(response.status(), wreq::StatusCode::OK);
}

#[tokio::test]
async fn test_client_emulation_device() {
    let server = server::http(move |req| async move {
        let header = |name| {
            req.headers()
                .get(name)
                .and_then(|value| value.to_str().ok())
        };

        assert_eq!(
            header("user-agent"),
            Some(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36"
            )
        );
        assert_eq!(
            header("sec-ch-ua"),
            Some(r#""Not=A?Brand";v="99", "Google Chrome";v="151", "Chromium";v="151""#)
        );
        assert_eq!(header("sec-ch-ua-mobile"), Some("?0"));
        assert_eq!(header("sec-ch-ua-platform"), Some("\"Linux\""));
        http::Response::default()
    });

    let url = format!("http://{}/ua", server.addr());
    let res = Client::builder()
        .emulation(
            Emulation::builder()
                .profile(Emulation::Chrome151)
                .platform(Platform::Linux)
                .http2(true)
                .build(),
        )
        .build()
        .expect("Unable to build client")
        .get(&url)
        .send()
        .await
        .expect("request");

    assert_eq!(res.status(), wreq::StatusCode::OK);
}

#[tokio::test]
async fn test_chrome_default_header_order() {
    assert_emulation_headers(Emulation::Chrome133, CHROMIUM_HEADER_ORDER).await;
}

#[tokio::test]
async fn test_firefox_default_header_order() {
    assert_emulation_headers(Emulation::Firefox109, FIREFOX_HEADER_ORDER).await;
}

#[tokio::test]
async fn test_opera_default_header_order() {
    assert_emulation_headers(Emulation::Opera116, CHROMIUM_HEADER_ORDER).await;
}

#[tokio::test]
async fn test_opera_linux_headers() {
    for (profile, expected_user_agent) in [
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
            http::Response::default()
        });

        let response = Client::builder()
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

        assert_eq!(response.status(), wreq::StatusCode::OK, "{profile:?}");
    }
}

#[tokio::test]
async fn test_opera140_desktop_headers() {
    let server = server::http(move |request| async move {
        let headers = request.headers();
        assert_eq!(
            headers["user-agent"],
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36 OPR/140.0.0.0"
        );
        assert_eq!(
            headers["sec-ch-ua"],
            r#""Not-A.Brand";v="99", "Opera";v="140", "Chromium";v="157""#
        );
        assert!(
            headers["accept"]
                .to_str()
                .unwrap()
                .contains("image/jxl,image/avif")
        );
        http::Response::default()
    });

    let response = Client::builder()
        .emulation(
            Emulation::builder()
                .profile(Emulation::Opera140)
                .platform(Platform::Windows)
                .build(),
        )
        .build()
        .unwrap()
        .get(format!("http://{}/", server.addr()))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), wreq::StatusCode::OK);
}
