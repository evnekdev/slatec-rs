//! Partial differential equations.
//!
//! The `fishpack-cartesian-2d` feature exposes a checked, owned facade over
//! the selected single-precision FISHPACK `HWSCRT` driver.  It solves a
//! constant-coefficient Cartesian Poisson/Helmholtz finite-difference problem
//! on a uniform rectangular grid.  Other PDE families remain planned.

#[cfg(feature = "fishpack-cartesian-2d")]
mod fishpack_cartesian_2d {
    use alloc::vec::Vec;
    use core::convert::TryFrom;
    use core::fmt;
    use core::ops::{Index, IndexMut};

    use slatec_sys::FortranInteger;

    use crate::runtime::lock_native;

    /// One coordinate axis of a uniform Cartesian grid.
    ///
    /// `intervals` is the number of equal panels, so this axis has
    /// `intervals + 1` nodes, including both endpoints.  `HWSCRT` requires at
    /// least four intervals on each axis.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct UniformAxis {
        lower: f32,
        upper: f32,
        intervals: usize,
    }

    impl UniformAxis {
        /// Creates a finite, strictly increasing uniform axis with at least four panels.
        pub fn new(lower: f32, upper: f32, intervals: usize) -> Result<Self, PdeError> {
            if !lower.is_finite() || !upper.is_finite() {
                return Err(PdeError::NonFiniteInput {
                    field: "axis endpoint",
                });
            }
            if lower >= upper {
                return Err(PdeError::InvalidAxis);
            }
            if intervals < 4 {
                return Err(PdeError::GridTooSmall {
                    intervals,
                    minimum: 4,
                });
            }
            Ok(Self {
                lower,
                upper,
                intervals,
            })
        }

        /// Returns the lower endpoint.
        #[must_use]
        pub fn lower(self) -> f32 {
            self.lower
        }

        /// Returns the upper endpoint.
        #[must_use]
        pub fn upper(self) -> f32 {
            self.upper
        }

        /// Returns the number of equal panels.
        #[must_use]
        pub fn intervals(self) -> usize {
            self.intervals
        }

        /// Returns the number of nodes, including both endpoints.
        pub fn nodes(self) -> Result<usize, PdeError> {
            self.intervals
                .checked_add(1)
                .ok_or(PdeError::DimensionOverflow)
        }

        /// Returns the uniform grid spacing.
        #[must_use]
        pub fn spacing(self) -> f32 {
            (self.upper - self.lower) / self.intervals as f32
        }
    }

    /// An owned two-dimensional `f32` grid.
    ///
    /// Values use row-major storage with `(x, y)` indexing: the x-coordinate
    /// varies fastest and `values[y * nx + x]` is the value at `(x, y)`.  This
    /// is also the contiguous memory order expected by Fortran
    /// `F(IDIMF, N + 1)` when `IDIMF == nx`; no public leading dimension is
    /// exposed.  Grids include every boundary node.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Grid2 {
        values: Vec<f32>,
        nx: usize,
        ny: usize,
    }

    impl Grid2 {
        /// Creates a grid from owned x-fast row-major values.
        pub fn new(nx: usize, ny: usize, values: Vec<f32>) -> Result<Self, PdeError> {
            let expected = checked_area(nx, ny)?;
            if values.len() != expected {
                return Err(PdeError::InvalidRightHandSideShape {
                    expected,
                    actual: values.len(),
                });
            }
            Ok(Self { values, nx, ny })
        }

        /// Allocates a zero-filled grid with checked dimensions.
        pub fn zeros(nx: usize, ny: usize) -> Result<Self, PdeError> {
            let len = checked_area(nx, ny)?;
            let values = allocate_zeroed(len)?;
            Ok(Self { values, nx, ny })
        }

        /// Returns the number of x nodes.
        #[must_use]
        pub fn nx(&self) -> usize {
            self.nx
        }

        /// Returns the number of y nodes.
        #[must_use]
        pub fn ny(&self) -> usize {
            self.ny
        }

        /// Returns the x-fast row-major values.
        #[must_use]
        pub fn values(&self) -> &[f32] {
            &self.values
        }

        /// Returns mutable x-fast row-major values.
        #[must_use]
        pub fn values_mut(&mut self) -> &mut [f32] {
            &mut self.values
        }

        /// Returns the value at `(x, y)`, or `None` when either index is out of bounds.
        #[must_use]
        pub fn get(&self, x: usize, y: usize) -> Option<&f32> {
            self.offset(x, y).map(|index| &self.values[index])
        }

        /// Returns the mutable value at `(x, y)`, or `None` when out of bounds.
        pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
            self.offset(x, y).map(|index| &mut self.values[index])
        }

        fn offset(&self, x: usize, y: usize) -> Option<usize> {
            (x < self.nx && y < self.ny).then_some(y * self.nx + x)
        }
    }

    impl Index<(usize, usize)> for Grid2 {
        type Output = f32;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            self.get(index.0, index.1)
                .expect("Grid2 index is within the documented dimensions")
        }
    }

    impl IndexMut<(usize, usize)> for Grid2 {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            self.get_mut(index.0, index.1)
                .expect("Grid2 index is within the documented dimensions")
        }
    }

    /// Boundary conditions for one increasing-coordinate axis.
    ///
    /// Every edge vector contains all nodes on that edge, corners included.
    /// On the x axis its length is `y.nodes()`; on the y axis its length is
    /// `x.nodes()`.  Derivatives are with respect to the increasing coordinate
    /// (`dU/dx` or `dU/dy`), not outward normals.  Inputs are read-only from
    /// Rust's perspective; `HWSCRT` does not overwrite them.
    #[derive(Clone, Debug, PartialEq)]
    pub enum AxisBoundary {
        /// Code 0: identify the two endpoints periodically.
        Periodic,
        /// Code 1: prescribe values at both endpoints.
        Dirichlet {
            /// Values at the lower-coordinate endpoint.
            lower: Vec<f32>,
            /// Values at the upper-coordinate endpoint.
            upper: Vec<f32>,
        },
        /// Code 2: prescribe a lower value and upper increasing-coordinate derivative.
        DirichletNeumann {
            /// Values at the lower-coordinate endpoint.
            lower: Vec<f32>,
            /// Derivatives at the upper-coordinate endpoint.
            upper_derivative: Vec<f32>,
        },
        /// Code 3: prescribe increasing-coordinate derivatives at both endpoints.
        Neumann {
            /// Derivatives at the lower-coordinate endpoint.
            lower_derivative: Vec<f32>,
            /// Derivatives at the upper-coordinate endpoint.
            upper_derivative: Vec<f32>,
        },
        /// Code 4: prescribe a lower increasing-coordinate derivative and upper value.
        NeumannDirichlet {
            /// Derivatives at the lower-coordinate endpoint.
            lower_derivative: Vec<f32>,
            /// Values at the upper-coordinate endpoint.
            upper: Vec<f32>,
        },
    }

    impl AxisBoundary {
        fn native_code(&self) -> FortranInteger {
            match self {
                Self::Periodic => 0,
                Self::Dirichlet { .. } => 1,
                Self::DirichletNeumann { .. } => 2,
                Self::Neumann { .. } => 3,
                Self::NeumannDirichlet { .. } => 4,
            }
        }

        fn lower_value(&self) -> Option<&[f32]> {
            match self {
                Self::Dirichlet { lower, .. } | Self::DirichletNeumann { lower, .. } => Some(lower),
                Self::Periodic | Self::Neumann { .. } | Self::NeumannDirichlet { .. } => None,
            }
        }

        fn upper_value(&self) -> Option<&[f32]> {
            match self {
                Self::Dirichlet { upper, .. } | Self::NeumannDirichlet { upper, .. } => Some(upper),
                Self::Periodic | Self::DirichletNeumann { .. } | Self::Neumann { .. } => None,
            }
        }

        fn lower_derivative(&self) -> Option<&[f32]> {
            match self {
                Self::Neumann {
                    lower_derivative, ..
                }
                | Self::NeumannDirichlet {
                    lower_derivative, ..
                } => Some(lower_derivative),
                Self::Periodic | Self::Dirichlet { .. } | Self::DirichletNeumann { .. } => None,
            }
        }

        fn upper_derivative(&self) -> Option<&[f32]> {
            match self {
                Self::DirichletNeumann {
                    upper_derivative, ..
                }
                | Self::Neumann {
                    upper_derivative, ..
                } => Some(upper_derivative),
                Self::Periodic | Self::Dirichlet { .. } | Self::NeumannDirichlet { .. } => None,
            }
        }

        fn validate(&self, expected: usize, axis: &'static str) -> Result<(), PdeError> {
            for values in [
                self.lower_value(),
                self.upper_value(),
                self.lower_derivative(),
                self.upper_derivative(),
            ]
            .into_iter()
            .flatten()
            {
                if values.len() != expected {
                    return Err(PdeError::InvalidBoundaryLength {
                        axis,
                        expected,
                        actual: values.len(),
                    });
                }
                if values.iter().any(|value| !value.is_finite()) {
                    return Err(PdeError::NonFiniteInput {
                        field: "boundary data",
                    });
                }
            }
            Ok(())
        }
    }

    /// A checked Cartesian problem for the selected `HWSCRT` driver.
    ///
    /// The discrete equation at interior nodes is
    /// `(u[x-1,y] - 2u[x,y] + u[x+1,y]) / dx^2 +`
    /// `(u[x,y-1] - 2u[x,y] + u[x,y+1]) / dy^2 + coefficient * u[x,y] = rhs[x,y]`.
    /// The problem owns all inputs.  Solving consumes it and reuses the RHS
    /// allocation as the returned solution grid.
    #[derive(Clone, Debug, PartialEq)]
    pub struct CartesianHelmholtz2d {
        x: UniformAxis,
        y: UniformAxis,
        coefficient: f32,
        rhs: Grid2,
        x_boundary: AxisBoundary,
        y_boundary: AxisBoundary,
    }

    impl CartesianHelmholtz2d {
        /// Validates and owns one Cartesian Poisson/Helmholtz problem.
        ///
        /// `rhs` must have exactly `(x.nodes(), y.nodes())` values.  Dirichlet
        /// edges override RHS samples at their nodes.  When a Dirichlet corner
        /// is supplied by both axes, the two finite values must compare equal;
        /// otherwise construction rejects the ambiguity.  For a periodic axis,
        /// RHS values at its duplicate endpoint edges must compare equal.
        pub fn new(
            x: UniformAxis,
            y: UniformAxis,
            coefficient: f32,
            rhs: Grid2,
            x_boundary: AxisBoundary,
            y_boundary: AxisBoundary,
        ) -> Result<Self, PdeError> {
            if !coefficient.is_finite() {
                return Err(PdeError::NonFiniteInput {
                    field: "Helmholtz coefficient",
                });
            }
            let nx = x.nodes()?;
            let ny = y.nodes()?;
            let expected = checked_area(nx, ny)?;
            if rhs.nx != nx || rhs.ny != ny {
                return Err(PdeError::InvalidRightHandSideShape {
                    expected,
                    actual: checked_area(rhs.nx, rhs.ny)?,
                });
            }
            if rhs.values.iter().any(|value| !value.is_finite()) {
                return Err(PdeError::NonFiniteInput {
                    field: "right-hand side",
                });
            }
            x_boundary.validate(ny, "x")?;
            y_boundary.validate(nx, "y")?;
            validate_periodic_rhs(&rhs, &x_boundary, &y_boundary)?;
            validate_corners(nx, ny, &x_boundary, &y_boundary)?;
            Ok(Self {
                x,
                y,
                coefficient,
                rhs,
                x_boundary,
                y_boundary,
            })
        }

        /// Solves the checked finite-difference problem through `HWSCRT`.
        ///
        /// The native call and all reachable FISHPACK/BLAS state are guarded
        /// by the crate's process-global native lock.  The result always owns
        /// its grid and never exposes a Fortran leading dimension or workspace.
        pub fn solve(self) -> Result<CartesianPdeSolution, PdeError> {
            let nx = self.x.nodes()?;
            let ny = self.y.nodes()?;
            let m = FortranInteger::try_from(self.x.intervals)
                .map_err(|_| PdeError::DimensionOverflow)?;
            let n = FortranInteger::try_from(self.y.intervals)
                .map_err(|_| PdeError::DimensionOverflow)?;
            let idimf = FortranInteger::try_from(nx).map_err(|_| PdeError::DimensionOverflow)?;
            let workspace_len = workspace_len(nx, ny)?;
            let mut workspace = allocate_zeroed(workspace_len)?;
            let mut values = self.rhs.values;
            apply_dirichlet_edges(nx, ny, &mut values, &self.x_boundary, &self.y_boundary);

            let dummy = [0.0_f32];
            let bda = self.x_boundary.lower_derivative().unwrap_or(&dummy);
            let bdb = self.x_boundary.upper_derivative().unwrap_or(&dummy);
            let bdc = self.y_boundary.lower_derivative().unwrap_or(&dummy);
            let bdd = self.y_boundary.upper_derivative().unwrap_or(&dummy);
            let mut perturbation = 0.0;
            let mut native_code = 0;
            let x_code = self.x_boundary.native_code();
            let y_code = self.y_boundary.native_code();
            let _native = lock_native();
            // SAFETY: all scalar values have the selected GNU Fortran ABI;
            // every passed slice is contiguous, live for the entire call, and
            // has the exact audited dimension.  `values` is x-fast/Fortran
            // column-major with first dimension `nx`; only it and workspace
            // are mutable native arrays.
            unsafe {
                slatec_sys::fishpack_cartesian_2d::hwscrt(
                    &self.x.lower,
                    &self.x.upper,
                    &m,
                    &x_code,
                    bda.as_ptr(),
                    bdb.as_ptr(),
                    &self.y.lower,
                    &self.y.upper,
                    &n,
                    &y_code,
                    bdc.as_ptr(),
                    bdd.as_ptr(),
                    &self.coefficient,
                    values.as_mut_ptr(),
                    &idimf,
                    &mut perturbation,
                    &mut native_code,
                    workspace.as_mut_ptr(),
                );
            }
            if native_code != 0 && native_code != 6 {
                return Err(PdeError::NativeFailure { code: native_code });
            }
            if !perturbation.is_finite() {
                return Err(PdeError::NativeFailure { code: native_code });
            }
            let reported_workspace = workspace[0];
            if !reported_workspace.is_finite()
                || reported_workspace < 1.0
                || reported_workspace > workspace_len as f32
            {
                return Err(PdeError::InconsistentNativeWorkspace {
                    reported: reported_workspace,
                    allocated: workspace_len,
                });
            }
            let singular = self.coefficient == 0.0
                && is_singular_axis(&self.x_boundary)
                && is_singular_axis(&self.y_boundary);
            let uniqueness = if singular {
                SolutionUniqueness::DefinedUpToAdditiveConstant
            } else if self.coefficient > 0.0 {
                SolutionUniqueness::MayBeNonunique
            } else {
                SolutionUniqueness::Unique
            };
            let status = if native_code == 6 {
                NativePdeStatus::PositiveCoefficientMayNotHaveSolution
            } else {
                NativePdeStatus::Success
            };
            Ok(CartesianPdeSolution {
                values: Grid2 { values, nx, ny },
                perturbation: singular.then_some(perturbation),
                uniqueness,
                native_status: status,
            })
        }
    }

    /// A solved Cartesian FISHPACK problem.
    #[derive(Clone, Debug, PartialEq)]
    pub struct CartesianPdeSolution {
        values: Grid2,
        perturbation: Option<f32>,
        uniqueness: SolutionUniqueness,
        native_status: NativePdeStatus,
    }

    impl CartesianPdeSolution {
        /// Returns the owned solution on every grid node, including boundaries.
        #[must_use]
        pub fn values(&self) -> &Grid2 {
            &self.values
        }

        /// Consumes the result and returns the owned solution grid.
        #[must_use]
        pub fn into_values(self) -> Grid2 {
            self.values
        }

        /// Returns the native compatibility correction for a singular Poisson problem.
        ///
        /// `Some(value)` means `HWSCRT` subtracted this constant from its
        /// finite-difference RHS before solving.  A zero value means the
        /// original RHS was already compatible; a nonzero value means the grid
        /// solves the corrected system rather than the original one.
        #[must_use]
        pub fn perturbation(&self) -> Option<f32> {
            self.perturbation
        }

        /// Returns whether a nonzero RHS correction was applied.
        #[must_use]
        pub fn rhs_was_perturbed(&self) -> bool {
            self.perturbation.is_some_and(|value| value != 0.0)
        }

        /// Returns the uniqueness contract for this solve.
        #[must_use]
        pub fn uniqueness(&self) -> SolutionUniqueness {
            self.uniqueness
        }

        /// Returns the reviewed native completion status.
        #[must_use]
        pub fn native_status(&self) -> NativePdeStatus {
            self.native_status
        }
    }

    /// Whether the returned finite-difference solution is uniquely determined.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SolutionUniqueness {
        /// The reviewed boundary/coefficient contract makes the discrete solution unique.
        Unique,
        /// Any additive constant can be added to the compatible Poisson solution.
        DefinedUpToAdditiveConstant,
        /// A positive coefficient triggered `HWSCRT`'s documented warning.
        MayBeNonunique,
    }

    /// The documented `HWSCRT` completion status.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum NativePdeStatus {
        /// `HWSCRT` returned `IERROR = 0`.
        Success,
        /// `HWSCRT` returned `IERROR = 6` after attempting the solve.
        PositiveCoefficientMayNotHaveSolution,
    }

    impl NativePdeStatus {
        /// Returns the exact native `IERROR` code.
        #[must_use]
        pub fn code(self) -> i32 {
            match self {
                Self::Success => 0,
                Self::PositiveCoefficientMayNotHaveSolution => 6,
            }
        }
    }

    /// The corner at which two prescribed-value edge vectors disagreed.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CartesianCorner {
        /// `(x.lower(), y.lower())`.
        LowerLeft,
        /// `(x.upper(), y.lower())`.
        LowerRight,
        /// `(x.lower(), y.upper())`.
        UpperLeft,
        /// `(x.upper(), y.upper())`.
        UpperRight,
    }

    /// Validation or native-completion failure for the Cartesian PDE facade.
    #[derive(Clone, Debug, PartialEq)]
    pub enum PdeError {
        /// Axis endpoints were not strictly increasing.
        InvalidAxis,
        /// An axis has fewer panels than the `HWSCRT` minimum.
        GridTooSmall {
            /// Requested panel count.
            intervals: usize,
            /// Required minimum panel count.
            minimum: usize,
        },
        /// A checked dimension or workspace expression overflowed `usize` or `INTEGER`.
        DimensionOverflow,
        /// The RHS grid did not have the exact required number of values.
        InvalidRightHandSideShape {
            /// Required value count.
            expected: usize,
            /// Actual value count.
            actual: usize,
        },
        /// An input scalar, RHS sample, or boundary sample was not finite.
        NonFiniteInput {
            /// Human-readable input category.
            field: &'static str,
        },
        /// A boundary vector did not cover every node on its edge.
        InvalidBoundaryLength {
            /// The affected coordinate axis.
            axis: &'static str,
            /// Required edge-vector length.
            expected: usize,
            /// Actual edge-vector length.
            actual: usize,
        },
        /// Two prescribed Dirichlet values supplied for one corner were unequal.
        InconsistentCornerValues {
            /// The conflicting geometric corner.
            corner: CartesianCorner,
            /// Value supplied by the x-edge vector.
            x_value: f32,
            /// Value supplied by the y-edge vector.
            y_value: f32,
        },
        /// Duplicate RHS samples for one periodic endpoint pair were unequal.
        InconsistentPeriodicRightHandSide {
            /// The periodic coordinate axis.
            axis: &'static str,
        },
        /// Native workspace allocation could not be reserved.
        AllocationFailed,
        /// Native `W(1)` was not a valid requirement within the allocated bound.
        InconsistentNativeWorkspace {
            /// Native-reported `W(1)` value.
            reported: f32,
            /// Safe upper-bound allocation length.
            allocated: usize,
        },
        /// A prevalidated native call returned an unexpected `IERROR` code.
        NativeFailure {
            /// Exact `HWSCRT` error flag.
            code: i32,
        },
    }

    impl fmt::Display for PdeError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::InvalidAxis => {
                    formatter.write_str("axis endpoints must be finite and strictly increasing")
                }
                Self::GridTooSmall { intervals, minimum } => write!(
                    formatter,
                    "axis has {intervals} intervals; HWSCRT requires at least {minimum}"
                ),
                Self::DimensionOverflow => {
                    formatter.write_str("PDE dimension or workspace arithmetic overflowed")
                }
                Self::InvalidRightHandSideShape { expected, actual } => {
                    write!(formatter, "RHS has {actual} values; expected {expected}")
                }
                Self::NonFiniteInput { field } => {
                    write!(formatter, "{field} must contain only finite values")
                }
                Self::InvalidBoundaryLength {
                    axis,
                    expected,
                    actual,
                } => write!(
                    formatter,
                    "{axis}-boundary vector has length {actual}; expected {expected}"
                ),
                Self::InconsistentCornerValues { corner, .. } => {
                    write!(formatter, "inconsistent prescribed values at {corner:?}")
                }
                Self::InconsistentPeriodicRightHandSide { axis } => write!(
                    formatter,
                    "periodic {axis}-edge RHS samples must match at duplicate nodes"
                ),
                Self::AllocationFailed => formatter.write_str("PDE allocation failed"),
                Self::InconsistentNativeWorkspace {
                    reported,
                    allocated,
                } => write!(
                    formatter,
                    "HWSCRT reported invalid workspace {reported} for allocation {allocated}"
                ),
                Self::NativeFailure { code } => write!(
                    formatter,
                    "HWSCRT returned unexpected native error code {code}"
                ),
            }
        }
    }

    impl std::error::Error for PdeError {}

    fn checked_area(nx: usize, ny: usize) -> Result<usize, PdeError> {
        nx.checked_mul(ny).ok_or(PdeError::DimensionOverflow)
    }

    fn allocate_zeroed(length: usize) -> Result<Vec<f32>, PdeError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(length)
            .map_err(|_| PdeError::AllocationFailed)?;
        values.resize(length, 0.0);
        Ok(values)
    }

    fn workspace_len(nx: usize, ny: usize) -> Result<usize, PdeError> {
        let log2_ny = (usize::BITS - 1 - ny.leading_zeros()) as usize;
        ny.checked_mul(4)
            .and_then(|first| {
                log2_ny
                    .checked_add(13)
                    .and_then(|factor| factor.checked_mul(nx))
                    .and_then(|second| first.checked_add(second))
            })
            .ok_or(PdeError::DimensionOverflow)
    }

    fn is_singular_axis(boundary: &AxisBoundary) -> bool {
        matches!(
            boundary,
            AxisBoundary::Periodic | AxisBoundary::Neumann { .. }
        )
    }

    fn validate_periodic_rhs(
        rhs: &Grid2,
        x_boundary: &AxisBoundary,
        y_boundary: &AxisBoundary,
    ) -> Result<(), PdeError> {
        if matches!(x_boundary, AxisBoundary::Periodic)
            && (0..rhs.ny).any(|y| rhs[(0, y)] != rhs[(rhs.nx - 1, y)])
        {
            return Err(PdeError::InconsistentPeriodicRightHandSide { axis: "x" });
        }
        if matches!(y_boundary, AxisBoundary::Periodic)
            && (0..rhs.nx).any(|x| rhs[(x, 0)] != rhs[(x, rhs.ny - 1)])
        {
            return Err(PdeError::InconsistentPeriodicRightHandSide { axis: "y" });
        }
        Ok(())
    }

    fn validate_corners(
        nx: usize,
        ny: usize,
        x_boundary: &AxisBoundary,
        y_boundary: &AxisBoundary,
    ) -> Result<(), PdeError> {
        check_corner(
            CartesianCorner::LowerLeft,
            x_boundary.lower_value().map(|values| values[0]),
            y_boundary.lower_value().map(|values| values[0]),
        )?;
        check_corner(
            CartesianCorner::LowerRight,
            x_boundary.upper_value().map(|values| values[0]),
            y_boundary.lower_value().map(|values| values[nx - 1]),
        )?;
        check_corner(
            CartesianCorner::UpperLeft,
            x_boundary.lower_value().map(|values| values[ny - 1]),
            y_boundary.upper_value().map(|values| values[0]),
        )?;
        check_corner(
            CartesianCorner::UpperRight,
            x_boundary.upper_value().map(|values| values[ny - 1]),
            y_boundary.upper_value().map(|values| values[nx - 1]),
        )
    }

    fn check_corner(
        corner: CartesianCorner,
        x_value: Option<f32>,
        y_value: Option<f32>,
    ) -> Result<(), PdeError> {
        if let (Some(x_value), Some(y_value)) = (x_value, y_value) {
            if x_value != y_value {
                return Err(PdeError::InconsistentCornerValues {
                    corner,
                    x_value,
                    y_value,
                });
            }
        }
        Ok(())
    }

    fn apply_dirichlet_edges(
        nx: usize,
        ny: usize,
        values: &mut [f32],
        x_boundary: &AxisBoundary,
        y_boundary: &AxisBoundary,
    ) {
        if let Some(lower) = x_boundary.lower_value() {
            for y in 0..ny {
                values[y * nx] = lower[y];
            }
        }
        if let Some(upper) = x_boundary.upper_value() {
            for y in 0..ny {
                values[y * nx + nx - 1] = upper[y];
            }
        }
        if let Some(lower) = y_boundary.lower_value() {
            values[..nx].copy_from_slice(lower);
        }
        if let Some(upper) = y_boundary.upper_value() {
            values[(ny - 1) * nx..].copy_from_slice(upper);
        }
    }

    #[cfg(test)]
    mod tests {
        use alloc::vec;
        use core::convert::TryFrom;

        use super::{
            AxisBoundary, CartesianCorner, CartesianHelmholtz2d, Grid2, PdeError, UniformAxis,
            allocate_zeroed, workspace_len,
        };
        use slatec_sys::FortranInteger;

        fn axes() -> (UniformAxis, UniformAxis) {
            (
                UniformAxis::new(0.0, 1.0, 4).unwrap(),
                UniformAxis::new(0.0, 1.0, 5).unwrap(),
            )
        }

        #[test]
        fn grid_is_x_fast_row_major() {
            let grid = Grid2::new(3, 2, vec![0.0, 1.0, 2.0, 10.0, 11.0, 12.0]).unwrap();
            assert_eq!(grid[(2, 0)], 2.0);
            assert_eq!(grid[(1, 1)], 11.0);
        }

        #[test]
        fn rejects_conflicting_dirichlet_corner() {
            let (x, y) = axes();
            let rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                rhs,
                AxisBoundary::Dirichlet {
                    lower: vec![1.0; 6],
                    upper: vec![2.0; 6],
                },
                AxisBoundary::Dirichlet {
                    lower: vec![3.0; 5],
                    upper: vec![4.0; 5],
                },
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::InconsistentCornerValues {
                    corner: CartesianCorner::LowerLeft,
                    ..
                }
            ));
        }

        #[test]
        fn validates_periodic_rhs_duplicates() {
            let (x, y) = axes();
            let mut rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            rhs[(4, 2)] = 1.0;
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                rhs,
                AxisBoundary::Periodic,
                AxisBoundary::Neumann {
                    lower_derivative: vec![0.0; 5],
                    upper_derivative: vec![0.0; 5],
                },
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::InconsistentPeriodicRightHandSide { axis: "x" }
            ));
        }

        #[test]
        fn rejects_invalid_axes_and_too_few_panels() {
            assert!(matches!(
                UniformAxis::new(f32::NAN, 1.0, 4),
                Err(PdeError::NonFiniteInput { .. })
            ));
            assert!(matches!(
                UniformAxis::new(1.0, 0.0, 4),
                Err(PdeError::InvalidAxis)
            ));
            assert!(matches!(
                UniformAxis::new(1.0, 1.0, 4),
                Err(PdeError::InvalidAxis)
            ));
            assert!(matches!(
                UniformAxis::new(0.0, 1.0, 3),
                Err(PdeError::GridTooSmall { .. })
            ));
        }

        #[test]
        fn rejects_mismatched_rhs_and_boundary_shapes() {
            let (x, y) = axes();
            let mismatched_rhs = Grid2::new(1, 1, vec![0.0]).unwrap();
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                mismatched_rhs,
                AxisBoundary::Periodic,
                AxisBoundary::Periodic,
            )
            .unwrap_err();
            assert!(matches!(error, PdeError::InvalidRightHandSideShape { .. }));

            let rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                rhs,
                AxisBoundary::Dirichlet {
                    lower: vec![0.0; 5],
                    upper: vec![0.0; 6],
                },
                AxisBoundary::Periodic,
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::InvalidBoundaryLength { axis: "x", .. }
            ));
        }

        #[test]
        fn rejects_nonfinite_coefficient_rhs_and_boundary_data() {
            let (x, y) = axes();
            let rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                f32::INFINITY,
                rhs,
                AxisBoundary::Periodic,
                AxisBoundary::Periodic,
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::NonFiniteInput {
                    field: "Helmholtz coefficient"
                }
            ));

            let mut rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            rhs[(2, 3)] = f32::NAN;
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                rhs,
                AxisBoundary::Periodic,
                AxisBoundary::Periodic,
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::NonFiniteInput {
                    field: "right-hand side"
                }
            ));

            let rhs = Grid2::zeros(x.nodes().unwrap(), y.nodes().unwrap()).unwrap();
            let error = CartesianHelmholtz2d::new(
                x,
                y,
                0.0,
                rhs,
                AxisBoundary::Neumann {
                    lower_derivative: vec![0.0; 6],
                    upper_derivative: vec![f32::NAN; 6],
                },
                AxisBoundary::Periodic,
            )
            .unwrap_err();
            assert!(matches!(
                error,
                PdeError::NonFiniteInput {
                    field: "boundary data"
                }
            ));
        }

        #[test]
        fn checked_dimension_paths_reject_native_and_allocation_overflow() {
            assert!(FortranInteger::try_from(i32::MAX as usize + 1).is_err());
            assert!(matches!(
                Grid2::zeros(usize::MAX, 2),
                Err(PdeError::DimensionOverflow)
            ));
            assert!(matches!(
                workspace_len(usize::MAX, 5),
                Err(PdeError::DimensionOverflow)
            ));
            assert!(matches!(
                allocate_zeroed(usize::MAX),
                Err(PdeError::AllocationFailed)
            ));
        }
    }
}

#[cfg(feature = "fishpack-cartesian-2d")]
pub use fishpack_cartesian_2d::{
    AxisBoundary, CartesianCorner, CartesianHelmholtz2d, CartesianPdeSolution, Grid2,
    NativePdeStatus, PdeError, SolutionUniqueness, UniformAxis,
};
