macro_rules! define_ruby_enum {
    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_ruby_enum!($(#[$meta])* $enum_type, $ruby_class, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* const, $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_ruby_enum!($(#[$meta])* const, $enum_type, $ruby_class, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $(($rust_variant:ident, $ffi_variant:ident)),* $(,)?) => {
        $(#[$meta])*
        #[magnus::wrap(class = $ruby_class)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($rust_variant),*
        }

        impl $enum_type {
            pub fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$rust_variant => <$ffi_type>::$ffi_variant,)*
                }
            }

            pub fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$ffi_variant => <$enum_type>::$rust_variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };

    ($(#[$meta:meta])* const, $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $(($rust_variant:ident, $ffi_variant:ident)),* $(,)?) => {
        $(#[$meta])*
        #[magnus::wrap(class = $ruby_class)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($rust_variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$rust_variant => <$ffi_type>::$ffi_variant,)*
                }
            }

            #[allow(dead_code)]
            pub const fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$ffi_variant => <$enum_type>::$rust_variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}
