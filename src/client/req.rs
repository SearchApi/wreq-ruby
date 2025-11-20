use http::HeaderValue;
use indexmap::IndexMap;
use magnus::{RHash, TryConvert, value::ReprValue};
use serde::Deserialize;
use wreq::{
    Proxy, Version,
    header::{HeaderMap, OrigHeaderMap},
};

use super::body::{Body, Json};
use crate::extractor::Extractor;

/// The parameters for a request.
#[derive(Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct Request {
    /// The proxy to use for the request.
    #[serde(skip)]
    pub proxy: Option<Proxy>,

    /// The timeout to use for the request.
    pub timeout: Option<u64>,

    /// The read timeout to use for the request.
    pub read_timeout: Option<u64>,

    /// The HTTP version to use for the request.
    #[serde(skip)]
    pub version: Option<Version>,

    /// The headers to use for the request.
    #[serde(skip)]
    pub headers: Option<HeaderMap>,

    /// The original headers to use for the request.
    #[serde(skip)]
    pub orig_headers: Option<OrigHeaderMap>,

    /// The option enables default headers.
    pub default_headers: Option<bool>,

    /// The cookies to use for the request.
    #[serde(skip)]
    pub cookies: Option<Vec<HeaderValue>>,

    /// Whether to allow redirects.
    pub allow_redirects: Option<bool>,

    /// The maximum number of redirects to follow.
    pub max_redirects: Option<usize>,

    /// Sets gzip as an accepted encoding.
    pub gzip: Option<bool>,

    /// Sets brotli as an accepted encoding.
    pub brotli: Option<bool>,

    /// Sets deflate as an accepted encoding.
    pub deflate: Option<bool>,

    /// Sets zstd as an accepted encoding.
    pub zstd: Option<bool>,

    /// The authentication to use for the request.
    pub auth: Option<String>,

    /// The bearer authentication to use for the request.
    pub bearer_auth: Option<String>,

    /// The basic authentication to use for the request.
    pub basic_auth: Option<(String, Option<String>)>,

    /// The query parameters to use for the request.
    pub query: Option<IndexMap<String, String>>,

    /// The form parameters to use for the request.
    pub form: Option<IndexMap<String, String>>,

    /// The JSON body to use for the request.
    pub json: Option<Json>,

    /// The body to use for the request.
    pub body: Option<Body>,
}

impl Request {
    /// Create a new [`Request`] from Ruby keyword arguments.
    pub fn new(ruby: &magnus::Ruby, kwargs: RHash) -> Result<Self, magnus::Error> {
        let kwargs = kwargs.as_value();
        let mut builder: Self = serde_magnus::deserialize(&ruby, kwargs)?;

        // extra version handling
        builder.version = Extractor::<Version>::try_convert(kwargs)?.into_inner();

        // extra headers handling
        builder.headers = Extractor::<HeaderMap>::try_convert(kwargs)?.into_inner();

        // extra original headers handling
        builder.orig_headers = Extractor::<OrigHeaderMap>::try_convert(kwargs)?.into_inner();

        // extra cookies handling
        builder.cookies = Extractor::<Vec<HeaderValue>>::try_convert(kwargs)?.into_inner();

        // extra proxy handling
        builder.proxy = Extractor::<Proxy>::try_convert(kwargs)?.into_inner();

        Ok(builder)
    }
}
