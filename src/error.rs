use std::{
    borrow::Cow,
    cell::{BorrowError, BorrowMutError},
    fmt,
};

use magnus::{
    Attr, Class, Error as MagnusError, Exception, RModule, RObject, Ruby, TryConvert,
    error::ErrorType, exception::ExceptionClass, prelude::*, value::Lazy,
};
use tokio::sync::mpsc::error::SendError;

const ERROR_PREDICATES_IVAR: &str = "wreq_error_predicates";
type ErrorPredicateBits = u64;

const RACE_CONDITION_ERROR_MSG: &str = r#"Due to Rust's memory management with borrowing,
you cannot use certain instances multiple times as they may be consumed.

This error can occur in the following cases:
1) You passed a non-clonable instance to a function that requires ownership.
2) You attempted to use a method that consumes ownership more than once (e.g., reading a response body twice).
3) You tried to reference an instance after it was borrowed.

Potential solutions:
1) Avoid sharing instances; create a new instance each time you use it.
2) Refrain from performing actions that consume ownership multiple times.
3) Change the order of operations to reference the instance before borrowing it.
"#;

macro_rules! define_exception {
    ($name:ident, $ruby_name:literal, $parent_method:ident) => {
        static $name: Lazy<ExceptionClass> = Lazy::new(|ruby| {
            ruby.class_object()
                .const_get::<_, RModule>(crate::RUBY_MODULE_NAME)
                .and_then(|module| module.const_get::<_, ExceptionClass>($ruby_name))
                .unwrap_or_else(|_| ruby.$parent_method())
        });
    };
}

macro_rules! initialize_exception {
    ($ruby:expr, $module:expr, $name:ident, $ruby_name:literal, $parent:expr) => {{
        $module.define_error($ruby_name, $parent)?;
        Lazy::force(&$name, $ruby);
    }};
}

macro_rules! define_native_error_predicates {
    (
        $(
            $predicate:ident [$role:ident]:
                $native_method:ident as $ruby_method:ident
        ),+ $(,)?
    ) => {
        /// How a native predicate participates in the Ruby error contract.
        #[cfg(test)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum ErrorPredicateRole {
            NativeKind,
            TransportDetail,
            Diagnostic,
        }

        /// Predicates captured from a native `wreq::Error`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        enum ErrorPredicate {
            $($predicate),+
        }

        impl ErrorPredicate {
            /// Every predicate retained in Ruby exception metadata.
            const ALL: &'static [Self] = &[$(Self::$predicate),+];

            /// Return this predicate's position in the compact Ruby metadata.
            const fn mask(self) -> ErrorPredicateBits {
                1 << (self as u8)
            }

            /// Return how this native fact participates in the Ruby contract.
            #[cfg(test)]
            const fn role(self) -> ErrorPredicateRole {
                match self {
                    $(Self::$predicate => ErrorPredicateRole::$role,)+
                }
            }

            /// Evaluate this predicate before the native error is consumed.
            fn matches_wreq(self, error: &wreq::Error) -> bool {
                match self {
                    $(Self::$predicate => error.$native_method(),)+
                }
            }
        }

        const _: () =
            assert!(ErrorPredicate::ALL.len() <= ErrorPredicateBits::BITS as usize);

        $(
            fn $native_method(rb_self: RObject) -> Result<bool, MagnusError> {
                error_has_predicate(rb_self, ErrorPredicate::$predicate)
            }
        )+

        /// Define idiomatic Ruby predicate methods on `Wreq::Error`.
        fn include_error_predicates(class: ExceptionClass) -> Result<(), MagnusError> {
            $(
                class.define_method(
                    concat!(stringify!($ruby_method), "?"),
                    magnus::method!($native_method, 0),
                )?;
            )+
            Ok(())
        }
    };
}

macro_rules! define_ruby_error_categories {
    (
        $(
            $category:ident => $class:ident $(($ruby_name:literal))?
        ),+ $(,)?
    ) => {
        /// Stable exception categories owned by the Ruby API.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum RubyErrorCategory {
            $($category),+
        }

        impl RubyErrorCategory {
            /// Return the Ruby exception class for this stable category.
            fn error_class(self) -> &'static Lazy<ExceptionClass> {
                match self {
                    $(Self::$category => &$class,)+
                }
            }
        }

        $(
            $(define_exception!($class, $ruby_name, exception_runtime_error);)?
        )+

        /// Define and retain every mapped Ruby exception class.
        fn initialize_mapped_errors(
            ruby: &Ruby,
            gem_module: &RModule,
            parent: ExceptionClass,
        ) -> Result<(), MagnusError> {
            $(
                $(
                    initialize_exception!(ruby, gem_module, $class, $ruby_name, parent);
                )?
            )+
            Ok(())
        }
    };
}

// wreq keeps its error kind private. Keep its mutually exclusive kind predicates
// separate from transport details, which inspect the source chain and may overlap.
// Each entry maps the native method before `as` to the Ruby predicate after it.
// Classification order is declared separately as part of the Ruby contract.
define_native_error_predicates! {
    Builder [NativeKind]: is_builder as builder,
    Body [NativeKind]: is_body as body,
    Tls [NativeKind]: is_tls as tls,
    Decode [NativeKind]: is_decode as decoding,
    Redirect [NativeKind]: is_redirect as redirect,
    Status [NativeKind]: is_status as status,
    Upgrade [NativeKind]: is_upgrade as upgrade,
    Request [NativeKind]: is_request as request,
    ConnectionReset [TransportDetail]:
        is_connection_reset as connection_reset,
    Timeout [TransportDetail]: is_timeout as timeout,
    ProxyConnect [TransportDetail]:
        is_proxy_connect as proxy_connect,
    Connect [TransportDetail]: is_connect as connect,
}

define_ruby_error_categories! {
    Base => WREQ_ERROR,
    Builder => BUILDER_ERROR("BuilderError"),
    Body => BODY_ERROR("BodyError"),
    Tls => TLS_ERROR("TlsError"),
    Decoding => DECODING_ERROR("DecodingError"),
    Redirect => REDIRECT_ERROR("RedirectError"),
    Status => STATUS_ERROR("StatusError"),
    Request => REQUEST_ERROR("RequestError"),
    ConnectionReset => CONNECTION_RESET_ERROR("ConnectionResetError"),
    Timeout => TIMEOUT_ERROR("TimeoutError"),
    ProxyConnect => PROXY_CONNECT_ERROR("ProxyConnectError"),
    Connect => CONNECT_ERROR("ConnectError"),
}

/// Stable mapping from native facts to Ruby exception categories.
///
/// Non-request kinds keep their existing precedence. Source-chain details are
/// then classified independently of `is_request()`, with the generic request
/// category retained only as a fallback. This order belongs to the Ruby API.
const RUBY_ERROR_CLASSIFICATION: &[(ErrorPredicate, RubyErrorCategory)] = &[
    (ErrorPredicate::Builder, RubyErrorCategory::Builder),
    (ErrorPredicate::Body, RubyErrorCategory::Body),
    (ErrorPredicate::Tls, RubyErrorCategory::Tls),
    (ErrorPredicate::Decode, RubyErrorCategory::Decoding),
    (ErrorPredicate::Redirect, RubyErrorCategory::Redirect),
    (ErrorPredicate::Status, RubyErrorCategory::Status),
    (ErrorPredicate::Upgrade, RubyErrorCategory::Base),
    (
        ErrorPredicate::ConnectionReset,
        RubyErrorCategory::ConnectionReset,
    ),
    (ErrorPredicate::Timeout, RubyErrorCategory::Timeout),
    (
        ErrorPredicate::ProxyConnect,
        RubyErrorCategory::ProxyConnect,
    ),
    (ErrorPredicate::Connect, RubyErrorCategory::Connect),
    (ErrorPredicate::Request, RubyErrorCategory::Request),
];

/// Native facts exposed to Ruby without changing the exception class.
///
/// Add new overlapping predicates here unless a major release intentionally
/// changes which exception existing rescue clauses receive.
#[cfg(test)]
const RUBY_DIAGNOSTIC_PREDICATES: &[ErrorPredicate] = &[];

/// Native error facts retained after consuming a wreq error.
#[derive(Clone, Copy, Default)]
struct NativeErrorFacts(ErrorPredicateBits);

impl NativeErrorFacts {
    /// Restore predicates from compact Ruby metadata.
    const fn from_bits(bits: ErrorPredicateBits) -> Self {
        Self(bits)
    }

    /// Return the compact representation stored on the Ruby exception.
    const fn bits(self) -> ErrorPredicateBits {
        self.0
    }

    /// Return whether the set contains a native predicate.
    const fn contains(self, predicate: ErrorPredicate) -> bool {
        self.0 & predicate.mask() != 0
    }

    /// Include a predicate when its native check succeeds.
    #[must_use]
    const fn include_if(mut self, predicate: ErrorPredicate, include: bool) -> Self {
        if include {
            self.0 |= predicate.mask();
        }
        self
    }

    /// Classify captured facts using the binding-owned Ruby contract.
    fn ruby_category(self) -> RubyErrorCategory {
        RUBY_ERROR_CLASSIFICATION
            .iter()
            .find_map(|&(predicate, category)| self.contains(predicate).then_some(category))
            .unwrap_or(RubyErrorCategory::Base)
    }
}

impl From<&wreq::Error> for NativeErrorFacts {
    /// Snapshot every native predicate before consuming the wreq error.
    fn from(error: &wreq::Error) -> Self {
        ErrorPredicate::ALL
            .iter()
            .copied()
            .fold(Self::default(), |predicates, predicate| {
                predicates.include_if(predicate, predicate.matches_wreq(error))
            })
    }
}

/// Native error details retained after converting a wreq error to Ruby.
struct ErrorMetadata<'a> {
    uri: Option<&'a str>,
    status: Option<wreq::StatusCode>,
    facts: NativeErrorFacts,
}

// Stable roots for native errors.
define_exception!(WREQ_ERROR, "Error", exception_runtime_error);

// Keep interruption outside StandardError so a broad transport rescue
// never swallows a Ruby interrupt.
define_exception!(INTERRUPT_ERROR, "InterruptError", exception_interrupt);

// System-level and runtime errors
define_exception!(MEMORY, "MemoryError", exception_runtime_error);
define_exception!(FORK_ERROR, "ForkError", exception_runtime_error);

/// Memory error constant
pub fn memory_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.get_inner(&MEMORY), RACE_CONDITION_ERROR_MSG)
}

/// Create a `Wreq::InterruptError` when Ruby interrupts a request.
pub fn interrupt_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.get_inner(&INTERRUPT_ERROR), "request interrupted")
}

/// Build `Wreq::ForkError` without touching inherited native state.
#[cfg(unix)]
pub fn fork_error(ruby: &Ruby, owner_pid: u32, current_pid: u32) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&FORK_ERROR),
        format!(
            "wreq-ruby was loaded in process {owner_pid} and cannot be used after fork in process {current_pid}"
        ),
    )
}

/// Map a failed process-fork handler registration to `Wreq::ForkError`.
#[cfg(unix)]
pub fn fork_handler_error(ruby: &Ruby, err: &std::io::Error) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&FORK_ERROR),
        format!("failed to initialize process fork tracking: {err}"),
    )
}

/// Map a Tokio runtime initialization failure to `Wreq::BuilderError`.
pub fn runtime_initialization_error(ruby: &Ruby, err: &std::io::Error) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("failed to initialize Tokio runtime: {err}"),
    )
}

/// LocalJumpError for methods that require a Ruby block.
pub fn no_block_given_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.exception_local_jump_error(), "no block given (yield)")
}

/// Build an `IOError` for writes to a closed request-body sender.
pub fn closed_body_sender_error(ruby: &Ruby) -> MagnusError {
    MagnusError::new(ruby.exception_io_error(), "closed body sender")
}

/// Map a failed body-channel send to `IOError`.
pub fn body_sender_send_error<T>(ruby: &Ruby, err: SendError<T>) -> MagnusError {
    MagnusError::new(
        ruby.exception_io_error(),
        format!("closed body sender: {err}"),
    )
}

/// Map an immutable sender-state borrow failure to `Wreq::BodyError`.
pub fn body_sender_borrow_error(ruby: &Ruby, err: BorrowError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BODY_ERROR),
        format!("body sender state is unavailable: {err}"),
    )
}

/// Map a mutable sender-state borrow failure to `Wreq::BodyError`.
pub fn body_sender_borrow_mut_error(ruby: &Ruby, err: BorrowMutError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BODY_ERROR),
        format!("body sender state is unavailable: {err}"),
    )
}

/// Map [`wreq::header::InvalidHeaderName`] to corresponding [`magnus::Error`]
pub fn header_name_error(ruby: &Ruby, err: wreq::header::InvalidHeaderName) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("invalid header name: {err}"),
    )
}

/// Map [`wreq::header::InvalidHeaderValue`] to corresponding [`magnus::Error`]
pub fn header_value_error(ruby: &Ruby, err: wreq::header::InvalidHeaderValue) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("invalid header value: {err}"),
    )
}

/// Build a `Wreq::BuilderError` for an invalid Ruby header structure.
pub fn header_type_error(ruby: &Ruby, err: &str) -> MagnusError {
    MagnusError::new(ruby.get_inner(&BUILDER_ERROR), format!("type error: {err}"))
}

/// Build a `Wreq::BuilderError` for unsupported request JSON values.
pub fn json_serialization_error(ruby: &Ruby, err: MagnusError) -> MagnusError {
    MagnusError::new(
        ruby.get_inner(&BUILDER_ERROR),
        format!("JSON serialization error: {err}"),
    )
}

/// Prefix a Magnus error while preserving its Ruby exception class and cause.
pub fn contextualize_magnus_error(err: MagnusError, context: fmt::Arguments<'_>) -> MagnusError {
    match err.error_type() {
        ErrorType::Error(class, message) => {
            MagnusError::new(*class, format!("{context}: {message}"))
        }
        ErrorType::Exception(exception) => {
            let contextualized: Result<Exception, MagnusError> =
                exception.funcall("exception", (format!("{context}: {exception}"),));
            contextualized.map_or_else(|error| error, MagnusError::from)
        }
        ErrorType::Jump(_) => err,
    }
}

/// Add an option name while preserving the original Ruby exception class.
pub fn option_value_error(option: &str, err: MagnusError) -> MagnusError {
    contextualize_magnus_error(err, format_args!("invalid value for :{option}"))
}

/// Build an `ArgumentError` from a validation message.
pub fn argument_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_arg_error(), message)
}

/// Builds a Ruby `RangeError` from a validation message.
pub fn range_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_range_error(), message)
}

/// Build a `TypeError` from a conversion message.
pub fn type_error(ruby: &Ruby, message: impl Into<Cow<'static, str>>) -> MagnusError {
    MagnusError::new(ruby.exception_type_error(), message)
}

/// Select the Ruby exception class from the binding-owned category.
fn wreq_error_class(ruby: &Ruby, facts: NativeErrorFacts) -> ExceptionClass {
    ruby.get_inner(facts.ruby_category().error_class())
}

/// Read one native predicate from a Ruby error, defaulting to false.
fn error_has_predicate(rb_self: RObject, predicate: ErrorPredicate) -> Result<bool, MagnusError> {
    rb_self
        .ivar_get::<_, Option<ErrorPredicateBits>>(ERROR_PREDICATES_IVAR)
        .map(|bits| bits.is_some_and(|bits| NativeErrorFacts::from_bits(bits).contains(predicate)))
}

/// Construct a Ruby exception and attach captured native error metadata.
fn error_with_metadata(
    ruby: &Ruby,
    class: ExceptionClass,
    message: String,
    metadata: ErrorMetadata<'_>,
) -> MagnusError {
    match class.new_instance((message,)).and_then(|exception| {
        let object = RObject::try_convert(exception.as_value())?;
        object.ivar_set(ERROR_PREDICATES_IVAR, metadata.facts.bits())?;

        if let Some(uri) = metadata.uri {
            let uri = ruby.str_new(uri);
            uri.freeze();
            object.ivar_set("@uri", uri)?;
        }

        if let Some(status) = metadata.status {
            object.ivar_set("@status", status.as_u16())?;
        }

        Ok(exception)
    }) {
        Ok(exception) => exception.into(),
        Err(error) => error,
    }
}

/// Map [`wreq::Error`] to corresponding [`magnus::Error`].
pub fn wreq_error(ruby: &Ruby, err: wreq::Error) -> MagnusError {
    let facts = NativeErrorFacts::from(&err);
    let class = wreq_error_class(ruby, facts);
    let uri = err.uri().map(ToString::to_string);
    let status = err.status();
    let message = err.without_uri().to_string();

    error_with_metadata(
        ruby,
        class,
        message,
        ErrorMetadata {
            uri: uri.as_deref(),
            status,
            facts,
        },
    )
}

/// Define and retain the Ruby exception classes used by the binding.
///
/// # Errors
///
/// Returns the Ruby exception raised while defining an error class.
pub fn include(ruby: &Ruby, gem_module: &RModule) -> Result<(), MagnusError> {
    let error_class = gem_module.define_error("Error", ruby.exception_runtime_error())?;
    error_class.define_attr("uri", Attr::Read)?;
    error_class.define_attr("status", Attr::Read)?;
    include_error_predicates(error_class)?;
    Lazy::force(&WREQ_ERROR, ruby);

    gem_module.define_error("InterruptError", ruby.exception_interrupt())?;
    Lazy::force(&INTERRUPT_ERROR, ruby);

    initialize_exception!(ruby, gem_module, MEMORY, "MemoryError", error_class);
    initialize_exception!(ruby, gem_module, FORK_ERROR, "ForkError", error_class);
    initialize_mapped_errors(ruby, gem_module, error_class)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        ErrorPredicate, ErrorPredicateRole, NativeErrorFacts, RUBY_DIAGNOSTIC_PREDICATES,
        RUBY_ERROR_CLASSIFICATION, RubyErrorCategory,
    };

    fn facts(entries: &[ErrorPredicate]) -> NativeErrorFacts {
        entries
            .iter()
            .copied()
            .fold(NativeErrorFacts::default(), |facts, predicate| {
                facts.include_if(predicate, true)
            })
    }

    #[test]
    fn error_predicate_bits_are_unique_and_round_trip() {
        let facts = ErrorPredicate::ALL.iter().copied().fold(
            NativeErrorFacts::default(),
            |facts, predicate| {
                assert!(!facts.contains(predicate));
                facts.include_if(predicate, true)
            },
        );

        assert_eq!(
            ErrorPredicate::ALL.len(),
            facts.bits().count_ones() as usize
        );

        let restored = NativeErrorFacts::from_bits(facts.bits());
        for &predicate in ErrorPredicate::ALL {
            assert!(restored.contains(predicate));
        }
    }

    #[test]
    fn ruby_error_contract_covers_every_predicate_once() {
        let mut seen = NativeErrorFacts::default();

        for &(predicate, _) in RUBY_ERROR_CLASSIFICATION {
            assert_ne!(ErrorPredicateRole::Diagnostic, predicate.role());
            assert!(
                !seen.contains(predicate),
                "duplicate predicate: {predicate:?}"
            );
            seen = seen.include_if(predicate, true);
        }

        for &predicate in RUBY_DIAGNOSTIC_PREDICATES {
            assert_eq!(ErrorPredicateRole::Diagnostic, predicate.role());
            assert!(
                !seen.contains(predicate),
                "duplicate predicate: {predicate:?}"
            );
            seen = seen.include_if(predicate, true);
        }

        assert_eq!(ErrorPredicate::ALL.len(), seen.bits().count_ones() as usize);

        for &predicate in RUBY_DIAGNOSTIC_PREDICATES {
            assert_eq!(RubyErrorCategory::Base, facts(&[predicate]).ruby_category());
        }
    }

    #[test]
    fn ruby_error_classification_is_owned_by_the_binding() {
        for &(predicate, category) in RUBY_ERROR_CLASSIFICATION {
            assert_eq!(category, facts(&[predicate]).ruby_category());
        }

        assert_eq!(RubyErrorCategory::Base, facts(&[]).ruby_category());

        for &(kind, kind_category) in RUBY_ERROR_CLASSIFICATION {
            if kind.role() != ErrorPredicateRole::NativeKind || kind == ErrorPredicate::Request {
                continue;
            }

            for &(detail, _) in RUBY_ERROR_CLASSIFICATION {
                if detail.role() == ErrorPredicateRole::TransportDetail {
                    assert_eq!(
                        kind_category,
                        facts(&[kind, detail]).ruby_category(),
                        "native kind {kind:?} must take precedence over {detail:?}"
                    );
                }
            }
        }

        for (index, &(detail, detail_category)) in RUBY_ERROR_CLASSIFICATION.iter().enumerate() {
            if detail.role() != ErrorPredicateRole::TransportDetail {
                continue;
            }

            assert_eq!(
                detail_category,
                facts(&[ErrorPredicate::Request, detail]).ruby_category(),
                "transport detail {detail:?} must not depend on the request kind"
            );

            for &(lower_priority, _) in &RUBY_ERROR_CLASSIFICATION[index + 1..] {
                if lower_priority.role() == ErrorPredicateRole::TransportDetail {
                    assert_eq!(
                        detail_category,
                        facts(&[detail, lower_priority]).ruby_category(),
                        "transport detail {detail:?} must take precedence over {lower_priority:?}"
                    );
                }
            }
        }
    }
}
