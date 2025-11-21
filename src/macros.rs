macro_rules! apply_option {
    (set_if_some, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    (set_if_some_ref, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&value);
        }
    };
    (set_if_some_inner, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value.0);
        }
    };
    (set_if_some_map, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (set_if_some_map_ref, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(&value));
        }
    };
    (set_if_true, $builder:expr, $option:expr, $method:ident, $default:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method();
        }
    };
    (set_if_true_with, $builder:expr, $option:expr, $method:ident, $default:expr, $value:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method($value);
        }
    };
}

macro_rules! define_ruby_enum {
    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_ruby_enum!($(#[$meta])* $enum_type, $ruby_class, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* const, $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_ruby_enum!($(#[$meta])* const, $enum_type, $ruby_class, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $(($rust_variant:ident, $ffi_variant:ident)),* $(,)?) => {
        $(#[$meta])*
        #[magnus::wrap(class = $ruby_class, free_immediately, size)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

            #[allow(dead_code)]
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
        #[magnus::wrap(class = $ruby_class, free_immediately, size)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($rust_variant),*
        }

        impl $enum_type {
            #[allow(dead_code)]
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

macro_rules! ruby {
    () => {
        Ruby::get().expect("Failed to get Ruby VM instance")
    };
}

macro_rules! extract_request {
    ($args:expr, $required:ty) => {{
        let args = magnus::scan_args::scan_args::<$required, (), (), (), magnus::RHash, ()>($args)?;
        let required = args.required;
        let request = crate::client::req::Request::new(&ruby!(), args.keywords)?;
        (required, request)
    }};
}
