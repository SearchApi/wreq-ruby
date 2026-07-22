//! Ruby bindings for HTTP cookies and the shared cookie jar.
//!
//! Cookie expiration follows [RFC 6265]: Ruby `Time` values and finite Unix
//! timestamps are converted into signed native date-times, while non-positive
//! Max-Age values represent immediate expiration.
//!
//! [RFC 6265]: https://www.rfc-editor.org/rfc/rfc6265.html

use std::{fmt, sync::Arc};

use ::serde::Deserialize;
use bytes::Bytes;
use cookie::{Cookie as RawCookie, ParseError, time::Duration};
use magnus::{
    Error, Module, Object, RHash, RModule, RString, Ruby, Time, TryConvert, Value, function,
    method, r_hash::ForEach, typed_data::Obj, value::ReprValue,
};
use wreq::header::{self, HeaderMap, HeaderValue};

use crate::{
    error::{header_value_error, type_error},
    gvl,
    options::{NativeOption, Options},
};

use self::helper::{CookieExpiration, to_ruby_time, to_unix_timestamp};

// Defines constant registration, `into_ffi`/`from_ffi`, and handlers for
// Ruby's `to_s`, `to_sym`, `==`, `eql?`, and `hash` methods.
define_ruby_enum!(
    /// The Cookie SameSite attribute.
    SameSite,
    "Wreq::SameSite",
    cookie::SameSite,
    symbols:
    Strict => "strict",
    Lax => "lax",
    None => "none",
);

/// A single HTTP cookie.
#[derive(Clone)]
#[magnus::wrap(class = "Wreq::Cookie", free_immediately, size)]
pub struct Cookie(RawCookie<'static>);

/// A collection of HTTP cookies.
#[derive(Default)]
pub struct Cookies(pub Vec<HeaderValue>);

/// Keyword attributes used to build a [`Cookie`].
#[derive(Deserialize)]
struct Builder {
    /// The Domain attribute.
    domain: Option<String>,

    /// The Path attribute.
    path: Option<String>,

    /// The signed Max-Age in seconds.
    #[serde(default)]
    max_age: NativeOption<i64>,

    /// The absolute expiration accepted through Magnus conversion.
    #[serde(default)]
    expires: NativeOption<CookieExpiration>,

    /// Whether the cookie is inaccessible to client-side scripts.
    http_only: Option<bool>,

    /// Whether the cookie is restricted to secure connections.
    secure: Option<bool>,

    /// The SameSite policy.
    #[serde(default)]
    same_site: NativeOption<Obj<SameSite>>,
}

/// A good default `CookieStore` implementation.
///
/// This is the implementation used when simply calling `cookie_store(true)`.
/// This type is exposed to allow creating one and filling it with some
/// existing cookies more easily, before creating a `Client`.
#[derive(Clone, Default)]
#[magnus::wrap(class = "Wreq::Jar", free_immediately, size)]
pub struct Jar(pub Arc<wreq::cookie::Jar>);

// ===== impl Builder =====

impl Builder {
    /// Deserialize and convert one validated Cookie attribute Hash.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for unknown or duplicate attributes and retains
    /// the Ruby conversion error for invalid values.
    fn from_options(options: Options<'_>) -> Result<Self, Error> {
        let mut builder = options.validate_keys::<Self>()?.deserialize::<Self>()?;
        extract_native_option!(options, builder, max_age);
        extract_native_option!(options, builder, expires);
        extract_native_option!(options, builder, same_site);
        Ok(builder)
    }

    /// Build a native Cookie from its required identity and optional attributes.
    fn build(mut self, name: String, value: String) -> Cookie {
        let mut cookie = RawCookie::new(name, value);

        if let Some(domain) = self.domain {
            cookie.set_domain(domain);
        }

        if let Some(path) = self.path {
            cookie.set_path(path);
        }

        if let Some(max_age) = self.max_age.take() {
            cookie.set_max_age(Duration::seconds(max_age));
        }

        if let Some(expires) = self.expires.take() {
            cookie.set_expires(expires.into_inner());
        }

        cookie.set_http_only(self.http_only);
        cookie.set_secure(self.secure);

        if let Some(same_site) = self.same_site.take() {
            cookie.set_same_site(same_site.into_ffi());
        }

        Cookie(cookie)
    }
}

// ===== Ruby Cookie API =====

impl Cookie {
    /// Create a new [`Cookie`] from a name, value, and keyword attributes.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` for unknown or duplicate attributes and retains
    /// the Ruby conversion error for invalid values.
    pub fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = magnus::scan_args::scan_args::<(String, String), (), (), (), RHash, ()>(args)?;
        let (name, value) = args.required;
        Builder::from_options(Options::new(ruby, args.keywords))
            .map(|builder| builder.build(name, value))
    }

    /// The name of the cookie.
    #[inline]
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The value of the cookie.
    #[inline]
    pub fn value(&self) -> &str {
        self.0.value()
    }

    /// Returns true if the 'HttpOnly' directive is enabled.
    #[inline]
    pub fn http_only(&self) -> bool {
        self.0.http_only().unwrap_or(false)
    }

    /// Returns true if the 'Secure' directive is enabled.
    #[inline]
    pub fn secure(&self) -> bool {
        self.0.secure().unwrap_or(false)
    }

    /// Return whether the SameSite directive is Lax.
    #[inline]
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site() == Some(cookie::SameSite::Lax)
    }

    /// Return whether the SameSite directive is Strict.
    #[inline]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie::SameSite::Strict)
    }

    /// Return the SameSite directive, if set.
    #[inline]
    pub fn same_site(&self) -> Option<SameSite> {
        self.0.same_site().map(SameSite::from_ffi)
    }

    /// Returns the path directive of the cookie, if set.
    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.0.path()
    }

    /// Returns the domain directive of the cookie, if set.
    #[inline]
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Return the signed Max-Age in seconds.
    #[inline]
    pub fn max_age(&self) -> Option<i64> {
        self.0.max_age().map(|d| d.whole_seconds())
    }

    /// Return the cookie expiration as a UTC Ruby `Time`.
    pub fn expires_at(ruby: &Ruby, rb_self: &Self) -> Result<Option<Time>, Error> {
        rb_self
            .0
            .expires_datetime()
            .map(|value| to_ruby_time(ruby, value))
            .transpose()
    }

    /// Return the cookie expiration as legacy fractional Unix seconds.
    #[inline]
    pub fn expires(&self) -> Option<f64> {
        self.0.expires_datetime().map(to_unix_timestamp)
    }

    /// Serialize the cookie as a Set-Cookie string.
    #[inline]
    pub fn to_s(&self) -> String {
        self.to_string()
    }
}

// ===== Native Cookie helpers =====

impl Cookie {
    /// Clone this cookie for insertion into the native jar.
    ///
    /// [RFC 6265 section 5.2.2] treats a non-positive Max-Age as immediate
    /// expiration. The native jar currently recognizes zero, so a negative
    /// value is normalized only in this insertion clone; the Ruby cookie keeps
    /// its original signed value.
    ///
    /// [RFC 6265 section 5.2.2]: https://www.rfc-editor.org/rfc/rfc6265.html#section-5.2.2
    fn clone_for_jar(&self) -> RawCookie<'static> {
        let mut cookie = self.0.clone();
        if cookie.max_age().is_some_and(Duration::is_negative) {
            cookie.set_max_age(Duration::ZERO);
        }

        cookie
    }

    /// Parse cookies from a `HeaderMap`.
    pub(crate) fn extract_headers_cookies(headers: &HeaderMap) -> Vec<Cookie> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|value| Self::parse(value).ok())
            .map(RawCookie::into_owned)
            .map(Cookie)
            .collect()
    }

    /// Parse one Set-Cookie header value.
    fn parse<'a>(value: &'a HeaderValue) -> Result<RawCookie<'a>, ParseError> {
        std::str::from_utf8(value.as_bytes())
            .map_err(cookie::ParseError::from)
            .and_then(RawCookie::parse)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ===== impl Cookies =====

impl TryConvert for Cookies {
    fn try_convert(value: magnus::Value) -> Result<Self, magnus::Error> {
        let ruby = Ruby::get_with(value);
        // try extract uncompressed cookies
        if let Some(rhash) = RHash::from_value(value) {
            let mut cookies = Vec::new();
            rhash.foreach(|name: RString, value: RString| {
                let cookie = format!("{name}={value}");
                let header_value = HeaderValue::from_maybe_shared(Bytes::from(cookie))
                    .map_err(|err| header_value_error(&ruby, err))?;
                cookies.push(header_value);
                Ok(ForEach::Continue)
            })?;

            return Ok(Self(cookies));
        }

        // try extract compressed cookies
        if let Some(cookies) = RString::from_value(value) {
            return Ok(Self(vec![
                HeaderValue::from_maybe_shared(cookies.to_bytes())
                    .map_err(|err| header_value_error(&ruby, err))?,
            ]));
        }

        Err(type_error(&ruby, "cookies must be a Hash or String"))
    }
}

// ===== impl Jar =====

impl Jar {
    /// Create a new [`Jar`] with an empty cookie store.
    pub fn new() -> Self {
        Self(Arc::new(wreq::cookie::Jar::default()))
    }

    /// Get all cookies.
    pub fn get_all(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let cookies: Vec<Cookie> = rb_self
            .0
            .get_all()
            .map(RawCookie::from)
            .map(Cookie)
            .collect();
        let ary = ruby.ary_new_capa(cookies.len());
        for cookie in cookies {
            ary.push(cookie)?;
        }
        Ok(ary.as_value())
    }

    /// Add a cookie to this jar.
    ///
    /// # Errors
    ///
    /// Returns `TypeError` when `cookie` is neither a [`Cookie`] nor a String.
    pub fn add(&self, cookie: Value, url: String) -> Result<(), Error> {
        if let Ok(cookie) = Obj::<Cookie>::try_convert(cookie) {
            gvl::nogvl(|| self.0.add(cookie.clone_for_jar(), &url));
            return Ok(());
        }

        let ruby = Ruby::get_with(cookie);
        let cookie = String::try_convert(cookie)
            .map_err(|_| type_error(&ruby, "cookie must be a Wreq::Cookie or String"))?;
        gvl::nogvl(|| self.0.add(cookie.as_ref(), &url));
        Ok(())
    }

    /// Remove a cookie from this jar by name and URL.
    pub fn remove(&self, name: String, url: String) {
        gvl::nogvl(|| self.0.remove(name, &url))
    }

    /// Clear all cookies in this jar.
    pub fn clear(&self) {
        gvl::nogvl(|| self.0.clear())
    }
}

mod helper {
    //! Ruby time conversion helpers for `Wreq::Cookie`.

    use cookie::time::{Duration, OffsetDateTime, error::ComponentRange};
    use magnus::{
        Error, Integer, Ruby, Time, TryConvert, Value,
        time::{Offset, Timespec},
    };

    use crate::error::{argument_error, range_error};

    /// A validated cookie expiration accepted from Ruby.
    ///
    /// Ruby `Time` values retain nanosecond resolution. Integer timestamps avoid a
    /// floating-point conversion, while other Numeric values are accepted as
    /// finite fractional Unix timestamps.
    pub(super) struct CookieExpiration(OffsetDateTime);

    impl CookieExpiration {
        /// Return the validated UTC expiration used by the native cookie.
        pub(super) fn into_inner(self) -> OffsetDateTime {
            self.0
        }
    }

    impl TryConvert for CookieExpiration {
        fn try_convert(value: Value) -> Result<Self, Error> {
            let ruby = Ruby::get_with(value);
            expiration_from_value(&ruby, value).map(Self)
        }
    }

    /// Convert a native cookie expiration into a UTC Ruby `Time`.
    pub(super) fn to_ruby_time(ruby: &Ruby, value: OffsetDateTime) -> Result<Time, Error> {
        ruby.time_timespec_new(
            Timespec {
                tv_sec: value.unix_timestamp(),
                tv_nsec: i64::from(value.nanosecond()),
            },
            Offset::utc(),
        )
    }

    /// Convert a native cookie expiration into legacy fractional Unix seconds.
    pub(super) fn to_unix_timestamp(value: OffsetDateTime) -> f64 {
        value.unix_timestamp() as f64 + f64::from(value.nanosecond()) / 1_000_000_000.0
    }

    /// Convert a supported Ruby expiration value into a native UTC date-time.
    fn expiration_from_value(ruby: &Ruby, value: Value) -> Result<OffsetDateTime, Error> {
        if let Some(time) = Time::from_value(value) {
            return expiration_from_time(ruby, time);
        }

        if let Some(integer) = Integer::from_value(value) {
            return integer
                .to_i64()
                .and_then(|seconds| expiration_from_seconds(ruby, seconds));
        }

        expiration_from_float(ruby, f64::try_convert(value)?)
    }

    /// Convert a Ruby `Time` without passing through unsigned `SystemTime` math.
    fn expiration_from_time(ruby: &Ruby, value: Time) -> Result<OffsetDateTime, Error> {
        let timespec = value.timespec()?;
        let nanosecond = u32::try_from(timespec.tv_nsec)
            .ok()
            .filter(|value| *value < 1_000_000_000)
            .ok_or_else(|| {
                argument_error(ruby, "time nanoseconds are outside the supported range")
            })?;

        expiration_from_seconds(ruby, timespec.tv_sec)?
            .replace_nanosecond(nanosecond)
            .map_err(|error| expiration_range_error(ruby, error))
    }

    /// Convert exact signed Unix seconds into the native cookie time range.
    fn expiration_from_seconds(ruby: &Ruby, seconds: i64) -> Result<OffsetDateTime, Error> {
        OffsetDateTime::from_unix_timestamp(seconds)
            .map_err(|error| expiration_range_error(ruby, error))
    }

    /// Convert a finite fractional Unix timestamp without using a panicking API.
    fn expiration_from_float(ruby: &Ruby, seconds: f64) -> Result<OffsetDateTime, Error> {
        if !seconds.is_finite() {
            return Err(argument_error(ruby, "timestamp must be finite"));
        }

        let duration = Duration::checked_seconds_f64(seconds)
            .ok_or_else(|| range_error(ruby, "timestamp is outside the supported range"))?;
        OffsetDateTime::UNIX_EPOCH
            .checked_add(duration)
            .ok_or_else(|| range_error(ruby, "timestamp is outside the supported range"))
    }

    /// Map the native date-time range error to Ruby's `RangeError`.
    fn expiration_range_error(ruby: &Ruby, error: ComponentRange) -> Error {
        range_error(
            ruby,
            format!("timestamp is outside the supported range: {error}"),
        )
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // SameSite enum
    let same_site_class = gem_module.define_class("SameSite", ruby.class_object())?;
    SameSite::define_constants(same_site_class)?;
    same_site_class.define_method("to_s", method!(SameSite::to_s, 0))?;
    same_site_class.define_method("to_sym", method!(SameSite::to_sym, 0))?;
    same_site_class.define_method("==", method!(SameSite::equals, 1))?;
    same_site_class.define_method("eql?", method!(SameSite::is_eql, 1))?;
    same_site_class.define_method("hash", method!(SameSite::hash_value, 0))?;

    // Cookie class
    let cookie_class = gem_module.define_class("Cookie", ruby.class_object())?;
    cookie_class.define_singleton_method("new", function!(Cookie::new, -1))?;
    cookie_class.define_method("name", method!(Cookie::name, 0))?;
    cookie_class.define_method("value", method!(Cookie::value, 0))?;
    cookie_class.define_method("http_only", method!(Cookie::http_only, 0))?;
    cookie_class.define_method("http_only?", method!(Cookie::http_only, 0))?;
    cookie_class.define_method("secure", method!(Cookie::secure, 0))?;
    cookie_class.define_method("secure?", method!(Cookie::secure, 0))?;
    cookie_class.define_method("same_site_lax?", method!(Cookie::same_site_lax, 0))?;
    cookie_class.define_method("same_site_strict?", method!(Cookie::same_site_strict, 0))?;
    cookie_class.define_method("same_site", method!(Cookie::same_site, 0))?;
    cookie_class.define_method("path", method!(Cookie::path, 0))?;
    cookie_class.define_method("domain", method!(Cookie::domain, 0))?;
    cookie_class.define_method("max_age", method!(Cookie::max_age, 0))?;
    cookie_class.define_method("expires_at", method!(Cookie::expires_at, 0))?;
    cookie_class.define_method("expires", method!(Cookie::expires, 0))?;
    cookie_class.define_method("to_s", method!(Cookie::to_s, 0))?;

    // Jar class
    let jar_class = gem_module.define_class("Jar", ruby.class_object())?;
    jar_class.define_singleton_method("new", function!(Jar::new, 0))?;
    jar_class.define_method("get_all", method!(Jar::get_all, 0))?;
    jar_class.define_method("add", method!(Jar::add, 2))?;
    jar_class.define_method("remove", method!(Jar::remove, 2))?;
    jar_class.define_method("clear", method!(Jar::clear, 0))?;

    Ok(())
}
