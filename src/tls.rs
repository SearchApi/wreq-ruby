//! Ruby wrappers for TLS metadata attached to a response.
//!
//! Certificates use the DER encoding described by the X.509 profile in
//! [RFC 5280 section 4.1](https://www.rfc-editor.org/rfc/rfc5280#section-4.1).

use magnus::{Error, Module, RArray, RModule, RString, Ruby, value::ReprValue};

/// Read-only Ruby wrapper around [`wreq::tls::TlsInfo`].
///
/// The native value keeps certificate bytes alive independently of the response
/// body. Its `Bytes` buffers are cheap to clone, while accessors copy the data
/// into Ruby-owned Strings so callers cannot mutate the stored metadata.
#[derive(Clone)]
#[magnus::wrap(class = "Wreq::TlsInfo", free_immediately, size)]
pub(crate) struct TlsInfo(pub(crate) wreq::tls::TlsInfo);

impl TlsInfo {
    /// Copy the DER-encoded leaf certificate into a binary Ruby String.
    fn peer_certificate(ruby: &Ruby, rb_self: &Self) -> Option<RString> {
        rb_self
            .0
            .peer_certificate()
            .map(|der| ruby.str_from_slice(der))
    }

    /// Copy the certificate chain into a frozen Array of binary Ruby Strings.
    ///
    /// Only the Array is frozen. Its Strings are independent copies and remain
    /// mutable in Ruby.
    fn peer_certificate_chain(ruby: &Ruby, rb_self: &Self) -> Option<RArray> {
        rb_self.0.peer_certificate_chain().map(|chain| {
            let certificates = ruby.ary_from_iter(chain.map(|cert| ruby.str_from_slice(cert)));
            certificates.freeze();
            certificates
        })
    }
}

/// Define the `Wreq::TlsInfo` Ruby class and its readers.
pub(crate) fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), Error> {
    let tls_info_class = gem_module.define_class("TlsInfo", ruby.class_object())?;
    tls_info_class.define_method(
        "peer_certificate",
        magnus::method!(TlsInfo::peer_certificate, 0),
    )?;
    tls_info_class.define_method(
        "peer_certificate_chain",
        magnus::method!(TlsInfo::peer_certificate_chain, 0),
    )?;
    Ok(())
}
