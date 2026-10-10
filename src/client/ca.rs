//! Custom CA trust-store resolution for [`super::Client`] construction.

use magnus::Ruby;

use crate::error::argument_error;

use super::Builder;

/// PEM material used to build a custom [`wreq::tls::trust::CertStore`].
pub(super) struct CaPem {
    /// PEM-encoded certificate bytes (one cert or a bundle).
    pub(super) pem: Vec<u8>,
    /// When `true`, keep system roots and add these certs.
    /// When `false`, replace the default trust store.
    pub(super) augment: bool,
}

/// Resolve the mutually exclusive CA client options into PEM bytes.
///
/// Reads `*_file` options here (with the GVL held) so missing/unreadable
/// files raise `ArgumentError` before native client construction.
pub(super) fn resolve(ruby: &Ruby, params: &mut Builder) -> Result<Option<CaPem>, magnus::Error> {
    let mut pem: Option<Vec<u8>> = None;
    let mut augment_flag: Option<bool> = None;

    // ---- path options: ca_file / additional_ca_file ----
    if let Some((path, additional)) = params
        .ca_file
        .take()
        .map(|path| (path, false))
        .or_else(|| params.additional_ca_file.take().map(|path| (path, true)))
    {
        let name = if additional {
            "additional_ca_file"
        } else {
            "ca_file"
        };
        pem =
            Some(std::fs::read(&path).map_err(|_| {
                argument_error(ruby, format!("{name}: cannot read certificate file"))
            })?);
        augment_flag = Some(additional);
    }

    // ---- string options: ca_pem / additional_ca_pem ----
    if let Some((value, additional)) = params
        .ca_pem
        .take()
        .map(|value| (value, false))
        .or_else(|| params.additional_ca_pem.take().map(|value| (value, true)))
    {
        pem = Some(value.into_bytes());
        augment_flag = Some(additional);
    }

    if let (Some(pem), Some(augment)) = (pem, augment_flag) {
        return Ok(Some(CaPem { pem, augment }));
    }

    Ok(None)
}

/// Attach a custom certificate store to the native client builder.
pub(super) fn into_cert_store(ca: CaPem) -> wreq::Result<wreq::tls::trust::CertStore> {
    let mut store_builder = wreq::tls::trust::CertStore::builder();
    if ca.augment {
        store_builder = store_builder.set_default_paths();
    }
    store_builder.add_stack_pem_certs(&ca.pem).build()
}
