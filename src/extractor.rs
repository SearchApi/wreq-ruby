use magnus::{RHash, RString, Ruby, TryConvert, value::ReprValue};
use wreq::Proxy;

use crate::error::{option_value_error, wreq_error};
use crate::options;

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

// ===== impl Extractor<Proxy> =====

impl ExtractorName for Proxy {
    const NAME: &str = "proxy";
}

impl TryConvert for Extractor<Proxy> {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        let rhash = RHash::try_convert(value)?;

        let Some(value) = options::get(&ruby, rhash, Proxy::NAME) else {
            return Ok(Extractor(None));
        };
        if value.is_nil() {
            return Ok(Extractor(None));
        }

        let proxy =
            RString::try_convert(value).map_err(|error| option_value_error(Proxy::NAME, error))?;
        let proxy = Proxy::all(proxy.to_bytes().as_ref())
            .map_err(|err| wreq_error(&ruby, err))
            .map_err(|error| option_value_error(Proxy::NAME, error))?;

        Ok(Extractor(Some(proxy)))
    }
}
