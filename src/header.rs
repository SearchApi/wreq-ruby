use std::{cell::RefCell, sync::atomic::Ordering};

use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue};
use magnus::{Error, Module, Object, RModule, Ruby, block::Yield, function, method};

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
    pub fn new() -> Self {
        Self::from(HeaderMap::new())
    }

    /// Get a header value by name (case-insensitive).
    pub fn get(&self, name: String) -> Option<Bytes> {
        self.0.borrow().get(&name).cloned().map(Bytes::from_owner)
    }

    /// Get all values for a header name (case-insensitive).
    pub fn get_all(&self, name: String) -> Vec<String> {
        self.0
            .borrow()
            .get_all(&name)
            .iter()
            .map(|v| String::from_utf8_lossy(v.as_bytes()).to_string())
            .collect()
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
    pub fn remove(&self, name: String) -> Option<Bytes> {
        self.0.borrow_mut().remove(&name).map(Bytes::from_owner)
    }

    /// Check if a header exists (case-insensitive).
    pub fn contains(&self, name: String) -> bool {
        self.0.borrow().contains_key(&name)
    }

    /// Get the number of headers.
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Check if headers are empty.
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    /// Clear all headers.
    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    /// Get all header names.
    pub fn keys(&self) -> Vec<String> {
        self.0
            .borrow()
            .keys()
            .map(|k| k.as_str().to_string())
            .collect()
    }

    /// Get all header values.
    pub fn values(&self) -> Vec<String> {
        self.0
            .borrow()
            .values()
            .map(|v| String::from_utf8_lossy(v.as_bytes()).to_string())
            .collect()
    }

    /// Iterate over headers with Ruby block support.
    pub fn each(&self) -> Result<Yield<HeaderIterator>, Error> {
        Ok(Yield::Iter(HeaderIterator::new(&self.0.borrow())))
    }
}

impl From<HeaderMap> for Headers {
    fn from(headers: HeaderMap) -> Self {
        Self(RefCell::new(headers))
    }
}

/// Iterator for HTTP headers that yields (name, value) pairs to Ruby blocks.
#[magnus::wrap(class = "Wreq::HeaderIterator", free_immediately, size)]
pub struct HeaderIterator {
    headers: Vec<(Bytes, Bytes)>,
    index: std::sync::atomic::AtomicUsize,
}

impl HeaderIterator {
    /// Create a new header iterator from a HeaderMap.
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            headers: headers
                .iter()
                .map(|(name, value)| {
                    (
                        Bytes::from_owner(name.clone()),
                        Bytes::from_owner(value.clone()),
                    )
                })
                .collect(),
            index: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl Iterator for HeaderIterator {
    type Item = (Bytes, Bytes);

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index.fetch_add(1, Ordering::SeqCst);
        if current_index < self.headers.len() {
            let (name, value) = &self.headers[current_index];
            Some((name.clone(), value.clone()))
        } else {
            None
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

    // Define HeaderIterator class
    gem_module.define_class("HeaderIterator", ruby.class_object())?;

    Ok(())
}
