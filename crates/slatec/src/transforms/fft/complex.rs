//! Safe one-dimensional complex FFTPACK plans.
//!
//! `ComplexFftPlan32` wraps the selected single-precision FFTPACK standard
//! real-array interface: `CFFTI1`, `CFFTF1`, and `CFFTB1`. It transforms an
//! in-place [`num_complex::Complex32`] slice, preserving native order and the
//! unnormalised equations
//!
//! ```text
//! forward(X)[j]  = sum_k X[k] exp(-i 2πjk / n)
//! backward(X)[j] = sum_k X[k] exp(+i 2πjk / n)
//! ```
//!
//! Thus `backward(forward(x)) = n * x`; neither operation divides by `n`.
//! The selected snapshot contains no native double-precision complex FFTPACK
//! entry points, so there is intentionally no `Complex64` plan.
//!
//! The standard `*1` routines declare their complex sequence as a real,
//! interleaved `C(*)` array. `num-complex` 0.4 guarantees that `Complex32` is
//! `repr(C)`, with real then imaginary `f32` fields, and memory-layout
//! compatible with `[f32; 2]`. The one audited pointer conversion is therefore
//! to `*mut f32`, not to a Fortran `COMPLEX` ABI type. All native calls retain
//! the process-wide runtime lock because `CFFTI1` has a DATA/SAVE factor table
//! and the hosted provider/runtime remains conservatively serialized.

#![cfg(feature = "fftpack-complex")]

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::mem::{align_of, size_of};
use core::slice;

use num_complex::Complex32;
use slatec_sys::FortranInteger;

pub use super::FftError;
use crate::runtime::lock_native;

const FACTOR_WORDS: usize = 15;

const _: () = assert!(size_of::<Complex32>() == 2 * size_of::<f32>());
const _: () = assert!(align_of::<Complex32>() == align_of::<f32>());

/// A reusable initialized single-precision complex FFTPACK plan.
///
/// The plan owns separate `2 * len()`-word scratch and initialized twiddle
/// tables plus a 15-word integer factor table. It borrows no caller data and
/// must be mutably borrowed for each transform because `CFFTF1`/`CFFTB1` use
/// its private scratch allocation. Valid lengths are at least two: `CFFTI1`
/// has no safe identity-length path, unlike the legacy wrapper that is not
/// bound by this API.
#[derive(Debug)]
pub struct ComplexFftPlan32 {
    length: usize,
    native_length: FortranInteger,
    scratch: Vec<f32>,
    twiddles: Vec<f32>,
    factors: Vec<FortranInteger>,
}

impl ComplexFftPlan32 {
    /// Creates and initializes a plan for `length >= 2` complex values.
    ///
    /// This allocates the exact `CFFTI1`/`CFFTF1` contract: two independent
    /// real arrays of `2 * length` words and 15 Fortran integer factor words.
    /// `CFFTI1` is invoked exactly once for this plan.
    pub fn new(length: usize) -> Result<Self, FftError> {
        if length < 2 {
            return Err(FftError::InvalidLength { length, minimum: 2 });
        }
        let native_length =
            FortranInteger::try_from(length).map_err(|_| FftError::DimensionOverflow)?;
        let scalar_words = complex_words(length)?;
        let scratch = zeroed::<f32>(scalar_words)?;
        let mut twiddles = zeroed::<f32>(scalar_words)?;
        let mut factors = zeroed::<FortranInteger>(FACTOR_WORDS)?;
        let mut native_length_for_call = native_length;
        let _native = lock_native();
        // SAFETY: every pointer targets an exact-length private allocation;
        // CFFTI1 only initializes WA[2*N] and IFAC[15] for this N.
        unsafe {
            slatec_sys::fftpack_complex::cffti1(
                &mut native_length_for_call,
                twiddles.as_mut_ptr(),
                factors.as_mut_ptr(),
            )
        };
        Ok(Self {
            length,
            native_length,
            scratch,
            twiddles,
            factors,
        })
    }

    /// Returns the immutable transform length in complex elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns whether this plan has zero transform length.
    ///
    /// Constructible plans have positive length, so this always returns
    /// `false`; it follows the collection-style convention used by real plans.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Applies `CFFTF1` in place using the negative-exponent convention.
    ///
    /// The caller's original allocation is passed directly as the native
    /// interleaved real buffer; this method allocates no output buffer and does
    /// not normalize. NaN and infinity are not rejected because FFTPACK has no
    /// finite-value status channel; they may propagate under native arithmetic.
    pub fn forward(&mut self, values: &mut [Complex32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack_complex::cfftf1)
    }

    /// Applies `CFFTB1` in place using the positive-exponent convention.
    ///
    /// It is the unnormalised backward partner of [`Self::forward`]. A
    /// backward call after a forward call multiplies every original value by
    /// [`Self::len`].
    pub fn backward(&mut self, values: &mut [Complex32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack_complex::cfftb1)
    }

    fn apply(
        &mut self,
        values: &mut [Complex32],
        transform: unsafe extern "C" fn(
            *mut FortranInteger,
            *mut f32,
            *mut f32,
            *mut f32,
            *mut FortranInteger,
        ),
    ) -> Result<(), FftError> {
        self.check_values(values)?;
        let words = interleaved_words(values)?;
        let mut native_length = self.native_length;
        let _native = lock_native();
        // SAFETY: `words`, scratch, twiddles, and factors meet the reviewed
        // CFFTF1/CFFTB1 exact capacities and stay uniquely borrowed throughout
        // this serialized call. The only layout conversion is documented in
        // `interleaved_words` and uses num-complex's repr(C) guarantee.
        unsafe {
            transform(
                &mut native_length,
                words.as_mut_ptr(),
                self.scratch.as_mut_ptr(),
                self.twiddles.as_mut_ptr(),
                self.factors.as_mut_ptr(),
            )
        };
        Ok(())
    }

    fn check_values(&self, values: &[Complex32]) -> Result<(), FftError> {
        if values.len() == self.length {
            Ok(())
        } else {
            Err(FftError::LengthMismatch {
                expected: self.length,
                actual: values.len(),
            })
        }
    }
}

fn complex_words(length: usize) -> Result<usize, FftError> {
    length.checked_mul(2).ok_or(FftError::DimensionOverflow)
}

fn zeroed<T: Copy + Default>(length: usize) -> Result<Vec<T>, FftError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| FftError::AllocationFailed)?;
    values.resize(length, T::default());
    Ok(values)
}

/// Borrows a `Complex32` allocation as its documented real/imaginary words.
fn interleaved_words(values: &mut [Complex32]) -> Result<&mut [f32], FftError> {
    let words = complex_words(values.len())?;
    if words > isize::MAX as usize / size_of::<f32>() {
        return Err(FftError::DimensionOverflow);
    }
    // SAFETY: num-complex 0.4 declares `Complex<T>` repr(C), re then im, and
    // memory-layout compatible with `[T; 2]`. The checks above ensure the
    // resulting slice remains within the original allocation's byte extent.
    Ok(unsafe { slice::from_raw_parts_mut(values.as_mut_ptr().cast::<f32>(), words) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_complex_layout_matches_the_reviewed_interleaved_contract() {
        assert_eq!(size_of::<Complex32>(), 2 * size_of::<f32>());
        assert_eq!(align_of::<Complex32>(), align_of::<f32>());
        let mut values = [Complex32::new(1.25, -2.5), Complex32::new(3.0, 4.5)];
        assert_eq!(
            interleaved_words(&mut values).unwrap(),
            &[1.25, -2.5, 3.0, 4.5]
        );
    }

    #[test]
    fn validates_lengths_without_native_entry() {
        assert!(matches!(
            ComplexFftPlan32::new(0),
            Err(FftError::InvalidLength {
                length: 0,
                minimum: 2
            })
        ));
        assert!(matches!(
            complex_words(usize::MAX),
            Err(FftError::DimensionOverflow)
        ));
    }
}
