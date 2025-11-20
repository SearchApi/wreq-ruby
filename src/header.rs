use http::HeaderMap;
use magnus::{Error, Module, RModule, RString, Ruby};

/// Iterator for HTTP headers that yields (name, value) pairs to Ruby blocks.
///
/// This iterator is thread-safe and can be used with Ruby's `each` method.
#[magnus::wrap(class = "Wreq::HeaderIterator", free_immediately, size)]
pub struct HeaderIterator {
    headers: Vec<(String, String)>,
    index: std::sync::atomic::AtomicUsize,
}

impl HeaderIterator {
    /// Create a new header iterator from a HeaderMap.
    pub fn new(headers: &HeaderMap) -> Self {
        let headers: Vec<(String, String)> = headers
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_string(),
                    String::from_utf8_lossy(value.as_bytes()).to_string(),
                )
            })
            .collect();

        Self {
            headers,
            index: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl Iterator for HeaderIterator {
    type Item = (RString, RString);

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        if current_index < self.headers.len() {
            let (name, value) = &self.headers[current_index];
            let ruby = ruby!();
            Some((ruby.str_new(name), ruby.str_new(value)))
        } else {
            None
        }
    }
}

pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    // Also define the HeaderIterator class for completeness
    gem_module.define_class("HeaderIterator", ruby.class_object())?;

    Ok(())
}
