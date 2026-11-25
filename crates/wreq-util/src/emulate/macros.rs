macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        dispatch,
        $name:ident, $default_variant:ident,
        $const_target:ident,
        $(
            $variant:ident => ($rename:expr, $emulation_fn:path)
        ),* $(,)?
    ) => {
        $(#[$meta])*
        #[non_exhaustive]
        #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "emulation-serde", derive(Deserialize, Serialize))]
        pub enum $name {
            $(
                #[cfg_attr(feature = "emulation-serde", serde(rename = $rename))]
                $variant,
            )*
        }

        impl $name {
            pub const VARIANTS: &[$name] = &[
                $(
                    $name::$variant,
                )*
            ];

            pub fn match_emulation(self, opt: $const_target) -> wreq::Emulation {
                match self {
                    $(
                        $name::$variant => $emulation_fn(opt),
                    )*
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default_variant
            }
        }

        #[allow(non_upper_case_globals)]
        impl $const_target {
            $(
                pub const $variant: $name = $name::$variant;
            )*
        }
    };

    (
        $(#[$meta:meta])*
        plain,
        $name:ident, $default_variant:ident,
        $(
            $variant:ident => $rename:expr
        ),* $(,)?
    ) => {
        $(#[$meta])*
        #[non_exhaustive]
        #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "emulation-serde", derive(Deserialize, Serialize))]
        pub enum $name {
            $(
                #[cfg_attr(feature = "emulation-serde", serde(rename = $rename))]
                $variant,
            )*
        }

        impl $name {
            pub const VARIANTS: &[$name] = &[
                $(
                    $name::$variant,
                )*
            ];
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default_variant
            }
        }
    };
}

macro_rules! join {
    ($sep:expr, $first:expr $(, $rest:expr)*) => {
        concat!($first $(, $sep, $rest)*)
    };
}

macro_rules! standard_mod_generator {
    ($mod_name:ident, $tls_options:expr, $http2_options:expr, $headers:expr) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline]
            pub fn emulation(emulation: Emulation) -> wreq::Emulation {
                build_emulation(emulation.http2, ($headers)(&emulation))
            }

            pub fn build_emulation(
                http2: bool,
                default_headers: Option<HeaderMap>,
            ) -> wreq::Emulation {
                build_standard_emulation(
                    stringify!($mod_name),
                    $tls_options,
                    http2.then(|| $http2_options),
                    default_headers,
                )
            }
        }
    };
    ($mod_name:ident, $build_emulation:expr, $headers:expr) => {
        pub(crate) mod $mod_name {
            use super::*;

            #[inline]
            pub fn emulation(emulation: Emulation) -> wreq::Emulation {
                $build_emulation(emulation.http2, ($headers)(&emulation))
            }
        }
    };
}

macro_rules! fixed_headers {
    ($emulation:expr, $header_initializer:ident, $ua:expr) => {
        $emulation.headers.then(|| $header_initializer($ua))
    };
}

macro_rules! platform_headers {
    (
        $emulation:expr,
        $header_preset:path,
        $brands:literal,
        [($default_os:ident, $default_ua:literal) $(, ($other_os:ident, $other_ua:literal))* $(,)?]
    ) => {{
        #[allow(unreachable_patterns)]
        $emulation.headers.then(|| {
            let (ua, platform) = match $emulation.platform {
                $( Platform::$other_os => ($other_ua, Platform::$other_os), )*
                _ => ($default_ua, Platform::$default_os),
            };
            ($header_preset).build($brands, ua, platform)
        })
    }};
}

macro_rules! mod_generator {
    (
        $mod_name:ident,
        $tls_options:expr,
        $http2_options:expr,
        $header_initializer:ident,
        $ua:expr
    ) => {
        standard_mod_generator!(
            $mod_name,
            $tls_options,
            $http2_options,
            |emulation: &Emulation| fixed_headers!(emulation, $header_initializer, $ua)
        );
    };
    (
        $mod_name:ident,
        $build_emulation:expr,
        $header_initializer:ident,
        $ua:expr
    ) => {
        standard_mod_generator!(
            $mod_name,
            $build_emulation,
            |emulation: &Emulation| fixed_headers!(emulation, $header_initializer, $ua)
        );
    };
}

macro_rules! chromium_profile {
    (
        $name:ident,
        tls: $tls:path,
        http2: $http2:path,
        headers: $headers:path,
        brands: $brands:literal,
        $platforms:tt $(,)?
    ) => {
        standard_mod_generator!(
            $name,
            ($tls).build(),
            ($http2).build(),
            |emulation: &Emulation| platform_headers!(emulation, $headers, $brands, $platforms)
        );
    };
    (
        $name:ident,
        transport: $transport:path,
        headers: $headers:path,
        brands: $brands:literal,
        $platforms:tt $(,)?
    ) => {
        standard_mod_generator!($name, $transport, |emulation: &Emulation| {
            platform_headers!(emulation, $headers, $brands, $platforms)
        });
    };
}
