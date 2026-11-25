use super::super::{
    AlpnProtocol, BrotliCompressor, CertificateCompressor, EchGrease, ExtensionType, KeyShare,
    Tls13Cipher, TlsOptions, TlsVersion, ZlibCompressor, ZstdCompressor,
};

#[derive(Clone, Copy)]
pub(super) enum Preset {
    Legacy,
    EchWithoutResumption,
    MlkemResumption,
    MlkemResumptionWithSct,
    PrivateBrowsing,
    AndroidResumption,
    NoEcdsaAes128Cbc,
    NoEcdsaCbc,
    Nss3127Grease,
    NoFfdhe,
}

struct Settings {
    curves: &'static str,
    ciphers: &'static str,
    key_shares: Option<&'static [KeyShare]>,
    session_ticket: bool,
    ech_grease: Option<EchGrease>,
    signed_cert_timestamps: bool,
    pre_shared_key: bool,
    psk_skip_session_ticket: bool,
    psk_dhe_ke: bool,
    certificate_compression: bool,
}

impl Preset {
    pub(super) fn build(self) -> TlsOptions {
        const LEGACY: Settings = Settings {
            curves: CLASSIC_FFDHE_GROUPS,
            ciphers: CIPHERS_WITH_ECDSA_CBC,
            key_shares: None,
            session_ticket: true,
            ech_grease: None,
            signed_cert_timestamps: false,
            pre_shared_key: false,
            psk_skip_session_ticket: false,
            psk_dhe_ke: true,
            certificate_compression: false,
        };

        const ECH: Settings = Settings {
            key_shares: Some(CLASSIC_KEY_SHARES),
            ech_grease: Some(EchGrease::NssPre3127),
            ..LEGACY
        };

        const RESUMPTION: Settings = Settings {
            pre_shared_key: true,
            psk_skip_session_ticket: true,
            certificate_compression: true,
            ..ECH
        };

        const MLKEM_RESUMPTION: Settings = Settings {
            curves: MLKEM_FFDHE_GROUPS,
            key_shares: Some(MLKEM_KEY_SHARES),
            ..RESUMPTION
        };

        const MLKEM_RESUMPTION_WITH_SCT: Settings = Settings {
            signed_cert_timestamps: true,
            ..MLKEM_RESUMPTION
        };

        const NO_ECDSA_CBC: Settings = Settings {
            ciphers: CIPHERS_WITHOUT_ECDSA_CBC,
            ..MLKEM_RESUMPTION_WITH_SCT
        };

        const NSS_3127_GREASE: Settings = Settings {
            ech_grease: Some(EchGrease::Nss3127),
            ..NO_ECDSA_CBC
        };

        let settings = match self {
            Self::Legacy => LEGACY,
            Self::EchWithoutResumption => Settings {
                ciphers: CIPHERS_WITHOUT_ECDSA_CBC,
                session_ticket: false,
                psk_dhe_ke: false,
                ..ECH
            },
            Self::MlkemResumption => MLKEM_RESUMPTION,
            Self::MlkemResumptionWithSct => MLKEM_RESUMPTION_WITH_SCT,
            Self::PrivateBrowsing => Settings {
                curves: MLKEM_FFDHE_GROUPS,
                key_shares: Some(MLKEM_KEY_SHARES),
                session_ticket: false,
                signed_cert_timestamps: true,
                psk_dhe_ke: false,
                certificate_compression: true,
                ..ECH
            },
            Self::AndroidResumption => RESUMPTION,
            Self::NoEcdsaAes128Cbc => Settings {
                ciphers: CIPHERS_WITHOUT_ECDSA_AES_128_CBC,
                ..MLKEM_RESUMPTION_WITH_SCT
            },
            Self::NoEcdsaCbc => NO_ECDSA_CBC,
            Self::Nss3127Grease => NSS_3127_GREASE,
            Self::NoFfdhe => Settings {
                curves: MLKEM_GROUPS,
                ..NSS_3127_GREASE
            },
        };

        let mut builder = TlsOptions::builder()
            .curves_list(settings.curves)
            .sigalgs_list(SIGALGS_LIST)
            .cipher_list(settings.ciphers)
            .tls13_cipher_list(TLS13_CIPHER_LIST)
            .session_ticket(settings.session_ticket)
            .delegated_credentials(DELEGATED_CREDENTIALS)
            .record_size_limit(RECORD_SIZE_LIMIT)
            .enable_ocsp_stapling(true)
            .ech_grease(settings.ech_grease)
            .enable_signed_cert_timestamps(settings.signed_cert_timestamps)
            .alpn_protocols([AlpnProtocol::HTTP2, AlpnProtocol::HTTP1])
            .min_tls_version(TlsVersion::TLS_1_2)
            .max_tls_version(TlsVersion::TLS_1_3)
            .pre_shared_key(settings.pre_shared_key)
            .psk_skip_session_ticket(settings.psk_skip_session_ticket)
            .psk_dhe_ke(settings.psk_dhe_ke)
            .extension_permutation(EXTENSION_PERMUTATION_INDICES);

        if let Some(key_shares) = settings.key_shares {
            builder = builder.key_shares(key_shares);
        }

        if settings.certificate_compression {
            builder = builder.certificate_compressors(CERTIFICATE_COMPRESSORS);
        }

        builder.build()
    }
}

const CLASSIC_FFDHE_GROUPS: &str = join!(
    ":",
    "X25519",
    "P-256",
    "P-384",
    "P-521",
    "ffdhe2048",
    "ffdhe3072"
);

const MLKEM_FFDHE_GROUPS: &str = join!(
    ":",
    "X25519MLKEM768",
    "X25519",
    "P-256",
    "P-384",
    "P-521",
    "ffdhe2048",
    "ffdhe3072"
);

// Firefox 156 stopped advertising FFDHE groups by default.
const MLKEM_GROUPS: &str = join!(":", "X25519MLKEM768", "X25519", "P-256", "P-384", "P-521");

const CLASSIC_KEY_SHARES: &[KeyShare] = &[KeyShare::X25519, KeyShare::P256];

const MLKEM_KEY_SHARES: &[KeyShare] =
    &[KeyShare::X25519_MLKEM768, KeyShare::X25519, KeyShare::P256];

const CIPHERS_WITH_ECDSA_CBC: &str = join!(
    ":",
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
    "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_RSA_WITH_AES_128_CBC_SHA",
    "TLS_RSA_WITH_AES_256_CBC_SHA"
);

const CIPHERS_WITHOUT_ECDSA_CBC: &str = join!(
    ":",
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_RSA_WITH_AES_128_CBC_SHA",
    "TLS_RSA_WITH_AES_256_CBC_SHA"
);

const CIPHERS_WITHOUT_ECDSA_AES_128_CBC: &str = join!(
    ":",
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_RSA_WITH_AES_128_CBC_SHA",
    "TLS_RSA_WITH_AES_256_CBC_SHA"
);

const SIGALGS_LIST: &str = join!(
    ":",
    "ecdsa_secp256r1_sha256",
    "ecdsa_secp384r1_sha384",
    "ecdsa_secp521r1_sha512",
    "rsa_pss_rsae_sha256",
    "rsa_pss_rsae_sha384",
    "rsa_pss_rsae_sha512",
    "rsa_pkcs1_sha256",
    "rsa_pkcs1_sha384",
    "rsa_pkcs1_sha512",
    "ecdsa_sha1",
    "rsa_pkcs1_sha1"
);

const TLS13_CIPHER_LIST: &[Tls13Cipher] = &[
    Tls13Cipher::AES_128_GCM_SHA256,
    Tls13Cipher::CHACHA20_POLY1305_SHA256,
    Tls13Cipher::AES_256_GCM_SHA384,
];

const CERTIFICATE_COMPRESSORS: &[&'static dyn CertificateCompressor] =
    &[&ZlibCompressor, &BrotliCompressor, &ZstdCompressor];

const DELEGATED_CREDENTIALS: &str = join!(
    ":",
    "ecdsa_secp256r1_sha256",
    "ecdsa_secp384r1_sha384",
    "ecdsa_secp521r1_sha512",
    "ecdsa_sha1"
);

const RECORD_SIZE_LIMIT: u16 = 0x4001;

const EXTENSION_PERMUTATION_INDICES: &[ExtensionType] = &[
    ExtensionType::SERVER_NAME,
    ExtensionType::EXTENDED_MASTER_SECRET,
    ExtensionType::RENEGOTIATE,
    ExtensionType::SUPPORTED_GROUPS,
    ExtensionType::EC_POINT_FORMATS,
    ExtensionType::SESSION_TICKET,
    ExtensionType::APPLICATION_LAYER_PROTOCOL_NEGOTIATION,
    ExtensionType::STATUS_REQUEST,
    ExtensionType::DELEGATED_CREDENTIAL,
    ExtensionType::CERTIFICATE_TIMESTAMP,
    ExtensionType::KEY_SHARE,
    ExtensionType::SUPPORTED_VERSIONS,
    ExtensionType::SIGNATURE_ALGORITHMS,
    ExtensionType::PSK_KEY_EXCHANGE_MODES,
    ExtensionType::RECORD_SIZE_LIMIT,
    ExtensionType::CERT_COMPRESSION,
    ExtensionType::ENCRYPTED_CLIENT_HELLO,
];
