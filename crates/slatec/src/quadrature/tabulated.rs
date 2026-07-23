//! Tabulated-data quadrature over checked sample storage.

use core::ops::RangeInclusive;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::interpolation::tabulated::TabulatedData;
use crate::runtime::lock_native;

use super::IntegrationError;

fn native_integer(value: usize) -> Result<FortranInteger, IntegrationError> {
    to_fortran_integer(value).map_err(|_| IntegrationError::IntegerOverflow)
}

macro_rules! impl_tabulated_integration {
    ($name:ident, $scalar:ty, $native:path) => {
        /// Integrates checked samples by SLATEC's overlapping-parabola rule.
        ///
        /// This is `AVINT`/`DAVINT`, intended for values tabulated at
        /// arbitrarily spaced abscissas.  Bounds must be finite, ordered, and
        /// inside the closed sampled domain. Equal bounds return zero without
        /// native entry. A two-sample data set uses the source's trapezoid
        /// special case; otherwise at least three sample abscissas must lie in
        /// the requested interval.
        ///
        /// The data is read-only, no caller workspace is exposed, and native
        /// execution is process-globally serialized because the error runtime
        /// remains reachable. This operation has no Rust callback and can be
        /// called while a non-callback native scope is already active.
        pub fn $name(
            data: &TabulatedData<$scalar>,
            interval: RangeInclusive<$scalar>,
        ) -> Result<$scalar, IntegrationError> {
            let mut lower = *interval.start();
            let mut upper = *interval.end();
            if !lower.is_finite()
                || !upper.is_finite()
                || lower > upper
                || lower < data.abscissas()[0]
                || upper > data.abscissas()[data.len() - 1]
            {
                return Err(IntegrationError::InvalidBounds);
            }
            if lower == upper {
                return Ok(0.0);
            }
            if data.len() > 2 {
                let count = data
                    .abscissas()
                    .iter()
                    .filter(|&&value| value >= lower && value <= upper)
                    .count();
                if count < 3 {
                    return Err(IntegrationError::InsufficientTabulatedPoints { found: count });
                }
            }
            let mut count = native_integer(data.len())?;
            let mut value = <$scalar>::default();
            let mut status = 0;
            let _native = lock_native();
            // SAFETY: `TabulatedData` owns equal-length finite strictly
            // increasing arrays. The Rust preflight establishes AVINT/DAVINT's
            // bounds and in-interval sample-count contract, while every scalar
            // output is stack-owned for the duration of this call.
            unsafe {
                $native(
                    data.abscissas().as_ptr().cast_mut(),
                    data.values().as_ptr().cast_mut(),
                    &mut count,
                    &mut lower,
                    &mut upper,
                    &mut value,
                    &mut status,
                );
            }
            if status != 1 {
                return Err(IntegrationError::NativeStatus(status));
            }
            Ok(value)
        }
    };
}

impl_tabulated_integration!(integrate_tabulated_f32, f32, slatec_sys::quadrature::avint);
impl_tabulated_integration!(integrate_tabulated, f64, slatec_sys::quadrature::davint);

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::interpolation::tabulated::TabulatedData;

    use super::{IntegrationError, integrate_tabulated};

    #[test]
    fn rejects_out_of_domain_and_underresolved_intervals_before_native_entry() {
        let data =
            TabulatedData::<f64>::from_samples(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]).unwrap();
        assert_eq!(
            integrate_tabulated(&data, -1.0..=1.0),
            Err(IntegrationError::InvalidBounds)
        );
        assert_eq!(
            integrate_tabulated(&data, 0.0..=0.5),
            Err(IntegrationError::InsufficientTabulatedPoints { found: 1 })
        );
    }
}
