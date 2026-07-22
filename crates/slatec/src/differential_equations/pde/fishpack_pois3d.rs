//! Checked owned access to the structured FISHPACK `POIS3D` system solver.

use alloc::{vec, vec::Vec};
use core::convert::TryFrom;
use core::fmt;
use core::ops::{Index, IndexMut};

use slatec_sys::{FortranInteger, pde::fishpack as raw};

use crate::runtime::lock_native;

/// An owned three-dimensional single-precision grid.
///
/// Values use x-fast storage: `values[z * nx * ny + y * nx + x]` is the
/// value at `(x, y, z)`. This is precisely the contiguous order required by
/// `POIS3D`'s `F(LDIMF, MDIMF, N)` when the private leading dimensions are
/// `nx` and `ny`. Unlike M1's `Grid2`, every dimension is a count of
/// unknowns, not a panel count plus one.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid3 {
    values: Vec<f32>,
    nx: usize,
    ny: usize,
    nz: usize,
}

impl Grid3 {
    /// Creates a grid from owned x-fast values.
    pub fn new(nx: usize, ny: usize, nz: usize, values: Vec<f32>) -> Result<Self, Pois3dError> {
        let expected = checked_volume(nx, ny, nz)?;
        if values.len() != expected {
            return Err(Pois3dError::InvalidStorageLength {
                expected,
                actual: values.len(),
            });
        }
        Ok(Self { values, nx, ny, nz })
    }

    /// Allocates a zero-filled grid with checked volume arithmetic.
    pub fn zeros(nx: usize, ny: usize, nz: usize) -> Result<Self, Pois3dError> {
        let len = checked_volume(nx, ny, nz)?;
        Ok(Self {
            values: allocate_zeroed(len)?,
            nx,
            ny,
            nz,
        })
    }

    /// Returns the first-index extent.
    #[must_use]
    pub fn nx(&self) -> usize {
        self.nx
    }

    /// Returns the second-index extent.
    #[must_use]
    pub fn ny(&self) -> usize {
        self.ny
    }

    /// Returns the third-index extent.
    #[must_use]
    pub fn nz(&self) -> usize {
        self.nz
    }

    /// Returns the x-fast backing storage.
    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    /// Returns mutable x-fast backing storage.
    #[must_use]
    pub fn values_mut(&mut self) -> &mut [f32] {
        &mut self.values
    }

    /// Returns a value when all indices are in range.
    #[must_use]
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&f32> {
        self.offset(x, y, z).map(|index| &self.values[index])
    }

    /// Returns a mutable value when all indices are in range.
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut f32> {
        self.offset(x, y, z).map(|index| &mut self.values[index])
    }

    fn offset(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        (x < self.nx && y < self.ny && z < self.nz)
            .then_some(z * self.nx * self.ny + y * self.nx + x)
    }
}

impl Index<(usize, usize, usize)> for Grid3 {
    type Output = f32;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        self.get(index.0, index.1, index.2)
            .expect("Grid3 index is within the documented dimensions")
    }
}

impl IndexMut<(usize, usize, usize)> for Grid3 {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1, index.2)
            .expect("Grid3 index is within the documented dimensions")
    }
}

/// A first- or second-index ghost-point rule accepted by `POIS3D`.
///
/// The same encoding applies to the first index `i` and second index `j`.
/// The descriptions below use a generic sequence `X(1)..X(Q)` and its ghost
/// nodes `X(0)` and `X(Q+1)`; no nonzero face values are accepted by this
/// structured solver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransverseBoundary {
    /// `X(0)=X(Q)` and `X(Q+1)=X(1)`.
    Periodic,
    /// `X(0)=0` and `X(Q+1)=0`.
    ZeroBoth,
    /// `X(0)=0` and `X(Q+1)=X(Q-1)`.
    ZeroLowerReflectUpper,
    /// `X(0)=X(2)` and `X(Q+1)=X(Q-1)`.
    ReflectBoth,
    /// `X(0)=X(2)` and `X(Q+1)=0`.
    ReflectLowerZeroUpper,
}

impl TransverseBoundary {
    fn native_code(self) -> FortranInteger {
        match self {
            Self::Periodic => 0,
            Self::ZeroBoth => 1,
            Self::ZeroLowerReflectUpper => 2,
            Self::ReflectBoth => 3,
            Self::ReflectLowerZeroUpper => 4,
        }
    }
}

/// Constant coefficients for a cyclic third-index operator.
///
/// At every `k`, the lower and upper coefficients are the same nonzero
/// `off_diagonal` value and the diagonal is `diagonal`; indices wrap between
/// `1` and `N`. This is the exact condition checked by `POIS3D` for
/// `NPEROD=0`, represented without exposing an invalid raw-array form.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CyclicAxisCoefficients {
    off_diagonal: f32,
    diagonal: f32,
}

impl CyclicAxisCoefficients {
    /// Creates finite cyclic coefficients with a nonzero wrapping coupling.
    pub fn new(off_diagonal: f32, diagonal: f32) -> Result<Self, Pois3dError> {
        if !off_diagonal.is_finite() || !diagonal.is_finite() {
            return Err(Pois3dError::NonFiniteInput {
                field: "cyclic third-axis coefficient",
            });
        }
        if off_diagonal == 0.0 {
            return Err(Pois3dError::CyclicCouplingMustBeNonzero);
        }
        Ok(Self {
            off_diagonal,
            diagonal,
        })
    }

    /// Returns the common lower and upper coefficient.
    #[must_use]
    pub fn off_diagonal(self) -> f32 {
        self.off_diagonal
    }

    /// Returns the constant diagonal coefficient.
    #[must_use]
    pub fn diagonal(self) -> f32 {
        self.diagonal
    }
}

/// Owned noncyclic tridiagonal third-index coefficients.
#[derive(Clone, Debug, PartialEq)]
pub struct TridiagonalAxisCoefficients {
    lower: Vec<f32>,
    diagonal: Vec<f32>,
    upper: Vec<f32>,
}

impl TridiagonalAxisCoefficients {
    /// Creates finite tridiagonal coefficients with the required zero endpoints.
    pub fn new(lower: Vec<f32>, diagonal: Vec<f32>, upper: Vec<f32>) -> Result<Self, Pois3dError> {
        if lower.is_empty() {
            return Err(Pois3dError::CoefficientVectorsEmpty);
        }
        if lower.len() != diagonal.len() || lower.len() != upper.len() {
            return Err(Pois3dError::CoefficientLengthMismatch {
                lower: lower.len(),
                diagonal: diagonal.len(),
                upper: upper.len(),
            });
        }
        if lower
            .iter()
            .chain(diagonal.iter())
            .chain(upper.iter())
            .any(|value| !value.is_finite())
        {
            return Err(Pois3dError::NonFiniteInput {
                field: "tridiagonal third-axis coefficient",
            });
        }
        if lower[0] != 0.0 {
            return Err(Pois3dError::NoncyclicLowerEndpointMustBeZero { value: lower[0] });
        }
        if upper[upper.len() - 1] != 0.0 {
            return Err(Pois3dError::NoncyclicUpperEndpointMustBeZero {
                value: upper[upper.len() - 1],
            });
        }
        Ok(Self {
            lower,
            diagonal,
            upper,
        })
    }

    /// Returns the lower tridiagonal coefficients, including the leading zero.
    #[must_use]
    pub fn lower(&self) -> &[f32] {
        &self.lower
    }

    /// Returns the diagonal coefficients.
    #[must_use]
    pub fn diagonal(&self) -> &[f32] {
        &self.diagonal
    }

    /// Returns the upper tridiagonal coefficients, including the trailing zero.
    #[must_use]
    pub fn upper(&self) -> &[f32] {
        &self.upper
    }
}

/// The third-index operator model accepted by `POIS3D`.
#[derive(Clone, Debug, PartialEq)]
pub enum ThirdAxisOperator {
    /// A wrapping operator with one constant off-diagonal and diagonal value.
    Cyclic(CyclicAxisCoefficients),
    /// A nonwrapping operator with independently owned tridiagonal vectors.
    Tridiagonal(TridiagonalAxisCoefficients),
}

impl ThirdAxisOperator {
    fn validate_for_length(&self, n: usize) -> Result<(), Pois3dError> {
        if let Self::Tridiagonal(coefficients) = self {
            if coefficients.lower.len() != n {
                return Err(Pois3dError::ThirdAxisLengthMismatch {
                    expected: n,
                    actual: coefficients.lower.len(),
                });
            }
        }
        Ok(())
    }

    fn into_native_coefficients(self, n: usize) -> (FortranInteger, Vec<f32>, Vec<f32>, Vec<f32>) {
        match self {
            Self::Cyclic(coefficients) => (
                0,
                vec![coefficients.off_diagonal; n],
                vec![coefficients.diagonal; n],
                vec![coefficients.off_diagonal; n],
            ),
            Self::Tridiagonal(coefficients) => (
                1,
                coefficients.lower,
                coefficients.diagonal,
                coefficients.upper,
            ),
        }
    }
}

/// A checked structured three-dimensional system for `POIS3D`.
///
/// It solves, for `i=1..L`, `j=1..M`, and `k=1..N`, the discrete system
/// `C1*(X(i-1,j,k)-2X(i,j,k)+X(i+1,j,k)) +`
/// `C2*(X(i,j-1,k)-2X(i,j,k)+X(i,j+1,k)) +`
/// `A(k)X(i,j,k-1)+B(k)X(i,j,k)+C(k)X(i,j,k+1)=F(i,j,k)`.
/// Physical spacings are not passed separately: callers incorporate them in
/// `c1`, `c2`, and the third-axis coefficients. The problem owns its RHS and
/// solving consumes it, returning the RHS allocation as the solution grid.
#[derive(Clone, Debug, PartialEq)]
pub struct Pois3dProblem {
    l_boundary: TransverseBoundary,
    m_boundary: TransverseBoundary,
    c1: f32,
    c2: f32,
    third_axis: ThirdAxisOperator,
    rhs: Grid3,
}

impl Pois3dProblem {
    /// Validates and owns one `POIS3D` structured system.
    pub fn new(
        l_boundary: TransverseBoundary,
        m_boundary: TransverseBoundary,
        c1: f32,
        c2: f32,
        third_axis: ThirdAxisOperator,
        rhs: Grid3,
    ) -> Result<Self, Pois3dError> {
        validate_dimension("first", rhs.nx)?;
        validate_dimension("second", rhs.ny)?;
        validate_dimension("third", rhs.nz)?;
        if !c1.is_finite() || !c2.is_finite() {
            return Err(Pois3dError::NonFiniteInput {
                field: "transverse coefficient",
            });
        }
        if rhs.values.iter().any(|value| !value.is_finite()) {
            return Err(Pois3dError::NonFiniteInput {
                field: "right-hand side",
            });
        }
        third_axis.validate_for_length(rhs.nz)?;
        Ok(Self {
            l_boundary,
            m_boundary,
            c1,
            c2,
            third_axis,
            rhs,
        })
    }

    /// Solves the validated structured system through the serialized native driver.
    ///
    /// No singularity or perturbation status is documented by `POIS3D`; a
    /// successful native return therefore yields only the owned solution.
    pub fn solve(self) -> Result<Grid3, Pois3dError> {
        let l = checked_fortran_dimension(self.rhs.nx)?;
        let m = checked_fortran_dimension(self.rhs.ny)?;
        let n = checked_fortran_dimension(self.rhs.nz)?;
        let ldimf = l;
        let mdimf = m;
        let workspace_len = workspace_len(self.rhs.nx, self.rhs.ny, self.rhs.nz)?;
        let mut workspace = allocate_zeroed(workspace_len)?;
        let mut values = self.rhs.values;
        let (nperod, mut lower, mut diagonal, mut upper) =
            self.third_axis.into_native_coefficients(self.rhs.nz);
        let lperod = self.l_boundary.native_code();
        let mperod = self.m_boundary.native_code();
        let mut native_code = 0;
        let _native = lock_native();
        // SAFETY: construction checked logical dimensions, finite caller
        // inputs, operator restrictions, and native-integer conversion. All
        // passed arrays are owned, contiguous, and exactly sized for the
        // reviewed `F(LDIMF,MDIMF,N)`, `A(N)`, `B(N)`, `C(N)`, and workspace
        // contracts. The process-global native lock covers the whole call.
        unsafe {
            raw::pois3d(
                &lperod,
                &l,
                &self.c1,
                &mperod,
                &m,
                &self.c2,
                &nperod,
                &n,
                lower.as_mut_ptr(),
                diagonal.as_mut_ptr(),
                upper.as_mut_ptr(),
                &ldimf,
                &mdimf,
                values.as_mut_ptr(),
                &mut native_code,
                workspace.as_mut_ptr(),
            );
        }
        if native_code != 0 {
            return Err(Pois3dError::NativeFailure { code: native_code });
        }
        Ok(Grid3 {
            values,
            nx: self.rhs.nx,
            ny: self.rhs.ny,
            nz: self.rhs.nz,
        })
    }
}

/// Validation or unexpected native completion failure for `POIS3D`.
#[derive(Clone, Debug, PartialEq)]
pub enum Pois3dError {
    /// A grid dimension has fewer than the three unknowns required by `POIS3D`.
    GridTooSmall {
        /// Human-readable dimension name.
        axis: &'static str,
        /// Requested unknown count.
        count: usize,
        /// Native minimum unknown count.
        minimum: usize,
    },
    /// Checked volume, workspace, or native-integer arithmetic overflowed.
    DimensionOverflow,
    /// A supplied backing vector has the wrong checked volume.
    InvalidStorageLength {
        /// Required value count.
        expected: usize,
        /// Supplied value count.
        actual: usize,
    },
    /// An RHS or coefficient value was not finite.
    NonFiniteInput {
        /// Human-readable input category.
        field: &'static str,
    },
    /// The three noncyclic coefficient vectors do not have one common length.
    CoefficientLengthMismatch {
        /// Lower-vector length.
        lower: usize,
        /// Diagonal-vector length.
        diagonal: usize,
        /// Upper-vector length.
        upper: usize,
    },
    /// A noncyclic coefficient model was constructed without any coefficients.
    CoefficientVectorsEmpty,
    /// A noncyclic model's vector length does not equal the grid's third extent.
    ThirdAxisLengthMismatch {
        /// Required third extent.
        expected: usize,
        /// Coefficient-vector length.
        actual: usize,
    },
    /// The lower first noncyclic coefficient was not exactly zero.
    NoncyclicLowerEndpointMustBeZero {
        /// Rejected lower coefficient.
        value: f32,
    },
    /// The upper last noncyclic coefficient was not exactly zero.
    NoncyclicUpperEndpointMustBeZero {
        /// Rejected upper coefficient.
        value: f32,
    },
    /// A cyclic model had no wrapping off-diagonal coupling.
    CyclicCouplingMustBeNonzero,
    /// A private, prevalidated native call returned an unexpected `IERROR`.
    NativeFailure {
        /// Exact native `IERROR` code.
        code: i32,
    },
    /// A checked native workspace allocation could not be reserved.
    AllocationFailed,
}

impl fmt::Display for Pois3dError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GridTooSmall {
                axis,
                count,
                minimum,
            } => write!(
                formatter,
                "{axis} dimension has {count} unknowns; POIS3D requires at least {minimum}"
            ),
            Self::DimensionOverflow => {
                formatter.write_str("POIS3D dimension or workspace arithmetic overflowed")
            }
            Self::InvalidStorageLength { expected, actual } => {
                write!(formatter, "Grid3 has {actual} values; expected {expected}")
            }
            Self::NonFiniteInput { field } => write!(formatter, "{field} must be finite"),
            Self::CoefficientLengthMismatch {
                lower,
                diagonal,
                upper,
            } => write!(
                formatter,
                "third-axis coefficient lengths differ: lower={lower}, diagonal={diagonal}, upper={upper}"
            ),
            Self::CoefficientVectorsEmpty => {
                formatter.write_str("third-axis coefficient vectors cannot be empty")
            }
            Self::ThirdAxisLengthMismatch { expected, actual } => write!(
                formatter,
                "third-axis coefficient length is {actual}; expected grid extent {expected}"
            ),
            Self::NoncyclicLowerEndpointMustBeZero { value } => {
                write!(formatter, "noncyclic lower[0] must be zero, got {value}")
            }
            Self::NoncyclicUpperEndpointMustBeZero { value } => {
                write!(formatter, "noncyclic upper[N-1] must be zero, got {value}")
            }
            Self::CyclicCouplingMustBeNonzero => {
                formatter.write_str("cyclic third-axis off-diagonal coupling must be nonzero")
            }
            Self::NativeFailure { code } => {
                write!(
                    formatter,
                    "POIS3D returned unexpected native error code {code}"
                )
            }
            Self::AllocationFailed => formatter.write_str("POIS3D workspace allocation failed"),
        }
    }
}

impl std::error::Error for Pois3dError {}

fn validate_dimension(axis: &'static str, count: usize) -> Result<(), Pois3dError> {
    if count < 3 {
        return Err(Pois3dError::GridTooSmall {
            axis,
            count,
            minimum: 3,
        });
    }
    Ok(())
}

fn checked_fortran_dimension(value: usize) -> Result<FortranInteger, Pois3dError> {
    FortranInteger::try_from(value).map_err(|_| Pois3dError::DimensionOverflow)
}

fn checked_volume(nx: usize, ny: usize, nz: usize) -> Result<usize, Pois3dError> {
    nx.checked_mul(ny)
        .and_then(|area| area.checked_mul(nz))
        .ok_or(Pois3dError::DimensionOverflow)
}

fn allocate_zeroed(length: usize) -> Result<Vec<f32>, Pois3dError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Pois3dError::AllocationFailed)?;
    values.resize(length, 0.0);
    Ok(values)
}

fn workspace_len(l: usize, m: usize, n: usize) -> Result<usize, Pois3dError> {
    let half_sum = l
        .checked_add(1)
        .map(|value| value / 2)
        .and_then(|left| {
            m.checked_add(1)
                .map(|value| value / 2)
                .and_then(|right| left.checked_add(right))
        })
        .ok_or(Pois3dError::DimensionOverflow)?;
    30_usize
        .checked_add(l)
        .and_then(|value| value.checked_add(m))
        .and_then(|value| n.checked_mul(2).and_then(|twice| value.checked_add(twice)))
        .and_then(|value| value.checked_add(l.max(m).max(n)))
        .and_then(|value| {
            half_sum
                .checked_mul(7)
                .and_then(|tail| value.checked_add(tail))
        })
        .ok_or(Pois3dError::DimensionOverflow)
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{
        CyclicAxisCoefficients, Grid3, Pois3dError, Pois3dProblem, ThirdAxisOperator,
        TransverseBoundary, TridiagonalAxisCoefficients, checked_fortran_dimension, workspace_len,
    };

    #[test]
    fn grid_is_x_fast_and_three_dimensional() {
        let grid = Grid3::new(2, 3, 4, (0..24).map(|value| value as f32).collect()).unwrap();
        assert_eq!(grid[(1, 0, 0)], 1.0);
        assert_eq!(grid[(0, 1, 0)], 2.0);
        assert_eq!(grid[(0, 0, 1)], 6.0);
        assert_eq!(grid[(1, 2, 3)], 23.0);
    }

    #[test]
    fn validates_grid_and_dimension_overflow() {
        assert!(matches!(
            Grid3::new(2, 3, 4, vec![0.0; 23]),
            Err(Pois3dError::InvalidStorageLength { .. })
        ));
        assert!(matches!(
            Grid3::zeros(usize::MAX, 2, 2),
            Err(Pois3dError::DimensionOverflow)
        ));
        assert!(matches!(
            workspace_len(usize::MAX, 3, 3),
            Err(Pois3dError::DimensionOverflow)
        ));
        assert!(matches!(
            checked_fortran_dimension(usize::MAX),
            Err(Pois3dError::DimensionOverflow)
        ));
    }

    #[test]
    fn validates_structured_operator_models() {
        assert!(matches!(
            CyclicAxisCoefficients::new(0.0, 1.0),
            Err(Pois3dError::CyclicCouplingMustBeNonzero)
        ));
        assert!(matches!(
            TridiagonalAxisCoefficients::new(vec![0.0, 1.0], vec![1.0], vec![1.0, 0.0]),
            Err(Pois3dError::CoefficientLengthMismatch { .. })
        ));
        assert!(matches!(
            TridiagonalAxisCoefficients::new(
                vec![1.0, 1.0, 1.0],
                vec![1.0; 3],
                vec![1.0, 1.0, 0.0]
            ),
            Err(Pois3dError::NoncyclicLowerEndpointMustBeZero { .. })
        ));
        assert!(matches!(
            TridiagonalAxisCoefficients::new(
                vec![0.0, 1.0, 1.0],
                vec![1.0; 3],
                vec![1.0, 1.0, 1.0]
            ),
            Err(Pois3dError::NoncyclicUpperEndpointMustBeZero { .. })
        ));
    }

    #[test]
    fn problem_requires_native_minimums_and_matching_third_axis() {
        let cyclic = ThirdAxisOperator::Cyclic(CyclicAxisCoefficients::new(1.0, -2.0).unwrap());
        assert!(matches!(
            Pois3dProblem::new(
                TransverseBoundary::Periodic,
                TransverseBoundary::ZeroBoth,
                1.0,
                1.0,
                cyclic,
                Grid3::zeros(2, 3, 3).unwrap(),
            ),
            Err(Pois3dError::GridTooSmall { axis: "first", .. })
        ));
        let tridiagonal = ThirdAxisOperator::Tridiagonal(
            TridiagonalAxisCoefficients::new(
                vec![0.0, 1.0, 1.0],
                vec![1.0; 3],
                vec![1.0, 1.0, 0.0],
            )
            .unwrap(),
        );
        assert!(matches!(
            Pois3dProblem::new(
                TransverseBoundary::ReflectBoth,
                TransverseBoundary::ZeroLowerReflectUpper,
                1.0,
                1.0,
                tridiagonal,
                Grid3::zeros(3, 4, 4).unwrap(),
            ),
            Err(Pois3dError::ThirdAxisLengthMismatch { .. })
        ));
    }
}
