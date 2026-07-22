//! Checked general real banded linear systems over LINPACK `SGBFA`/`DGBFA`.
//!
//! `BandMatrixRef` borrows compact column-major storage with logical entry
//! `A[row,col]` at `data[(upper + row - col) + leading_dimension * col]`.
//! Factorization copies that input into private expanded storage with
//! `2*lower + upper + 1` rows, then uses `SGBFA` or `DGBFA`; callers never
//! expose that mutable LU layout. `SGBSL`/`DGBSL` solve `A x=b` and `A^T x=b`.
//! `SGBCO`/`DGBCO` combine factorization with a reciprocal 1-norm condition
//! estimate, while `SGBDI`/`DGBDI` return a scaled determinant from those
//! private factors.

#![cfg(feature = "banded-linear-systems")]

use alloc::vec::Vec;
use core::convert::TryFrom;
use slatec_sys::FortranInteger;
use slatec_sys::linear_algebra::banded as raw;

use crate::runtime::lock_native;

/// A validation failure or documented factorization result for banded systems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BandError {
    /// A dimension or bandwidth is unsupported by the native contract.
    InvalidDimensions,
    /// Checked arithmetic or Fortran-integer conversion overflowed.
    DimensionOverflow,
    /// The compact storage leading dimension is too small.
    LeadingDimensionTooSmall {
        /// Minimum valid leading dimension.
        required: usize,
        /// Caller-supplied leading dimension.
        actual: usize,
    },
    /// The backing slice does not contain every declared storage column.
    StorageLengthTooSmall {
        /// Required scalar elements.
        required: usize,
        /// Available scalar elements.
        actual: usize,
    },
    /// Factorization is limited to square matrices.
    NonSquareMatrix {
        /// Logical row count.
        rows: usize,
        /// Logical column count.
        cols: usize,
    },
    /// A right-hand-side vector has the wrong length.
    RightHandSideLengthMismatch {
        /// Factorization dimension.
        expected: usize,
        /// Supplied vector length.
        actual: usize,
    },
    /// A column-major right-hand-side block has an invalid leading dimension.
    RightHandSideLeadingDimensionTooSmall {
        /// Factorization dimension.
        required: usize,
        /// Supplied right-hand-side leading dimension.
        actual: usize,
    },
    /// The native factorization encountered a zero pivot, using a zero-based index.
    Singular {
        /// Zero-based failing pivot position.
        pivot: usize,
    },
    /// Allocation for private native factor storage failed.
    AllocationFailed,
    /// Native output did not meet the reviewed pivot contract.
    NativeContractViolation,
}

/// A reciprocal estimate of the 1-norm condition number of a factored matrix.
///
/// This is the estimate produced by LINPACK `SGBCO`/`DGBCO`, not the condition
/// number itself. It estimates `1 / (||A||_1 * ||A^-1||_1)`. Zero can mean an
/// exact singularity or that the estimate underflowed; successful factorization
/// is reported separately, so callers must not automatically invert this value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReciprocalCondition<T> {
    value: T,
}

impl<T: Copy> ReciprocalCondition<T> {
    /// Returns the native reciprocal 1-norm condition estimate.
    #[must_use]
    pub fn value(self) -> T {
        self.value
    }
}

/// A base-ten scaled determinant returned by LINPACK `SGBDI`/`DGBDI`.
///
/// The determinant is represented as `mantissa * 10^exponent10`. For a
/// nonzero determinant the native routines normalize the absolute mantissa to
/// the half-open interval `[1, 10)`. If the mantissa is zero, the determinant
/// is zero and the native exponent has no mathematical significance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaledDeterminant<T> {
    mantissa: T,
    exponent10: i32,
}

impl<T: Copy> ScaledDeterminant<T> {
    /// Returns the signed base-ten mantissa.
    #[must_use]
    pub fn mantissa(self) -> T {
        self.mantissa
    }

    /// Returns the base-ten exponent.
    #[must_use]
    pub fn exponent10(self) -> i32 {
        self.exponent10
    }
}

/// The result of reading a logical matrix coordinate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BandElementRef<'a, T> {
    /// The coordinate lies within the declared band and has physical storage.
    Stored(&'a T),
    /// The coordinate is in bounds but structurally zero.
    StructuralZero,
}

/// A checked borrowed view of compact column-major general-band storage.
#[derive(Clone, Copy, Debug)]
pub struct BandMatrixRef<'a, T> {
    data: &'a [T],
    rows: usize,
    cols: usize,
    lower: usize,
    upper: usize,
    lda: usize,
}

impl<'a, T> BandMatrixRef<'a, T> {
    /// Validates compact general-band storage with an explicit leading dimension.
    pub fn from_band_storage(
        data: &'a [T],
        rows: usize,
        cols: usize,
        lower: usize,
        upper: usize,
        lda: usize,
    ) -> Result<Self, BandError> {
        if rows == 0 || cols == 0 || lower >= rows || upper >= cols {
            return Err(BandError::InvalidDimensions);
        }
        let required_lda = lower
            .checked_add(upper)
            .and_then(|v| v.checked_add(1))
            .ok_or(BandError::DimensionOverflow)?;
        if lda < required_lda {
            return Err(BandError::LeadingDimensionTooSmall {
                required: required_lda,
                actual: lda,
            });
        }
        let required = lda.checked_mul(cols).ok_or(BandError::DimensionOverflow)?;
        if data.len() < required {
            return Err(BandError::StorageLengthTooSmall {
                required,
                actual: data.len(),
            });
        }
        let _ = FortranInteger::try_from(rows).map_err(|_| BandError::DimensionOverflow)?;
        let _ = FortranInteger::try_from(lower).map_err(|_| BandError::DimensionOverflow)?;
        let _ = FortranInteger::try_from(upper).map_err(|_| BandError::DimensionOverflow)?;
        Ok(Self {
            data,
            rows,
            cols,
            lower,
            upper,
            lda,
        })
    }
    /// Validates tightly packed compact band storage.
    pub fn from_compact_storage(
        data: &'a [T],
        rows: usize,
        cols: usize,
        lower: usize,
        upper: usize,
    ) -> Result<Self, BandError> {
        let lda = lower
            .checked_add(upper)
            .and_then(|v| v.checked_add(1))
            .ok_or(BandError::DimensionOverflow)?;
        Self::from_band_storage(data, rows, cols, lower, upper, lda)
    }
    /// Returns the logical row count.
    #[must_use]
    pub fn rows(&self) -> usize {
        self.rows
    }
    /// Returns the logical column count.
    #[must_use]
    pub fn cols(&self) -> usize {
        self.cols
    }
    /// Returns the number of subdiagonals.
    #[must_use]
    pub fn lower_bandwidth(&self) -> usize {
        self.lower
    }
    /// Returns the number of superdiagonals.
    #[must_use]
    pub fn upper_bandwidth(&self) -> usize {
        self.upper
    }
    /// Returns the physical storage leading dimension.
    #[must_use]
    pub fn leading_dimension(&self) -> usize {
        self.lda
    }
    /// Distinguishes out-of-bounds coordinates from in-bounds structural zeros.
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<BandElementRef<'a, T>> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        if row.saturating_add(self.upper) < col || col.saturating_add(self.lower) < row {
            return Some(BandElementRef::StructuralZero);
        }
        let band_row = self.upper + row - col;
        Some(BandElementRef::Stored(
            &self.data[band_row + self.lda * col],
        ))
    }
}

/// Reusable single-precision general-band LU factors.
#[derive(Debug)]
pub struct BandLu32 {
    factors: Vec<f32>,
    pivots: Vec<FortranInteger>,
    n: FortranInteger,
    lower: FortranInteger,
    upper: FortranInteger,
    lda: FortranInteger,
}
/// Reusable double-precision general-band LU factors.
#[derive(Debug)]
pub struct BandLu64 {
    factors: Vec<f64>,
    pivots: Vec<FortranInteger>,
    n: FortranInteger,
    lower: FortranInteger,
    upper: FortranInteger,
    lda: FortranInteger,
}

impl BandMatrixRef<'_, f32> {
    /// Copies and factors this square compact matrix.
    pub fn factorize(self) -> Result<BandLu32, BandError> {
        factor32(self)
    }

    /// Copies, factors, and estimates the reciprocal 1-norm condition number.
    ///
    /// This invokes LINPACK `SGBCO`, which computes the original matrix
    /// 1-norm before overwriting its private expanded copy with LU factors.
    /// Exact singular pivots are returned as [`BandError::Singular`]; a zero
    /// estimate with successful factors is a valid underflowed or numerically
    /// singular estimate.
    pub fn factorize_with_condition_estimate(
        self,
    ) -> Result<(BandLu32, ReciprocalCondition<f32>), BandError> {
        factor_with_condition32(self)
    }
}
impl BandMatrixRef<'_, f64> {
    /// Copies and factors this square compact matrix.
    pub fn factorize(self) -> Result<BandLu64, BandError> {
        factor64(self)
    }

    /// Copies, factors, and estimates the reciprocal 1-norm condition number.
    ///
    /// This invokes LINPACK `DGBCO`, which computes the original matrix
    /// 1-norm before overwriting its private expanded copy with LU factors.
    /// Exact singular pivots are returned as [`BandError::Singular`]; a zero
    /// estimate with successful factors is a valid underflowed or numerically
    /// singular estimate.
    pub fn factorize_with_condition_estimate(
        self,
    ) -> Result<(BandLu64, ReciprocalCondition<f64>), BandError> {
        factor_with_condition64(self)
    }
}

impl BandLu32 {
    /// Returns the factorized square dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.n as usize
    }
    /// Returns zero-based partial-pivot row indices.
    pub fn pivots(&self) -> Result<Vec<usize>, BandError> {
        pivots(&self.pivots, self.n)
    }
    /// Solves `A x=b` in place without changing the factors.
    pub fn solve_in_place(&self, rhs: &mut [f32]) -> Result<(), BandError> {
        self.solve(rhs, 0)
    }
    /// Solves `A^T x=b` in place without changing the factors.
    pub fn solve_transpose_in_place(&self, rhs: &mut [f32]) -> Result<(), BandError> {
        self.solve(rhs, 1)
    }
    /// Solves independently stored column-major right-hand sides in place.
    pub fn solve_many_in_place(
        &self,
        rhs: &mut [f32],
        count: usize,
        lda: usize,
    ) -> Result<(), BandError> {
        solve_many32(self, rhs, count, lda, 0)
    }

    /// Returns the normalized base-ten determinant of this immutable LU factorization.
    ///
    /// LINPACK `SGBDI` reads the existing private factors and pivot vector; it
    /// does not consume or mutate this reusable factorization.
    pub fn scaled_determinant(&self) -> Result<ScaledDeterminant<f32>, BandError> {
        determinant32(self)
    }
    fn solve(&self, rhs: &mut [f32], job: FortranInteger) -> Result<(), BandError> {
        solve32(self, rhs, job)
    }
}
impl BandLu64 {
    /// Returns the factorized square dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.n as usize
    }
    /// Returns zero-based partial-pivot row indices.
    pub fn pivots(&self) -> Result<Vec<usize>, BandError> {
        pivots(&self.pivots, self.n)
    }
    /// Solves `A x=b` in place without changing the factors.
    pub fn solve_in_place(&self, rhs: &mut [f64]) -> Result<(), BandError> {
        self.solve(rhs, 0)
    }
    /// Solves `A^T x=b` in place without changing the factors.
    pub fn solve_transpose_in_place(&self, rhs: &mut [f64]) -> Result<(), BandError> {
        self.solve(rhs, 1)
    }
    /// Solves independently stored column-major right-hand sides in place.
    pub fn solve_many_in_place(
        &self,
        rhs: &mut [f64],
        count: usize,
        lda: usize,
    ) -> Result<(), BandError> {
        solve_many64(self, rhs, count, lda, 0)
    }

    /// Returns the normalized base-ten determinant of this immutable LU factorization.
    ///
    /// LINPACK `DGBDI` reads the existing private factors and pivot vector; it
    /// does not consume or mutate this reusable factorization.
    pub fn scaled_determinant(&self) -> Result<ScaledDeterminant<f64>, BandError> {
        determinant64(self)
    }
    fn solve(&self, rhs: &mut [f64], job: FortranInteger) -> Result<(), BandError> {
        solve64(self, rhs, job)
    }
}

fn expanded<T: Copy + Default>(
    m: BandMatrixRef<'_, T>,
) -> Result<
    (
        Vec<T>,
        FortranInteger,
        FortranInteger,
        FortranInteger,
        FortranInteger,
    ),
    BandError,
> {
    if m.rows != m.cols {
        return Err(BandError::NonSquareMatrix {
            rows: m.rows,
            cols: m.cols,
        });
    }
    let lda = m
        .lower
        .checked_mul(2)
        .and_then(|v| v.checked_add(m.upper))
        .and_then(|v| v.checked_add(1))
        .ok_or(BandError::DimensionOverflow)?;
    let size = lda
        .checked_mul(m.cols)
        .ok_or(BandError::DimensionOverflow)?;
    let mut out = Vec::new();
    out.try_reserve_exact(size)
        .map_err(|_| BandError::AllocationFailed)?;
    out.resize(size, T::default());
    for col in 0..m.cols {
        for row in col.saturating_sub(m.upper)..core::cmp::min(m.rows, col + m.lower + 1) {
            let src = m.upper + row - col + m.lda * col;
            let dst = m.lower + m.upper + row - col + lda * col;
            out[dst] = m.data[src];
        }
    }
    Ok((
        out,
        FortranInteger::try_from(m.rows).map_err(|_| BandError::DimensionOverflow)?,
        FortranInteger::try_from(m.lower).map_err(|_| BandError::DimensionOverflow)?,
        FortranInteger::try_from(m.upper).map_err(|_| BandError::DimensionOverflow)?,
        FortranInteger::try_from(lda).map_err(|_| BandError::DimensionOverflow)?,
    ))
}
fn factor32(m: BandMatrixRef<'_, f32>) -> Result<BandLu32, BandError> {
    let (mut a, n, ml, mu, mut lda) = expanded(m)?;
    let mut p = integer_workspace(n)?;
    let mut info = 0;
    {
        let _g = lock_native();
        unsafe {
            raw::sgbfa(
                a.as_mut_ptr(),
                &mut lda,
                &mut (n.clone()),
                &mut (ml.clone()),
                &mut (mu.clone()),
                p.as_mut_ptr(),
                &mut info,
            )
        }
    }
    if info != 0 {
        return Err(BandError::Singular {
            pivot: (info - 1) as usize,
        });
    }
    Ok(BandLu32 {
        factors: a,
        pivots: p,
        n,
        lower: ml,
        upper: mu,
        lda,
    })
}
fn factor64(m: BandMatrixRef<'_, f64>) -> Result<BandLu64, BandError> {
    let (mut a, n, ml, mu, mut lda) = expanded(m)?;
    let mut p = integer_workspace(n)?;
    let mut info = 0;
    {
        let _g = lock_native();
        unsafe {
            raw::dgbfa(
                a.as_mut_ptr(),
                &mut lda,
                &mut (n.clone()),
                &mut (ml.clone()),
                &mut (mu.clone()),
                p.as_mut_ptr(),
                &mut info,
            )
        }
    }
    if info != 0 {
        return Err(BandError::Singular {
            pivot: (info - 1) as usize,
        });
    }
    Ok(BandLu64 {
        factors: a,
        pivots: p,
        n,
        lower: ml,
        upper: mu,
        lda,
    })
}

fn factor_with_condition32(
    m: BandMatrixRef<'_, f32>,
) -> Result<(BandLu32, ReciprocalCondition<f32>), BandError> {
    let (mut a, n, ml, mu, mut lda) = expanded(m)?;
    let mut p = integer_workspace(n)?;
    let mut z = real_workspace32(n)?;
    let mut rcond = 0.0;
    {
        let _g = lock_native();
        unsafe {
            raw::sgbco(
                a.as_mut_ptr(),
                &mut lda,
                &mut (n.clone()),
                &mut (ml.clone()),
                &mut (mu.clone()),
                p.as_mut_ptr(),
                &mut rcond,
                z.as_mut_ptr(),
            )
        }
    }
    if let Some(pivot) = zero_diagonal32(&a, n, ml, mu, lda) {
        return Err(BandError::Singular { pivot });
    }
    if !rcond.is_finite() || rcond < 0.0 {
        return Err(BandError::NativeContractViolation);
    }
    Ok((
        BandLu32 {
            factors: a,
            pivots: p,
            n,
            lower: ml,
            upper: mu,
            lda,
        },
        ReciprocalCondition { value: rcond },
    ))
}

fn factor_with_condition64(
    m: BandMatrixRef<'_, f64>,
) -> Result<(BandLu64, ReciprocalCondition<f64>), BandError> {
    let (mut a, n, ml, mu, mut lda) = expanded(m)?;
    let mut p = integer_workspace(n)?;
    let mut z = real_workspace64(n)?;
    let mut rcond = 0.0;
    {
        let _g = lock_native();
        unsafe {
            raw::dgbco(
                a.as_mut_ptr(),
                &mut lda,
                &mut (n.clone()),
                &mut (ml.clone()),
                &mut (mu.clone()),
                p.as_mut_ptr(),
                &mut rcond,
                z.as_mut_ptr(),
            )
        }
    }
    if let Some(pivot) = zero_diagonal64(&a, n, ml, mu, lda) {
        return Err(BandError::Singular { pivot });
    }
    if !rcond.is_finite() || rcond < 0.0 {
        return Err(BandError::NativeContractViolation);
    }
    Ok((
        BandLu64 {
            factors: a,
            pivots: p,
            n,
            lower: ml,
            upper: mu,
            lda,
        },
        ReciprocalCondition { value: rcond },
    ))
}

fn integer_workspace(n: FortranInteger) -> Result<Vec<FortranInteger>, BandError> {
    let count = usize::try_from(n).map_err(|_| BandError::DimensionOverflow)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| BandError::AllocationFailed)?;
    values.resize(count, 0);
    Ok(values)
}

fn real_workspace32(n: FortranInteger) -> Result<Vec<f32>, BandError> {
    let count = usize::try_from(n).map_err(|_| BandError::DimensionOverflow)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| BandError::AllocationFailed)?;
    values.resize(count, 0.0);
    Ok(values)
}

fn real_workspace64(n: FortranInteger) -> Result<Vec<f64>, BandError> {
    let count = usize::try_from(n).map_err(|_| BandError::DimensionOverflow)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| BandError::AllocationFailed)?;
    values.resize(count, 0.0);
    Ok(values)
}

fn zero_diagonal32(
    factors: &[f32],
    n: FortranInteger,
    lower: FortranInteger,
    upper: FortranInteger,
    lda: FortranInteger,
) -> Option<usize> {
    let row = usize::try_from(lower.checked_add(upper)?).ok()?;
    let stride = usize::try_from(lda).ok()?;
    (0..usize::try_from(n).ok()?)
        .filter(|column| factors[row + stride * column] == 0.0)
        .last()
}

fn zero_diagonal64(
    factors: &[f64],
    n: FortranInteger,
    lower: FortranInteger,
    upper: FortranInteger,
    lda: FortranInteger,
) -> Option<usize> {
    let row = usize::try_from(lower.checked_add(upper)?).ok()?;
    let stride = usize::try_from(lda).ok()?;
    (0..usize::try_from(n).ok()?)
        .filter(|column| factors[row + stride * column] == 0.0)
        .last()
}

fn determinant32(l: &BandLu32) -> Result<ScaledDeterminant<f32>, BandError> {
    let mut det = [0.0; 2];
    let _g = lock_native();
    unsafe {
        raw::sgbdi(
            l.factors.as_ptr(),
            &l.lda,
            &l.n,
            &l.lower,
            &l.upper,
            l.pivots.as_ptr(),
            det.as_mut_ptr(),
        )
    };
    scaled32(det)
}

fn determinant64(l: &BandLu64) -> Result<ScaledDeterminant<f64>, BandError> {
    let mut det = [0.0; 2];
    let _g = lock_native();
    unsafe {
        raw::dgbdi(
            l.factors.as_ptr(),
            &l.lda,
            &l.n,
            &l.lower,
            &l.upper,
            l.pivots.as_ptr(),
            det.as_mut_ptr(),
        )
    };
    scaled64(det)
}

fn scaled32(det: [f32; 2]) -> Result<ScaledDeterminant<f32>, BandError> {
    let exponent = f64::from(det[1]);
    if !det[0].is_finite()
        || !det[1].is_finite()
        || det[1].trunc() != det[1]
        || exponent < f64::from(i32::MIN)
        || exponent > f64::from(i32::MAX)
    {
        return Err(BandError::NativeContractViolation);
    }
    Ok(ScaledDeterminant {
        mantissa: det[0],
        exponent10: det[1] as i32,
    })
}

fn scaled64(det: [f64; 2]) -> Result<ScaledDeterminant<f64>, BandError> {
    if !det[0].is_finite()
        || !det[1].is_finite()
        || det[1].trunc() != det[1]
        || det[1] < f64::from(i32::MIN)
        || det[1] > f64::from(i32::MAX)
    {
        return Err(BandError::NativeContractViolation);
    }
    Ok(ScaledDeterminant {
        mantissa: det[0],
        exponent10: det[1] as i32,
    })
}
fn pivots(p: &[FortranInteger], n: FortranInteger) -> Result<Vec<usize>, BandError> {
    p.iter()
        .map(|&x| {
            if x > 0 && x <= n {
                Ok((x - 1) as usize)
            } else {
                Err(BandError::NativeContractViolation)
            }
        })
        .collect()
}
fn solve32(l: &BandLu32, b: &mut [f32], mut job: FortranInteger) -> Result<(), BandError> {
    if b.len() != l.n as usize {
        return Err(BandError::RightHandSideLengthMismatch {
            expected: l.n as usize,
            actual: b.len(),
        });
    };
    let _g = lock_native();
    let mut n = l.n;
    let mut ml = l.lower;
    let mut mu = l.upper;
    let mut lda = l.lda;
    unsafe {
        raw::sgbsl(
            l.factors.as_ptr() as *mut _,
            &mut lda,
            &mut n,
            &mut ml,
            &mut mu,
            l.pivots.as_ptr() as *mut _,
            b.as_mut_ptr(),
            &mut job,
        )
    };
    Ok(())
}
fn solve64(l: &BandLu64, b: &mut [f64], mut job: FortranInteger) -> Result<(), BandError> {
    if b.len() != l.n as usize {
        return Err(BandError::RightHandSideLengthMismatch {
            expected: l.n as usize,
            actual: b.len(),
        });
    };
    let _g = lock_native();
    let mut n = l.n;
    let mut ml = l.lower;
    let mut mu = l.upper;
    let mut lda = l.lda;
    unsafe {
        raw::dgbsl(
            l.factors.as_ptr() as *mut _,
            &mut lda,
            &mut n,
            &mut ml,
            &mut mu,
            l.pivots.as_ptr() as *mut _,
            b.as_mut_ptr(),
            &mut job,
        )
    };
    Ok(())
}
fn solve_many32(
    l: &BandLu32,
    b: &mut [f32],
    count: usize,
    lda: usize,
    job: FortranInteger,
) -> Result<(), BandError> {
    if lda < l.n as usize {
        return Err(BandError::RightHandSideLeadingDimensionTooSmall {
            required: l.n as usize,
            actual: lda,
        });
    }
    let need = lda.checked_mul(count).ok_or(BandError::DimensionOverflow)?;
    if b.len() < need {
        return Err(BandError::StorageLengthTooSmall {
            required: need,
            actual: b.len(),
        });
    }
    for j in 0..count {
        solve32(l, &mut b[lda * j..lda * j + l.n as usize], job)?
    }
    Ok(())
}
fn solve_many64(
    l: &BandLu64,
    b: &mut [f64],
    count: usize,
    lda: usize,
    job: FortranInteger,
) -> Result<(), BandError> {
    if lda < l.n as usize {
        return Err(BandError::RightHandSideLeadingDimensionTooSmall {
            required: l.n as usize,
            actual: lda,
        });
    }
    let need = lda.checked_mul(count).ok_or(BandError::DimensionOverflow)?;
    if b.len() < need {
        return Err(BandError::StorageLengthTooSmall {
            required: need,
            actual: b.len(),
        });
    }
    for j in 0..count {
        solve64(l, &mut b[lda * j..lda * j + l.n as usize], job)?
    }
    Ok(())
}
