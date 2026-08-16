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
    (set_if_some_into_inner, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value.0.into_inner());
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
}

/// Convert a Ruby-native option whose field name is also its keyword name.
macro_rules! extract_native_option {
    ($options:expr, $target:expr, $field:ident) => {{
        $target.$field.set($options.convert(stringify!($field))?);
    }};
    ($options:expr, $target:expr, $field:ident, present) => {{
        $target
            .$field
            .set($options.convert_present(stringify!($field))?);
    }};
    ($options:expr, $target:expr, $field:ident, $source:ty => $map:expr) => {{
        $target
            .$field
            .set($options.convert::<$source>(stringify!($field))?.map($map));
    }};
}

macro_rules! define_ruby_enum {
    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, strings: $($variant:ident => $display:expr),* $(,)?) => {
        define_ruby_enum!(@impl $(#[$meta])* $enum_type, $ruby_class, $ffi_type, [$(($variant, $display)),*], []);
    };

    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, symbols: $($variant:ident => $symbol:expr),* $(,)?) => {
        define_ruby_enum!(@impl $(#[$meta])* $enum_type, $ruby_class, $ffi_type, [$(($variant, stringify!($variant))),*], [$($variant => $symbol),*]);
    };

    ($(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_ruby_enum!(@impl $(#[$meta])* $enum_type, $ruby_class, $ffi_type, [$(($variant, stringify!($variant))),*], []);
    };

    (@impl $(#[$meta:meta])* $enum_type:ident, $ruby_class:expr, $ffi_type:ty, [$(($variant:ident, $display:expr)),*], [$($symbol_variant:ident => $symbol:expr),*]) => {
        $(#[$meta])*
        #[magnus::wrap(class = $ruby_class, free_immediately, size)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($variant),*
        }

        impl $enum_type {
            /// Return the static text exposed through Ruby's `to_s`.
            #[inline]
            pub const fn to_s(&self) -> &'static str {
                match self {
                    $(<$enum_type>::$variant => $display,)*
                }
            }

            define_ruby_enum!(@to_sym $enum_type, [$($symbol_variant => $symbol),*]);

            /// Convert this Ruby wrapper into its native enum value.
            pub fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$variant => <$ffi_type>::$variant,)*
                }
            }

            /// Convert a known native enum value into its Ruby wrapper.
            #[allow(dead_code)]
            pub fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$variant => <$enum_type>::$variant,)*
                    _ => unreachable!(),
                }
            }

            /// Register every enum variant as a constant on the Ruby class.
            pub fn define_constants(class: magnus::RClass) -> Result<(), magnus::Error> {
                $(class.const_set(stringify!($variant), <$enum_type>::$variant)?;)*
                Ok(())
            }

            /// Compare two wrapped enum values for Ruby's `==`.
            pub fn equals(&self, other: magnus::Value) -> bool {
                use magnus::TryConvert;
                <&$enum_type>::try_convert(other)
                    .map(|other| *self == *other)
                    .unwrap_or(false)
            }

            /// Compare two wrapped enum values for Ruby's `eql?`.
            pub fn is_eql(&self, other: magnus::Value) -> bool {
                self.equals(other)
            }

            /// Return a hash consistent with Ruby's `eql?` contract.
            pub fn hash_value(&self) -> u64 {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }
        }
    };

    (@to_sym $enum_type:ident, []) => {};

    (@to_sym $enum_type:ident, [$($variant:ident => $symbol:expr),+]) => {
        /// Return the configured Ruby symbol for this enum value.
        #[inline]
        pub fn to_sym(ruby: &magnus::Ruby, rb_self: &Self) -> magnus::Symbol {
            let name = match *rb_self {
                $(<$enum_type>::$variant => $symbol,)+
            };
            ruby.to_symbol(name)
        }
    };
}

macro_rules! extract_request {
    ($ruby:expr, $args:expr, $required:ty) => {{
        let args = magnus::scan_args::scan_args::<$required, (), (), (), magnus::RHash, ()>($args)?;
        let required = args.required;
        let request = crate::client::req::Request::new($ruby, args.keywords)?;
        (required, request)
    }};
}
