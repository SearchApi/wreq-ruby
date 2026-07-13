//! Native support for `Wreq::Headers`.
//!
//! Header names are normalized by [`HeaderMap`] and compared without regard to
//! case, as required by [RFC 9110 section 5.1]. Ruby collection adapters add
//! construction, indexing, and enumeration without changing the underlying
//! header representation. Exact wire casing and order remain the responsibility
//! of the `orig_headers` request option.
//!
//! [RFC 9110 section 5.1]: https://www.rfc-editor.org/rfc/rfc9110.html#section-5.1

use std::cell::RefCell;

use bytes::Bytes;
use http::{HeaderMap, HeaderValue};
use magnus::{
    Error, RArray, RModule, RString, Ruby, TryConvert, Value, function, method,
    prelude::*,
    typed_data::{Inspect, Obj},
};
use wreq::header::OrigHeaderMap;

use crate::error::{header_value_error_to_magnus, type_value_error_to_magnus};

use self::helper::{
    ensure_header_count, from_source, header_count_error, parse_header_name, parse_header_values,
};

/// A validated User-Agent header value accepted from Ruby.
pub struct UserAgent(pub HeaderValue);

/// Mutable HTTP headers exposed as `Wreq::Headers`.
///
/// Names are stored in their normalized form and lookups are case-insensitive.
/// A name can have multiple values, each counted as a separate occurrence.
#[derive(Clone, Default)]
#[magnus::wrap(class = "Wreq::Headers", free_immediately, size)]
pub struct Headers(pub RefCell<HeaderMap>);

/// Header casing and order supplied through the `orig_headers` request option.
pub struct OrigHeaders(pub OrigHeaderMap);

// ===== impl UserAgent =====

impl TryConvert for UserAgent {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let s = RString::try_convert(value)?;
        HeaderValue::from_maybe_shared(s.to_bytes())
            .map(Self)
            .map_err(header_value_error_to_magnus)
    }
}

// ===== impl Headers =====

impl Headers {
    /// Return the first value for a String or Symbol header name.
    ///
    /// Returns `nil` when the normalized name is not present.
    pub fn get(&self, name: Value) -> Result<Option<Bytes>, Error> {
        let name = parse_header_name(name)?;
        Ok(self.0.borrow().get(name).cloned().map(Bytes::from_owner))
    }

    /// Return every value for a String or Symbol header name.
    ///
    /// Values retain their append order. A missing name returns an empty Array.
    pub fn get_all(ruby: &Ruby, rb_self: &Self, name: Value) -> Result<RArray, Error> {
        let name = parse_header_name(name)?;
        let headers = rb_self.0.borrow();
        let values = headers.get_all(name).iter().cloned().map(Bytes::from_owner);
        Ok(ruby.ary_from_iter(values))
    }

    /// Replace every value for a header name.
    ///
    /// A String stores one occurrence, while an Array stores each String as a
    /// separate occurrence. An empty Array removes the header.
    pub fn set(&self, name: Value, value: Value) -> Result<(), Error> {
        let name = parse_header_name(name)?;
        let values = parse_header_values(value)?;
        let mut headers = self.0.borrow_mut();
        let replaced = headers.get_all(&name).iter().count();
        ensure_header_count(headers.len(), replaced, values.len())?;

        let mut values = values.into_iter();
        let Some(first) = values.next() else {
            headers.remove(name);
            return Ok(());
        };

        headers
            .try_insert(name.clone(), first)
            .map_err(|_| header_count_error())?;
        for value in values {
            headers
                .try_append(name.clone(), value)
                .map_err(|_| header_count_error())?;
        }
        Ok(())
    }

    /// Append one or more values without replacing existing occurrences.
    ///
    /// Array elements are appended separately and are never comma-folded.
    pub fn append(&self, name: Value, value: Value) -> Result<(), Error> {
        let name = parse_header_name(name)?;
        let values = parse_header_values(value)?;
        let mut headers = self.0.borrow_mut();
        ensure_header_count(headers.len(), 0, values.len())?;

        for value in values {
            headers
                .try_append(name.clone(), value)
                .map_err(|_| header_count_error())?;
        }
        Ok(())
    }

    /// Remove every value for a header name and return its first value.
    ///
    /// Returns `nil` when the normalized name is not present.
    pub fn remove(&self, name: Value) -> Result<Option<Bytes>, Error> {
        let name = parse_header_name(name)?;
        Ok(self.0.borrow_mut().remove(name).map(Bytes::from_owner))
    }

    /// Return whether a String or Symbol header name is present.
    pub fn contains(&self, name: Value) -> Result<bool, Error> {
        let name = parse_header_name(name)?;
        Ok(self.0.borrow().contains_key(name))
    }

    /// Return the total number of header occurrences.
    ///
    /// This can be greater than `keys.length` when names have multiple values.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Return whether the collection contains no header occurrences.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    /// Return each unique normalized header name.
    pub fn keys(ruby: &Ruby, rb_self: &Self) -> RArray {
        ruby.ary_from_iter(rb_self.0.borrow().keys().cloned().map(Bytes::from_owner))
    }

    /// Return all values, including duplicate-name occurrences.
    #[inline]
    pub fn values(ruby: &Ruby, rb_self: &Self) -> RArray {
        ruby.ary_from_iter(rb_self.0.borrow().values().cloned().map(Bytes::from_owner))
    }

    /// Return the debug representation of the underlying header map.
    #[inline]
    pub fn to_s(&self) -> String {
        self.0.borrow().inspect()
    }
}

// Ruby collection adapters are kept separate from the core HeaderMap operations.
impl Headers {
    /// Create an empty collection or populate it from a Ruby source.
    ///
    /// The optional source may be a Hash, another `Wreq::Headers`, or an
    /// Enumerable whose elements are name-value pairs.
    pub fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        match args {
            [] => Ok(Self::default()),
            [source] => from_source(*source),
            _ => Err(Error::new(
                ruby.exception_arg_error(),
                format!(
                    "wrong number of arguments (given {}, expected 0..1)",
                    args.len()
                ),
            )),
        }
    }

    /// Return a value using Ruby collection semantics.
    ///
    /// A missing name returns `nil`, one occurrence returns a String, and
    /// multiple occurrences return an Array of Strings.
    pub fn index(ruby: &Ruby, rb_self: &Self, name: Value) -> Result<Value, Error> {
        let name = parse_header_name(name)?;
        let headers = rb_self.0.borrow();
        let all_values = headers.get_all(name);
        let mut values = all_values.iter();

        let Some(first) = values.next() else {
            return Ok(ruby.qnil().as_value());
        };
        let Some(second) = values.next() else {
            return Ok(ruby.into_value(Bytes::from_owner(first.clone())));
        };

        let values = std::iter::once(first)
            .chain(std::iter::once(second))
            .chain(values)
            .cloned()
            .map(Bytes::from_owner);
        Ok(ruby.into_value(ruby.ary_from_iter(values)))
    }

    /// Replace a header and return the assigned Ruby value for `headers[name] = value`.
    pub fn set_index(&self, name: Value, value: Value) -> Result<Value, Error> {
        self.set(name, value)?;
        Ok(value)
    }

    /// Remove every occurrence and return the same `Wreq::Headers` object.
    pub fn clear(rb_self: Value) -> Result<Value, Error> {
        let headers = Obj::<Headers>::try_convert(rb_self)?;
        headers.0.borrow_mut().clear();
        Ok(rb_self)
    }

    /// Yield every normalized name-value occurrence.
    ///
    /// Returns an Enumerator without a block and returns the collection after
    /// yielding when a block is provided.
    pub fn each(ruby: &Ruby, rb_self: Value) -> Result<Value, Error> {
        if !ruby.block_given() {
            return Ok(ruby.into_value(rb_self.enumeratorize("each", ())));
        }

        let headers = Obj::<Headers>::try_convert(rb_self)?;
        // Release the RefCell borrow before yielding because Ruby code may
        // mutate this collection from inside the block.
        let entries: Vec<_> = headers
            .0
            .borrow()
            .iter()
            .map(|(name, value)| {
                (
                    Bytes::from_owner(name.clone()),
                    Bytes::from_owner(value.clone()),
                )
            })
            .collect();
        for (name, value) in entries {
            let _: Value = ruby.yield_values((name, value))?;
        }
        Ok(rb_self)
    }
}

impl From<HeaderMap> for Headers {
    fn from(headers: HeaderMap) -> Self {
        Self(RefCell::new(headers))
    }
}

impl TryConvert for Headers {
    fn try_convert(value: Value) -> Result<Self, Error> {
        from_source(value)
    }
}

// ===== impl OrigHeaders =====

impl TryConvert for OrigHeaders {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let mut map = OrigHeaderMap::new();

        let rarray = RArray::from_value(value)
            .ok_or_else(|| type_value_error_to_magnus("Expected an array of strings"))?;

        for value in rarray.into_iter().flat_map(RString::from_value) {
            map.insert(value.to_bytes());
        }

        Ok(Self(map))
    }
}

mod helper {
    //! Ruby value conversion helpers for `Wreq::Headers`.

    use bytes::Bytes;
    use http::{HeaderName, HeaderValue};
    use magnus::{Error, RArray, RString, Symbol, TryConvert, Value, prelude::*, typed_data::Obj};

    use crate::error::{
        header_name_error_to_magnus, header_value_error_to_magnus, type_value_error_to_magnus,
    };

    use super::Headers;

    /// Maximum number of field-value occurrences supported by `HeaderMap`.
    const MAX_HEADER_ENTRIES: usize = 1 << 15;

    /// Build a header collection from a Ruby source object.
    ///
    /// Accepts another `Wreq::Headers`, a Hash, or any object whose `to_a` result
    /// contains two-element name-value pairs. Array values are delegated to
    /// [`Headers::append`] so each value remains a separate occurrence.
    pub(super) fn from_source(source: Value) -> Result<Headers, Error> {
        if let Ok(headers) = Obj::<Headers>::try_convert(source) {
            return Ok((*headers).clone());
        }
        if !source.respond_to("to_a", false)? {
            return Err(type_value_error_to_magnus(
                "Expected Headers, a Hash, or an enumerable of pairs",
            ));
        }

        let pairs: RArray = source.funcall_public("to_a", ())?;
        let headers = Headers::default();
        for pair in pairs {
            let pair = RArray::try_convert(pair).map_err(|_| {
                type_value_error_to_magnus("Expected each header entry to be a pair")
            })?;
            if pair.len() != 2 {
                return Err(type_value_error_to_magnus(
                    "Expected each header entry to contain a name and value",
                ));
            }

            headers.append(pair.entry(0)?, pair.entry(1)?)?;
        }
        Ok(headers)
    }

    /// Convert a Ruby String or Symbol into a normalized HTTP header name.
    ///
    /// Symbol underscores are changed to hyphens before [`HeaderName`] validates
    /// and normalizes the name. Other Ruby types produce `Wreq::BuilderError`.
    pub(super) fn parse_header_name(value: Value) -> Result<HeaderName, Error> {
        let name = match (RString::from_value(value), Symbol::from_value(value)) {
            (Some(name), _) => name.to_bytes(),
            (None, Some(name)) => Bytes::from(name.name()?.replace('_', "-")),
            (None, None) => {
                return Err(type_value_error_to_magnus(
                    "Expected a String or Symbol header name",
                ));
            }
        };
        HeaderName::from_bytes(name.as_ref()).map_err(header_name_error_to_magnus)
    }

    /// Convert a Ruby String or Array of Strings into validated header values.
    ///
    /// Each Array element becomes one [`HeaderValue`]. An empty Array therefore
    /// produces no values, allowing `set` to remove a header and `append` to do
    /// nothing.
    pub(super) fn parse_header_values(value: Value) -> Result<Vec<HeaderValue>, Error> {
        if let Some(values) = RArray::from_value(value) {
            values.into_iter().map(parse_header_value).collect()
        } else {
            Ok(vec![parse_header_value(value)?])
        }
    }

    /// Convert one Ruby String into a validated HTTP header value.
    ///
    /// Invalid Ruby types and bytes rejected by [`HeaderValue`] are mapped to
    /// `Wreq::BuilderError`.
    fn parse_header_value(value: Value) -> Result<HeaderValue, Error> {
        let value = RString::try_convert(value)
            .map_err(|_| type_value_error_to_magnus("Expected a String header value"))?;
        HeaderValue::from_maybe_shared(value.to_bytes()).map_err(header_value_error_to_magnus)
    }

    /// Validate the resulting number of header occurrences before a mutation.
    ///
    /// `current` is the collection length, `replaced` is the number of existing
    /// occurrences removed by `set`, and `added` is the incoming value count.
    /// Checked arithmetic prevents overflow; an invalid calculation or a result
    /// above the native [`HeaderMap`](http::HeaderMap) limit returns
    /// `Wreq::BuilderError` without mutating the collection.
    pub(super) fn ensure_header_count(
        current: usize,
        replaced: usize,
        added: usize,
    ) -> Result<(), Error> {
        let count = current
            .checked_sub(replaced)
            .and_then(|count| count.checked_add(added));
        if count.is_some_and(|count| count <= MAX_HEADER_ENTRIES) {
            Ok(())
        } else {
            Err(header_count_error())
        }
    }

    /// Build the error returned when the native header map reaches its entry limit.
    pub(super) fn header_count_error() -> Error {
        type_value_error_to_magnus("Header collection exceeds 32,768 entries")
    }
}

/// Register `Wreq::Headers` and its native methods with Ruby.
pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let headers_class = gem_module.define_class("Headers", ruby.class_object())?;

    // Core bindings expose direct HeaderMap operations.
    headers_class.define_method("get", method!(Headers::get, 1))?;
    headers_class.define_method("get_all", method!(Headers::get_all, 1))?;
    headers_class.define_method("set", method!(Headers::set, 2))?;
    headers_class.define_method("append", method!(Headers::append, 2))?;
    headers_class.define_method("remove", method!(Headers::remove, 1))?;
    headers_class.define_method("contains?", method!(Headers::contains, 1))?;
    headers_class.define_method("key?", method!(Headers::contains, 1))?;
    headers_class.define_method("length", method!(Headers::len, 0))?;
    headers_class.define_method("empty?", method!(Headers::is_empty, 0))?;
    headers_class.define_method("keys", method!(Headers::keys, 0))?;
    headers_class.define_method("values", method!(Headers::values, 0))?;
    headers_class.define_method("to_s", method!(Headers::to_s, 0))?;

    // Ruby collection bindings cover construction, indexing, and block semantics.
    headers_class.include_module(ruby.module_enumerable())?;
    headers_class.define_singleton_method("new", function!(Headers::new, -1))?;
    headers_class.define_method("[]", method!(Headers::index, 1))?;
    headers_class.define_method("[]=", method!(Headers::set_index, 2))?;
    headers_class.define_method("clear", method!(Headers::clear, 0))?;
    headers_class.define_method("each", method!(Headers::each, 0))?;
    Ok(())
}
