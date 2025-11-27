use std::cell::RefCell;

use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue};
use magnus::{
    Error, Module, Object, RArray, RModule, Ruby, block::Yield, function, method,
    typed_data::Inspect,
};

use crate::error::{header_name_error_to_magnus, header_value_error_to_magnus};

/// HTTP headers collection with read and write operations.
///
/// This class wraps HTTP headers and provides convenient methods for
/// accessing, modifying, and iterating over header name-value pairs.
#[derive(Clone, Default)]
#[magnus::wrap(class = "Wreq::Headers", free_immediately, size)]
pub struct Headers(RefCell<HeaderMap>);

impl Headers {
    /// Create a new empty Headers instance.
    #[inline]
    pub fn new() -> Self {
        Self::from(HeaderMap::new())
    }

    /// Get a header value by name (case-insensitive).
    #[inline]
    pub fn get(&self, name: String) -> Option<Bytes> {
        self.0.borrow().get(&name).cloned().map(Bytes::from_owner)
    }

    /// Get all values for a header name (case-insensitive).
    #[inline]
    pub fn get_all(ruby: &Ruby, rb_self: &Self, name: String) -> RArray {
        ruby.ary_from_iter(
            rb_self
                .0
                .borrow()
                .get_all(&name)
                .iter()
                .cloned()
                .map(Bytes::from_owner),
        )
    }

    /// Set a header, replacing any existing values.
    pub fn set(&self, name: String, value: String) -> Result<(), Error> {
        let header_name = name
            .parse::<HeaderName>()
            .map_err(header_name_error_to_magnus)?;
        let header_value = HeaderValue::from_maybe_shared(Bytes::from(value))
            .map_err(header_value_error_to_magnus)?;

        self.0.borrow_mut().insert(header_name, header_value);
        Ok(())
    }

    /// Append a header value without replacing existing values.
    pub fn append(&self, name: String, value: String) -> Result<(), Error> {
        let header_name = name
            .parse::<http::header::HeaderName>()
            .map_err(header_name_error_to_magnus)?;
        let header_value = HeaderValue::from_maybe_shared(Bytes::from(value))
            .map_err(header_value_error_to_magnus)?;

        self.0.borrow_mut().append(header_name, header_value);
        Ok(())
    }

    /// Remove all values for a header name.
    #[inline]
    pub fn remove(&self, name: String) -> Option<Bytes> {
        self.0.borrow_mut().remove(&name).map(Bytes::from_owner)
    }

    /// Check if a header exists (case-insensitive).
    #[inline]
    pub fn contains(&self, name: String) -> bool {
        self.0.borrow().contains_key(&name)
    }

    /// Get the number of headers.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Check if headers are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    /// Clear all headers.
    #[inline]
    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    /// Get all header names.
    #[inline]
    pub fn keys(ruby: &Ruby, rb_self: &Self) -> RArray {
        ruby.ary_from_iter(rb_self.0.borrow().keys().cloned().map(Bytes::from_owner))
    }

    /// Get all header values.
    #[inline]
    pub fn values(ruby: &Ruby, rb_self: &Self) -> RArray {
        ruby.ary_from_iter(rb_self.0.borrow().values().cloned().map(Bytes::from_owner))
    }

    /// Iterate over headers with Ruby block support.
    #[inline]
    pub fn each(&self) -> Yield<impl Iterator<Item = (Bytes, Bytes)>> {
        Yield::Iter(HeaderIter {
            inner: self.0.borrow().clone().into_iter(),
            next_name: None,
        })
    }

    /// Convert headers to string representation.
    #[inline]
    pub fn to_s(&self) -> String {
        self.0.borrow().inspect()
    }
}

impl From<HeaderMap> for Headers {
    fn from(headers: HeaderMap) -> Self {
        Self(RefCell::new(headers))
    }
}

struct HeaderIter {
    inner: http::header::IntoIter<HeaderValue>,
    next_name: Option<HeaderName>,
}

impl Iterator for HeaderIter {
    type Item = (Bytes, Bytes);
    fn next(&mut self) -> Option<Self::Item> {
        let (name, value) = self.inner.next()?;
        match (&self.next_name, name) {
            (Some(next_name), None) => Some((
                Bytes::from_owner(next_name.clone()),
                Bytes::from_owner(value),
            )),
            (_, Some(name)) => {
                self.next_name = Some(name.clone());
                Some((Bytes::from_owner(name), Bytes::from_owner(value)))
            }
            (None, None) => None,
        }
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // Define Headers class with methods
    let headers_class = gem_module.define_class("Headers", ruby.class_object())?;
    headers_class.define_singleton_method("new", function!(Headers::new, 0))?;
    headers_class.define_method("get", method!(Headers::get, 1))?;
    headers_class.define_method("get_all", method!(Headers::get_all, 1))?;
    headers_class.define_method("set", method!(Headers::set, 2))?;
    headers_class.define_method("append", method!(Headers::append, 2))?;
    headers_class.define_method("remove", method!(Headers::remove, 1))?;
    headers_class.define_method("contains?", method!(Headers::contains, 1))?;
    headers_class.define_method("key?", method!(Headers::contains, 1))?;
    headers_class.define_method("length", method!(Headers::len, 0))?;
    headers_class.define_method("empty?", method!(Headers::is_empty, 0))?;
    headers_class.define_method("clear", method!(Headers::clear, 0))?;
    headers_class.define_method("keys", method!(Headers::keys, 0))?;
    headers_class.define_method("values", method!(Headers::values, 0))?;
    headers_class.define_method("each", method!(Headers::each, 0))?;
    headers_class.define_method("to_s", method!(Headers::to_s, 0))?;
    Ok(())
}
