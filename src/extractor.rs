use bytes::Bytes;
use magnus::{RArray, RHash, RString, Ruby, TryConvert, r_hash::ForEach};
use wreq::{
    Proxy, Version,
    header::{HeaderMap, HeaderName, HeaderValue, OrigHeaderMap},
};

use crate::error::{
    header_name_error_to_magnus, header_value_error_to_magnus, wreq_error_to_magnus,
};

/// A trait that defines the parameter name for extraction.
pub trait ExtractorName {
    /// The name of the parameter in the Ruby hash.
    const NAME: &str;
}

/// A generic extractor for various types.
pub struct Extractor<T>(Option<T>)
where
    T: ExtractorName;

impl<T> Extractor<T>
where
    T: ExtractorName,
{
    /// Consumes the extractor and returns the wrapped value.
    ///
    /// Returns `Some(T)` if a value was extracted, `None` otherwise.
    #[inline]
    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}

// ===== impl Extractor<Version> =====

impl ExtractorName for Version {
    const NAME: &str = "version";
}

impl TryConvert for Extractor<Version> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let keyword = RHash::try_convert(value)?;
        if let Some(version_val) = keyword.get(Version::NAME) {
            return <&crate::http::Version>::try_convert(version_val)
                .cloned()
                .map(crate::http::Version::into_ffi)
                .map(Some)
                .map(Extractor);
        }

        Ok(Extractor(None))
    }
}

// ===== impl Extractor<HeaderValue> =====

impl ExtractorName for HeaderValue {
    const NAME: &str = "user_agent";
}

impl TryConvert for Extractor<HeaderValue> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let keyword = RHash::try_convert(value)?;

        if let Some(ruby_value) = keyword
            .get(ruby.to_symbol(HeaderValue::NAME))
            .and_then(RString::from_value)
        {
            return HeaderValue::from_maybe_shared(ruby_value.to_bytes())
                .map(Some)
                .map(Extractor)
                .map_err(header_value_error_to_magnus);
        }

        Ok(Extractor(None))
    }
}

// ===== impl Extractor<HeaderMap> =====

impl ExtractorName for HeaderMap {
    const NAME: &str = "headers";
}

impl TryConvert for Extractor<HeaderMap> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let keyword = RHash::try_convert(value)?;
        let mut headers = HeaderMap::new();

        if let Some(hash) = keyword
            .get(ruby.to_symbol(HeaderMap::NAME))
            .and_then(RHash::from_value)
        {
            hash.foreach(|name: RString, value: RString| {
                let name = HeaderName::from_bytes(&name.to_bytes())
                    .map_err(header_name_error_to_magnus)?;
                let value = HeaderValue::from_maybe_shared(value.to_bytes())
                    .map_err(header_value_error_to_magnus)?;
                headers.insert(name, value);

                Ok(ForEach::Continue)
            })?;

            return Ok(Extractor(Some(headers)));
        }

        Ok(Extractor(None))
    }
}

// ===== impl Extractor<OrigHeaderMap> =====

impl ExtractorName for OrigHeaderMap {
    const NAME: &str = "orig_headers";
}

impl TryConvert for Extractor<OrigHeaderMap> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let keyword = RHash::try_convert(value)?;

        if let Some(orig_headers) = keyword
            .get(ruby.to_symbol(OrigHeaderMap::NAME))
            .and_then(RArray::from_value)
        {
            let mut map = OrigHeaderMap::new();
            for value in orig_headers.into_iter().flat_map(RString::from_value) {
                map.insert(value.to_bytes());
            }
            return Ok(Extractor(Some(map)));
        }

        Ok(Extractor(None))
    }
}

// ===== impl Extractor<Vec<HeaderValue>> =====

impl ExtractorName for Vec<HeaderValue> {
    const NAME: &str = "cookies";
}

impl TryConvert for Extractor<Vec<HeaderValue>> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let keyword = RHash::try_convert(value)?;

        if let Some(hash) = keyword
            .get(ruby.to_symbol(Vec::<HeaderValue>::NAME))
            .and_then(RHash::from_value)
        {
            let mut cookies = Vec::new();
            hash.foreach(|name: RString, value: RString| {
                let cookie = format!("{name}={value}");
                let header_value = HeaderValue::from_maybe_shared(Bytes::from(cookie))
                    .map_err(header_value_error_to_magnus)?;
                cookies.push(header_value);
                Ok(ForEach::Continue)
            })?;

            return Ok(Extractor(Some(cookies)));
        }

        Ok(Extractor(None))
    }
}

// ===== impl Extractor<Proxy> =====

impl ExtractorName for Proxy {
    const NAME: &str = "proxy";
}

impl TryConvert for Extractor<Proxy> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let keyword = RHash::try_convert(value)?;

        if let Some(proxy) = keyword
            .get(ruby.to_symbol(Proxy::NAME))
            .and_then(RString::from_value)
        {
            return Proxy::all(proxy.to_bytes().as_ref())
                .map(Some)
                .map(Extractor)
                .map_err(wreq_error_to_magnus);
        }

        Ok(Extractor(None))
    }
}
