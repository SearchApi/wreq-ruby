use std::{fmt, sync::Arc, time::SystemTime};

use cookie::{Cookie as RawCookie, Expiration, ParseError, time::Duration};
use magnus::{
    Error, Module, Object, RModule, Ruby, Value, function, method, typed_data::Obj,
    value::ReprValue,
};
use wreq::{
    Uri,
    cookie::CookieStore,
    header::{self, HeaderMap, HeaderValue},
};

use crate::nogvl;

define_ruby_enum!(
    /// The Cookie SameSite attribute.
    const,
    SameSite,
    "Wreq::SameSite",
    cookie::SameSite,
    (Strict, Strict),
    (Lax, Lax),
    (Empty, None),
);

/// A single HTTP cookie.
#[derive(Clone)]
#[magnus::wrap(class = "Wreq::Cookie", free_immediately, size)]
pub struct Cookie(RawCookie<'static>);

/// A good default `CookieStore` implementation.
///
/// This is the implementation used when simply calling `cookie_store(true)`.
/// This type is exposed to allow creating one and filling it with some
/// existing cookies more easily, before creating a `Client`.
#[derive(Clone, Default)]
#[magnus::wrap(class = "Wreq::Jar", free_immediately, size)]
pub struct Jar(Arc<wreq::cookie::Jar>);

// ===== impl Cookie =====

impl Cookie {
    /// Create a new [`Cookie`].
    pub fn new(args: &[Value]) -> Result<Self, Error> {
        let args =
            magnus::scan_args::scan_args::<(String, String), (), (), (), magnus::RHash, ()>(args)?;
        #[allow(clippy::type_complexity)]
        let keywords: magnus::scan_args::KwArgs<
            (),
            (
                Option<String>,
                Option<String>,
                Option<u64>,
                Option<f64>,
                Option<bool>,
                Option<bool>,
                Option<Obj<SameSite>>,
            ),
            (),
        > = magnus::scan_args::get_kwargs(
            args.keywords,
            &[],
            &[
                "domain",
                "path",
                "max_age",
                "expires",
                "http_only",
                "secure",
                "same_site",
            ],
        )?;

        let (name, value) = args.required;

        let mut cookie = RawCookie::new(name, value);

        if let Some(domain) = keywords.optional.0 {
            cookie.set_domain(domain);
        }

        if let Some(path) = keywords.optional.1 {
            cookie.set_path(path);
        }

        if let Some(max_age) = keywords.optional.2 {
            cookie.set_max_age(Duration::seconds(max_age as i64));
        }

        if let Some(expires) = keywords.optional.3 {
            let duration = std::time::Duration::from_secs_f64(expires);
            if let Some(system_time) = SystemTime::UNIX_EPOCH.checked_add(duration) {
                cookie.set_expires(Expiration::DateTime(system_time.into()));
            }
        }

        cookie.set_http_only(keywords.optional.4);
        cookie.set_secure(keywords.optional.5);

        if let Some(same_site) = keywords.optional.6 {
            cookie.set_same_site(same_site.into_ffi());
        }

        Ok(Self(cookie))
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

    /// Returns true if  'SameSite' directive is 'Lax'.
    #[inline]
    pub fn same_site_lax(&self) -> bool {
        self.0.same_site() == Some(cookie::SameSite::Lax)
    }

    /// Returns true if  'SameSite' directive is 'Strict'.
    #[inline]
    pub fn same_site_strict(&self) -> bool {
        self.0.same_site() == Some(cookie::SameSite::Strict)
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

    /// Get the Max-Age information.
    #[inline]
    pub fn max_age(&self) -> Option<i64> {
        self.0.max_age().map(|d| d.whole_seconds())
    }

    /// The cookie expiration time.
    #[inline]
    pub fn expires(&self) -> Option<f64> {
        match self.0.expires() {
            Some(Expiration::DateTime(offset)) => {
                let system_time = SystemTime::from(offset);
                system_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs_f64())
            }
            None | Some(Expiration::Session) => None,
        }
    }
}

impl Cookie {
    /// Parse cookies from a `HeaderMap`.
    pub fn extract_headers_cookies(headers: &HeaderMap) -> Vec<Cookie> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .map(Cookie::parse)
            .flat_map(Result::ok)
            .map(RawCookie::into_owned)
            .map(Cookie)
            .collect()
    }

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

// ===== impl Jar =====

impl CookieStore for Jar {
    #[inline]
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, uri: &Uri) {
        self.0.set_cookies(cookie_headers, uri);
    }

    #[inline]
    fn cookies(&self, uri: &Uri) -> Vec<HeaderValue> {
        self.0.cookies(uri)
    }
}

impl Jar {
    /// Create a new [`Jar`] with an empty cookie store.
    pub fn new() -> Self {
        Jar(Arc::new(wreq::cookie::Jar::default()))
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
    pub fn add_cookie(&self, cookie: &Cookie, url: String) {
        nogvl::nogvl(|| self.0.add_cookie(cookie.0.clone(), &url))
    }

    /// Add a cookie str to this jar.
    pub fn add_cookie_str(&self, cookie: String, url: String) {
        nogvl::nogvl(|| self.0.add_cookie_str(&cookie, &url))
    }

    /// Remove a cookie from this jar by name and URL.
    pub fn remove(&self, name: String, url: String) {
        nogvl::nogvl(|| self.0.remove(name, &url))
    }

    /// Clear all cookies in this jar.
    pub fn clear(&self) {
        nogvl::nogvl(|| self.0.clear())
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // SameSite enum
    let same_site_class = gem_module.define_class("SameSite", ruby.class_object())?;
    same_site_class.const_set("Strict", SameSite::Strict)?;
    same_site_class.const_set("Lax", SameSite::Lax)?;
    same_site_class.const_set("Empty", SameSite::Empty)?;

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
    cookie_class.define_method("path", method!(Cookie::path, 0))?;
    cookie_class.define_method("domain", method!(Cookie::domain, 0))?;
    cookie_class.define_method("max_age", method!(Cookie::max_age, 0))?;
    cookie_class.define_method("expires", method!(Cookie::expires, 0))?;

    // Jar class
    let jar_class = gem_module.define_class("Jar", ruby.class_object())?;
    jar_class.define_singleton_method("new", function!(Jar::new, 0))?;
    jar_class.define_method("get_all", method!(Jar::get_all, 0))?;
    jar_class.define_method("add_cookie", method!(Jar::add_cookie, 2))?;
    jar_class.define_method("add_cookie_str", method!(Jar::add_cookie_str, 2))?;
    jar_class.define_method("remove", method!(Jar::remove, 2))?;
    jar_class.define_method("clear", method!(Jar::clear, 0))?;

    Ok(())
}
