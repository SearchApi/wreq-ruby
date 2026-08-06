//! Ruby time-value conversions shared by native binding modules.

use std::time::Duration as StdDuration;

use magnus::numeric::NumericValue;
use magnus::{Error, Integer, Ruby, TryConvert, Value, value::ReprValue};

use crate::error::argument_error;

/// A crate-wide duration converted from non-negative Ruby `Numeric` seconds.
///
/// Ruby integers are converted directly so the full `u64` seconds range is
/// retained. Other numeric values are converted through `f64` and then rounded
/// to the nanosecond precision of [`StdDuration`].
pub(crate) struct Duration(pub(crate) StdDuration);

impl TryConvert for Duration {
    /// Convert integer or fractional Ruby seconds without string coercion.
    ///
    /// # Errors
    ///
    /// Returns `TypeError` for non-numeric values and `ArgumentError` for
    /// negative, non-finite, or out-of-range durations.
    fn try_convert(value: Value) -> Result<Self, Error> {
        let ruby = Ruby::get_with(value);
        let numeric = NumericValue::try_convert(value)?;

        // `u64::try_convert` starts with `Integer::try_convert`. Probe with
        // `from_value` so a fractional Numeric takes the fallback without
        // using `TypeError` as control flow or repeating the Integer check.
        if let Some(integer) = Integer::from_value(numeric.as_value()) {
            return integer
                .to_u64()
                .map(StdDuration::from_secs)
                .map(Self)
                .map_err(|_| invalid_duration(&ruby));
        }

        // `Float::try_convert(...).to_f64()` takes a Ruby Float detour. The
        // `rb_num2dbl`-backed conversion yields the primitive required by
        // `StdDuration` directly from the already-checked Numeric.
        f64::try_convert(numeric.as_value())
            .and_then(|seconds| {
                StdDuration::try_from_secs_f64(seconds).map_err(|_| invalid_duration(&ruby))
            })
            .map(Self)
    }
}

/// Build the shared Ruby error for a numeric value outside `Duration`'s domain.
fn invalid_duration(ruby: &Ruby) -> Error {
    argument_error(
        ruby,
        "duration must be finite, non-negative, and within the supported range",
    )
}
