//! Safe plans for the single-precision real FFTPACK routines in SLATEC.
//!
//! This module retains FFTPACK's native, unnormalised conventions.  Its plan
//! objects own the initialized Fortran work array and transform the caller's
//! contiguous `f32` slice in place where the original routine does.  The
//! selected source snapshot contains no double-precision counterparts for
//! these real families, so this module intentionally exposes `f32` only.
//!
//! Calls remain process-wide serialized.  The FFTPACK initializer closure
//! contains two DATA/SAVE factor tables; they are read-only after loading, but
//! this first facade deliberately uses the project's existing conservative
//! native-runtime policy rather than advertising parallel native execution.
//!
//! Inputs are not restricted to finite values. FFTPACK has no finite-value
//! status channel, so IEEE NaNs and infinities are passed through and may
//! propagate according to the linked compiler's arithmetic.

use alloc::vec::Vec;
use core::convert::TryFrom;

use slatec_sys::FortranInteger;

use crate::runtime::lock_native;

/// Shared pre-native FFTPACK validation errors.
pub use crate::transforms::fft::FftError;

#[derive(Debug)]
struct PlanCore {
    length: usize,
    native_length: FortranInteger,
    workspace: Vec<f32>,
}

impl PlanCore {
    fn new(
        length: usize,
        minimum: usize,
        workspace_length: usize,
        initializer: unsafe extern "C" fn(*mut FortranInteger, *mut f32),
    ) -> Result<Self, FftError> {
        if length < minimum {
            return Err(FftError::InvalidLength { length, minimum });
        }
        let native_length =
            FortranInteger::try_from(length).map_err(|_| FftError::DimensionOverflow)?;
        let mut workspace = allocate_zeroed(workspace_length)?;
        let mut native_length_for_call = native_length;
        let _native = lock_native();
        // SAFETY: `native_length_for_call` and `workspace` are live, mutable,
        // correctly sized native allocations.  The exact workspace formula and
        // length precondition were verified from the corresponding prologue.
        unsafe { initializer(&mut native_length_for_call, workspace.as_mut_ptr()) };
        Ok(Self {
            length,
            native_length,
            workspace,
        })
    }

    fn check_values(&self, values: &[f32]) -> Result<(), FftError> {
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

fn allocate_zeroed(length: usize) -> Result<Vec<f32>, FftError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| FftError::AllocationFailed)?;
    values.resize(length, 0.0);
    Ok(values)
}

fn rfft_workspace(length: usize) -> Result<usize, FftError> {
    length
        .checked_mul(2)
        .and_then(|value| value.checked_add(15))
        .ok_or(FftError::DimensionOverflow)
}

fn three_n_workspace(length: usize) -> Result<usize, FftError> {
    length
        .checked_mul(3)
        .and_then(|value| value.checked_add(15))
        .ok_or(FftError::DimensionOverflow)
}

fn sint_workspace(length: usize) -> Result<usize, FftError> {
    length
        .checked_mul(7)
        .and_then(|value| value.checked_div(2))
        .and_then(|value| value.checked_add(16))
        .ok_or(FftError::DimensionOverflow)
}

/// A reusable initialized plan for `RFFTI`, `RFFTF`, and `RFFTB`.
///
/// `forward` stores FFTPACK's native packed real spectrum in the supplied
/// vector.  `backward` consumes that same packed representation.  A backward
/// call after a forward call scales the original sequence by `length`; neither
/// method normalizes automatically.  The plan owns a `2 * length + 15` word
/// workspace and uses the original input allocation directly.
#[derive(Debug)]
pub struct RealFftPlan {
    core: PlanCore,
}

impl RealFftPlan {
    /// Creates a plan for a real periodic sequence of `length >= 1`.
    ///
    /// This invokes SLATEC `RFFTI` exactly once and allocates its documented
    /// `2 * length + 15` word workspace.
    pub fn new(length: usize) -> Result<Self, FftError> {
        Ok(Self {
            core: PlanCore::new(
                length,
                1,
                rfft_workspace(length)?,
                slatec_sys::fftpack::rffti,
            )?,
        })
    }

    /// Returns the fixed transform length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.core.length
    }

    /// Returns whether the plan has zero transform length.
    ///
    /// All constructible plans have a positive length, so this always returns
    /// `false`; it exists for collection-style API consistency.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Applies SLATEC `RFFTF` in place.
    ///
    /// The output uses FFTPACK's packed real spectrum.  Call
    /// [`Self::spectrum`] to inspect it without allocating.  A later
    /// [`Self::backward`] multiplies this result by `length` rather than
    /// restoring it exactly.
    pub fn forward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.core.check_values(values)?;
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: checked exact-length caller allocation and initialized plan
        // workspace remain uniquely borrowed for the whole native call.
        unsafe {
            slatec_sys::fftpack::rfftf(
                &mut n,
                values.as_mut_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(())
    }

    /// Applies SLATEC `RFFTB` in place to FFTPACK's packed real spectrum.
    ///
    /// It is the unnormalised backward partner of [`Self::forward`].
    pub fn backward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.core.check_values(values)?;
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: checked exact-length caller allocation and initialized plan
        // workspace remain uniquely borrowed for the whole native call.
        unsafe {
            slatec_sys::fftpack::rfftb(
                &mut n,
                values.as_mut_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(())
    }

    /// Borrows a checked view of an `RFFTF` output buffer.
    pub fn spectrum<'a>(&self, values: &'a [f32]) -> Result<RealSpectrumRef<'a>, FftError> {
        self.core.check_values(values)?;
        Ok(RealSpectrumRef { values })
    }
}

/// A zero-allocation view of the packed spectrum produced by `RFFTF`.
///
/// `dc()` is `R(1)`.  For harmonic `k` in `1..=(n - 1) / 2`, `harmonic(k)`
/// returns `(cosine, negative_sine)`, the exact native values
/// `(R(2k), R(2k+1))`.  Thus the second value is the negative of the usual
/// positive-sine sum.  For even `n`, `nyquist()` returns native `R(n)`.
#[derive(Clone, Copy, Debug)]
pub struct RealSpectrumRef<'a> {
    values: &'a [f32],
}

impl<'a> RealSpectrumRef<'a> {
    /// Returns the unnormalised constant coefficient, equal to the input sum.
    #[must_use]
    pub fn dc(self) -> f32 {
        self.values[0]
    }

    /// Returns `(cosine, negative_sine)` for a positive harmonic, if present.
    #[must_use]
    pub fn harmonic(self, harmonic: usize) -> Option<(f32, f32)> {
        if harmonic == 0 || harmonic > (self.values.len() - 1) / 2 {
            return None;
        }
        Some((self.values[2 * harmonic - 1], self.values[2 * harmonic]))
    }

    /// Returns the unnormalised Nyquist coefficient for an even-length spectrum.
    #[must_use]
    pub fn nyquist(self) -> Option<f32> {
        (self.values.len() % 2 == 0).then(|| self.values[self.values.len() - 1])
    }
}

/// Separate Fourier-series coefficients produced by `EZFFTF`.
///
/// `mean` is the arithmetic mean.  `cosine[k]` and `sine[k]` represent
/// harmonic `k + 1`; for even lengths the final sine value is zero and the
/// final cosine value is the Nyquist coefficient.  Coefficient vectors have
/// exactly `length / 2` entries.
#[derive(Clone, Debug, PartialEq)]
pub struct EasyRealSpectrum {
    /// The constant Fourier coefficient, equal to the input mean.
    pub mean: f32,
    /// Cosine-series coefficients indexed by harmonic minus one.
    pub cosine: Vec<f32>,
    /// Sine-series coefficients indexed by harmonic minus one.
    pub sine: Vec<f32>,
}

/// A reusable initialized plan for `EZFFTI`, `EZFFTF`, and `EZFFTB`.
///
/// Unlike `RFFTF`, `EZFFTF` leaves its input slice unchanged and returns
/// separate Fourier-series coefficients.  `backward` synthesizes directly
/// into a caller-provided mutable output slice.
#[derive(Debug)]
pub struct EasyRealFftPlan {
    core: PlanCore,
}

impl EasyRealFftPlan {
    /// Creates a plan for a real periodic sequence of `length >= 1`.
    ///
    /// This invokes SLATEC `EZFFTI` and owns its documented
    /// `3 * length + 15` word workspace.
    pub fn new(length: usize) -> Result<Self, FftError> {
        Ok(Self {
            core: PlanCore::new(
                length,
                1,
                three_n_workspace(length)?,
                slatec_sys::fftpack::ezffti,
            )?,
        })
    }

    /// Returns the fixed transform length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.core.length
    }

    /// Returns whether the plan has zero transform length.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Computes separate `EZFFTF` coefficients without modifying `values`.
    ///
    /// The result owns its two explicitly allocated coefficient arrays.  The
    /// native routine uses the plan workspace as scratch storage.
    pub fn forward(&mut self, values: &[f32]) -> Result<EasyRealSpectrum, FftError> {
        self.core.check_values(values)?;
        let coefficient_length = self.core.length / 2;
        let mut cosine = allocate_zeroed(coefficient_length)?;
        let mut sine = allocate_zeroed(coefficient_length)?;
        let mut mean = 0.0;
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: source audit establishes that EZFFTF reads `values` but does
        // not modify it; all output and workspace allocations are valid for
        // their documented lengths and are uniquely borrowed.
        unsafe {
            slatec_sys::fftpack::ezfftf(
                &mut n,
                values.as_ptr(),
                &mut mean,
                cosine.as_mut_ptr(),
                sine.as_mut_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(EasyRealSpectrum { mean, cosine, sine })
    }

    /// Synthesizes `values` from `EZFFTF`-compatible coefficients.
    ///
    /// `spectrum` must have exactly `length / 2` cosine and sine entries;
    /// neither coefficient vector is modified.  The resulting sequence uses
    /// FFTPACK's native normalization and is written directly to `values`.
    pub fn backward(
        &mut self,
        spectrum: &EasyRealSpectrum,
        values: &mut [f32],
    ) -> Result<(), FftError> {
        self.core.check_values(values)?;
        let expected = self.core.length / 2;
        if spectrum.cosine.len() != expected {
            return Err(FftError::LengthMismatch {
                expected,
                actual: spectrum.cosine.len(),
            });
        }
        if spectrum.sine.len() != expected {
            return Err(FftError::LengthMismatch {
                expected,
                actual: spectrum.sine.len(),
            });
        }
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: source audit establishes EZFFTB reads these coefficient
        // inputs without changing them; `values` and workspace are uniquely
        // borrowed and meet the documented exact lengths.
        unsafe {
            slatec_sys::fftpack::ezfftb(
                &mut n,
                values.as_mut_ptr(),
                &spectrum.mean,
                spectrum.cosine.as_ptr(),
                spectrum.sine.as_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(())
    }
}

macro_rules! define_self_inverse_plan {
    ($type_name:ident, $initializer:ident, $transform:ident, $minimum:expr, $workspace:ident, $description:literal, $scale:literal) => {
        #[doc = $description]
        #[doc = "\n\nThis plan owns its initialized native workspace.  `transform` is"]
        #[doc = "self-inverse up to the native scale "]
        #[doc = $scale]
        #[derive(Debug)]
        pub struct $type_name {
            core: PlanCore,
        }

        impl $type_name {
            /// Creates a plan for the documented transform length.
            pub fn new(length: usize) -> Result<Self, FftError> {
                Ok(Self {
                    core: PlanCore::new(
                        length,
                        $minimum,
                        $workspace(length)?,
                        slatec_sys::fftpack::$initializer,
                    )?,
                })
            }

            /// Returns the fixed transform length.
            #[must_use]
            pub fn len(&self) -> usize {
                self.core.length
            }

            /// Returns whether the plan has zero transform length.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                false
            }

            /// Applies the native in-place transform once.
            pub fn transform(&mut self, values: &mut [f32]) -> Result<(), FftError> {
                self.core.check_values(values)?;
                let mut n = self.core.native_length;
                let _native = lock_native();
                // SAFETY: exact slice length and initialized private workspace
                // are validated before this native call.
                unsafe {
                    slatec_sys::fftpack::$transform(
                        &mut n,
                        values.as_mut_ptr(),
                        self.core.workspace.as_mut_ptr(),
                    )
                };
                Ok(())
            }
        }
    };
}

define_self_inverse_plan!(
    SineTransformPlan,
    sinti,
    sint,
    1,
    sint_workspace,
    "A plan for SLATEC `SINT`, the full sine transform.",
    "`2 * (length + 1)`."
);

define_self_inverse_plan!(
    CosineTransformPlan,
    costi,
    cost,
    2,
    three_n_workspace,
    "A plan for SLATEC `COST`, the full cosine transform.",
    "`2 * (length - 1)`."
);

/// A plan for `SINQI`, `SINQF`, and `SINQB`, the quarter-wave sine family.
///
/// `forward` and `backward` preserve FFTPACK's unnormalised convention: each
/// composition in either order scales the original data by `4 * length`.
#[derive(Debug)]
pub struct QuarterWaveSinePlan {
    core: PlanCore,
}

impl QuarterWaveSinePlan {
    /// Creates a quarter-wave sine plan for `length >= 1`.
    pub fn new(length: usize) -> Result<Self, FftError> {
        Ok(Self {
            core: PlanCore::new(
                length,
                1,
                three_n_workspace(length)?,
                slatec_sys::fftpack::sinqi,
            )?,
        })
    }

    /// Returns the fixed transform length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.core.length
    }

    /// Returns whether the plan has zero transform length.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Applies `SINQF` in place.
    pub fn forward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack::sinqf)
    }

    /// Applies `SINQB` in place.
    pub fn backward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack::sinqb)
    }

    fn apply(
        &mut self,
        values: &mut [f32],
        transform: unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut f32),
    ) -> Result<(), FftError> {
        self.core.check_values(values)?;
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: exact slice length and initialized private workspace are
        // validated before this native call.
        unsafe {
            transform(
                &mut n,
                values.as_mut_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(())
    }
}

/// A plan for `COSQI`, `COSQF`, and `COSQB`, the quarter-wave cosine family.
///
/// `forward` and `backward` preserve FFTPACK's unnormalised convention: each
/// composition in either order scales the original data by `4 * length`.
#[derive(Debug)]
pub struct QuarterWaveCosinePlan {
    core: PlanCore,
}

impl QuarterWaveCosinePlan {
    /// Creates a quarter-wave cosine plan for `length >= 1`.
    pub fn new(length: usize) -> Result<Self, FftError> {
        Ok(Self {
            core: PlanCore::new(
                length,
                1,
                three_n_workspace(length)?,
                slatec_sys::fftpack::cosqi,
            )?,
        })
    }

    /// Returns the fixed transform length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.core.length
    }

    /// Returns whether the plan has zero transform length.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Applies `COSQF` in place.
    pub fn forward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack::cosqf)
    }

    /// Applies `COSQB` in place.
    pub fn backward(&mut self, values: &mut [f32]) -> Result<(), FftError> {
        self.apply(values, slatec_sys::fftpack::cosqb)
    }

    fn apply(
        &mut self,
        values: &mut [f32],
        transform: unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut f32),
    ) -> Result<(), FftError> {
        self.core.check_values(values)?;
        let mut n = self.core.native_length;
        let _native = lock_native();
        // SAFETY: exact slice length and initialized private workspace are
        // validated before this native call.
        unsafe {
            transform(
                &mut n,
                values.as_mut_ptr(),
                self.core.workspace.as_mut_ptr(),
            )
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{CosineTransformPlan, EasyRealFftPlan, FftError, RealFftPlan, SineTransformPlan};

    #[test]
    fn validates_lengths_without_native_entry() {
        assert!(matches!(
            CosineTransformPlan::new(1),
            Err(FftError::InvalidLength {
                length: 1,
                minimum: 2
            })
        ));
        let plan = RealFftPlan::new(1).expect("single-point real FFT plan");
        assert!(matches!(
            plan.spectrum(&[]),
            Err(FftError::LengthMismatch {
                expected: 1,
                actual: 0
            })
        ));
        assert_eq!(SineTransformPlan::new(0).is_err(), true);
        assert_eq!(EasyRealFftPlan::new(usize::MAX).is_err(), true);
    }
}
