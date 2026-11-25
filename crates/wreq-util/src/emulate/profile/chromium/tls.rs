use super::super::{
    AlpsProtocol, BrotliCompressor, CertificateCompressor, EchGrease, EchGreaseAead, Tls13Cipher,
    TlsOptions, TlsVersion,
};

#[derive(Clone, Copy)]
pub(super) enum Preset {
    ClassicGroups,
    EchGrease,
    PermutedExtensions,
    PermutedExtensionsWithEchGrease,
    Resumption,
    ResumptionWithKyber,
    ResumptionWithMlkem,
    MlkemNewAlps,
    MlkemNewAlpsWithMldsa,
    ShuffledTrustAnchorIds,
    SortedTrustAnchorIds,
    EdgeSignatureGrease,
}

struct Settings {
    curves: &'static str,
    sigalgs: &'static str,
    permute_extensions: bool,
    ech_grease: bool,
    pre_shared_key: bool,
    new_alps: bool,
    grease_sigalgs: Option<bool>,
    trust_anchors: Option<TrustAnchorOrder>,
}

enum TrustAnchorOrder {
    Shuffled,
    Sorted,
}

impl Preset {
    pub(super) fn build(self) -> TlsOptions {
        const CLASSIC: Settings = Settings {
            curves: CLASSIC_X25519_GROUPS,
            sigalgs: SIGALGS_LIST,
            permute_extensions: false,
            ech_grease: false,
            pre_shared_key: false,
            new_alps: false,
            grease_sigalgs: None,
            trust_anchors: None,
        };

        const PERMUTED_ECH: Settings = Settings {
            permute_extensions: true,
            ech_grease: true,
            ..CLASSIC
        };

        const RESUMPTION: Settings = Settings {
            pre_shared_key: true,
            ..PERMUTED_ECH
        };

        const MLKEM_NEW_ALPS: Settings = Settings {
            curves: MLKEM_GROUPS,
            new_alps: true,
            ..RESUMPTION
        };

        const MLDSA: Settings = Settings {
            sigalgs: SIGALGS_WITH_MLDSA,
            ..MLKEM_NEW_ALPS
        };

        const SIGNATURE_GREASE: Settings = Settings {
            grease_sigalgs: Some(true),
            ..MLDSA
        };

        let settings = match self {
            Self::ClassicGroups => CLASSIC,
            Self::EchGrease => Settings {
                ech_grease: true,
                ..CLASSIC
            },
            Self::PermutedExtensions => Settings {
                permute_extensions: true,
                ..CLASSIC
            },
            Self::PermutedExtensionsWithEchGrease => PERMUTED_ECH,
            Self::Resumption => RESUMPTION,
            Self::ResumptionWithKyber => Settings {
                curves: KYBER_DRAFT_GROUPS,
                ..RESUMPTION
            },
            Self::ResumptionWithMlkem => Settings {
                curves: MLKEM_GROUPS,
                ..RESUMPTION
            },
            Self::MlkemNewAlps => MLKEM_NEW_ALPS,
            Self::MlkemNewAlpsWithMldsa => MLDSA,
            Self::ShuffledTrustAnchorIds => Settings {
                trust_anchors: Some(TrustAnchorOrder::Shuffled),
                ..SIGNATURE_GREASE
            },
            Self::SortedTrustAnchorIds => Settings {
                trust_anchors: Some(TrustAnchorOrder::Sorted),
                ..SIGNATURE_GREASE
            },
            Self::EdgeSignatureGrease => SIGNATURE_GREASE,
        };

        let builder = TlsOptions::builder()
            .grease_enabled(true)
            .enable_ocsp_stapling(true)
            .enable_signed_cert_timestamps(true)
            .curves_list(settings.curves)
            .sigalgs_list(settings.sigalgs)
            .cipher_list(CIPHER_LIST)
            .tls13_cipher_list(TLS13_CIPHER_LIST)
            .min_tls_version(TlsVersion::TLS_1_2)
            .max_tls_version(TlsVersion::TLS_1_3)
            .permute_extensions(settings.permute_extensions)
            .pre_shared_key(settings.pre_shared_key)
            .ech_grease(
                settings
                    .ech_grease
                    .then_some(EchGrease::Aead(EchGreaseAead::AES_128_GCM)),
            )
            .alps_protocols([AlpsProtocol::HTTP2])
            .alps_use_new_codepoint(settings.new_alps)
            .certificate_compressors(CERTIFICATE_COMPRESSORS)
            .grease_sigalgs_enabled(settings.grease_sigalgs);

        // Skipping this setter omits the extension; an empty list would still send it.
        let builder = match settings.trust_anchors {
            Some(TrustAnchorOrder::Shuffled) => {
                builder.trust_anchors(shuffled_trust_anchors(crate::rand::fast_random))
            }
            Some(TrustAnchorOrder::Sorted) => builder.trust_anchors(sorted_trust_anchors()),
            None => builder,
        };
        builder.build()
    }
}

const CLASSIC_X25519_GROUPS: &str = join!(":", "X25519", "P-256", "P-384");
const KYBER_DRAFT_GROUPS: &str = join!(":", "X25519Kyber768Draft00", "X25519", "P-256", "P-384");
const MLKEM_GROUPS: &str = join!(":", "X25519MLKEM768", "X25519", "P-256", "P-384");

pub const CIPHER_LIST: &str = join!(
    ":",
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_RSA_WITH_AES_128_CBC_SHA",
    "TLS_RSA_WITH_AES_256_CBC_SHA"
);

pub const TLS13_CIPHER_LIST: &[Tls13Cipher] = &[
    Tls13Cipher::AES_128_GCM_SHA256,
    Tls13Cipher::AES_256_GCM_SHA384,
    Tls13Cipher::CHACHA20_POLY1305_SHA256,
];

pub const SIGALGS_LIST: &str = join!(
    ":",
    "ecdsa_secp256r1_sha256",
    "rsa_pss_rsae_sha256",
    "rsa_pkcs1_sha256",
    "ecdsa_secp384r1_sha384",
    "rsa_pss_rsae_sha384",
    "rsa_pkcs1_sha384",
    "rsa_pss_rsae_sha512",
    "rsa_pkcs1_sha512"
);

const SIGALGS_WITH_MLDSA: &str = join!(
    ":",
    "mldsa44",
    "mldsa65",
    "mldsa87",
    "ecdsa_secp256r1_sha256",
    "rsa_pss_rsae_sha256",
    "rsa_pkcs1_sha256",
    "ecdsa_secp384r1_sha384",
    "rsa_pss_rsae_sha384",
    "rsa_pkcs1_sha384",
    "rsa_pss_rsae_sha512",
    "rsa_pkcs1_sha512"
);

// Encoded IDs and wreq's Chromium root store come from the same root set.
pub(super) const CHROME_TRUST_ANCHORS: &[u8] = &chromium_roots::encoded_trust_anchor_ids();

// Chrome 152/153 iterate a salted flat_hash_set held by SSLClientContext, rather than
// the root store's source order. Generate an order once per native configuration;
// connections using that client configuration retain the order.
// https://chromium.googlesource.com/chromium/src/+/f7b831a4e249fbd98eae4b391c69f70300a849d1/net/ssl/ssl_config_service.cc
fn shuffled_trust_anchors(mut random: impl FnMut() -> u64) -> Vec<u8> {
    let mut ids = chromium_roots::trust_anchor_ids();
    for index in (1..ids.len()).rev() {
        let bound = (index + 1) as u64;
        let limit = u64::MAX - u64::MAX % bound;
        let mut value = random();
        while value >= limit {
            value = random();
        }
        ids.swap(index, (value % bound) as usize);
    }

    encode_trust_anchors(&ids)
}

// Chrome 154 sorts the raw IDs before adding their length prefixes.
// https://chromium.googlesource.com/chromium/src/+/731082f0a26ce4b3976c3d82943092f5d13daf13/net/cert/x509_util.cc
fn sorted_trust_anchors() -> Vec<u8> {
    let mut ids = chromium_roots::trust_anchor_ids();
    ids.sort_unstable();
    encode_trust_anchors(&ids)
}

fn encode_trust_anchors(ids: &[&[u8]]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(CHROME_TRUST_ANCHORS.len());
    for id in ids {
        // encoded_trust_anchor_ids() validates these same IDs at compile time:
        // each length is nonzero and fits in one byte.
        encoded.push(id.len() as u8);
        encoded.extend_from_slice(id);
    }
    encoded
}

pub const CERTIFICATE_COMPRESSORS: &[&'static dyn CertificateCompressor] = &[&BrotliCompressor];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_trust_anchors_match_chromium_root_store() {
        let encoded = sorted_trust_anchors();
        let mut expected = chromium_roots::trust_anchor_ids();
        expected.sort_unstable();
        assert_eq!(decode_ids(&encoded), expected);
        assert_eq!(encoded.len(), CHROME_TRUST_ANCHORS.len());
    }

    #[test]
    fn trust_anchor_shuffle_preserves_ids_without_pinning_the_first() {
        // Choosing index zero at every step rotates the entire list left.
        // Rejecting u64::MAX also covers the unbiased sampling boundary.
        let mut samples = std::iter::once(u64::MAX).chain(std::iter::repeat(0));
        let encoded = shuffled_trust_anchors(|| samples.next().unwrap());
        let decoded = decode_ids(&encoded);
        let mut expected = chromium_roots::trust_anchor_ids();
        expected.rotate_left(1);
        assert_eq!(decoded, expected);
        assert_eq!(encoded.len(), CHROME_TRUST_ANCHORS.len());
    }

    fn decode_ids(mut remaining: &[u8]) -> Vec<&[u8]> {
        let mut decoded = Vec::new();
        while let Some((&length, rest)) = remaining.split_first() {
            assert_ne!(length, 0);
            let (id, rest) = rest.split_at(usize::from(length));
            decoded.push(id);
            remaining = rest;
        }
        decoded
    }
}
