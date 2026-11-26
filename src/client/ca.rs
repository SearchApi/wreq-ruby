use std::{fs, path::PathBuf};

use magnus::{Error, Ruby};
use wreq::tls::trust::CertStore;

use super::Builder;
use crate::{error::argument_error, gvl};

pub(super) enum CaBundle {
    Replace(Vec<u8>),
    Additional(Vec<u8>),
}

impl Builder {
    /// Consume the CA option selected by `from_options`. File reads release the GVL.
    ///
    /// # Errors
    ///
    /// Returns `ArgumentError` if the selected file cannot be read.
    pub(super) fn take_ca_bundle(&mut self, ruby: &Ruby) -> Result<Option<CaBundle>, Error> {
        let bundle = if let Some(path) = self.ca_file.take() {
            CaBundle::Replace(read_ca_file(ruby, path, stringify!(ca_file))?)
        } else if let Some(pem) = self.ca_pem.take() {
            CaBundle::Replace(pem.into_bytes())
        } else if let Some(path) = self.additional_ca_file.take() {
            CaBundle::Additional(read_ca_file(ruby, path, stringify!(additional_ca_file))?)
        } else if let Some(pem) = self.additional_ca_pem.take() {
            CaBundle::Additional(pem.into_bytes())
        } else {
            return Ok(None);
        };
        Ok(Some(bundle))
    }
}

impl CaBundle {
    /// Build the trust store on the native side of the GVL boundary.
    ///
    /// # Errors
    ///
    /// Returns a native error if loading the certificate paths or PEM bundle fails.
    pub(super) fn into_cert_store(self) -> wreq::Result<CertStore> {
        let builder = CertStore::builder();
        let (builder, pem) = match self {
            Self::Replace(pem) => (builder, pem),
            Self::Additional(pem) => (builder.set_default_paths(), pem),
        };
        builder.add_stack_pem_certs(pem).build()
    }
}

fn read_ca_file(ruby: &Ruby, path: PathBuf, option: &str) -> Result<Vec<u8>, Error> {
    gvl::nogvl(|| fs::read(path))
        .map_err(|_| argument_error(ruby, format!("{option}: cannot read certificate file")))
}
