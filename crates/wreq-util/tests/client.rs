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
        for (name, value) in req.headers() {
            if name == "user-agent" {
                assert_eq!(
                    value,
                    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
                );
            }
            if name == "sec-ch-ua" {
                assert_eq!(
                    value,
                    r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#
                );
            }
            if name == "sec-ch-ua-mobile" {
                assert_eq!(value, "?0");
            }
            if name == "sec-ch-ua-platform" {
                assert_eq!(value, "\"Linux\"");
            }
        }
        http::Response::default()
    });

    let url = format!("http://{}/ua", server.addr());
    let res = Client::builder()
        .emulation(
            Emulation::builder()
                .profile(Emulation::Chrome133)
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
