#![no_std]

//! Raw declarations generated from the selected, compiler-observed SLATEC corpus.
//!
//! The crate deliberately performs no download, native compilation, linking,
//! or safe conversion in `build.rs`. Native provider selection and automatic
//! hosted linking belong to `slatec-src`; an external-backend consumer can use
//! these declarations without any hidden native build directive.
//!
//! # Stability policy
//!
//! Reviewed declarations are promoted to canonical mathematical modules and
//! retain older paths as compatibility re-exports. The ABI-shaped [`generated`]
//! module is transitional implementation-generated access: its paths are not
//! stable merely because an item is available there. The source-hash-guarded
//! raw API inventory defines reviewed status, documentation, and validation
//! coverage. An evidence-proven ABI correction may change an unsafe signature,
//! because an incorrect FFI declaration is a safety bug.

/// GNU Fortran default `INTEGER` after the supported profile probe.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranInteger = i32;

/// GNU Fortran default `LOGICAL` after the supported profile probe. This is
/// intentionally not Rust `bool`.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranLogical = i32;

/// GNU Fortran hidden CHARACTER length for the supported 64-bit profile.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranCharacterLength = usize;

/// GNU Fortran-compatible default `COMPLEX` storage used by the validated raw
/// ABI profile. This is a layout record, not a safe numerical type.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

/// GNU Fortran-compatible default `DOUBLE COMPLEX` storage used by the
/// validated raw ABI profile. This is a layout record, not a safe numerical
/// type.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

pub mod generated;

/// Canonical reviewed raw BLAS namespace.
///
/// Enable `blas-level1`, `blas-level2`, `blas-level3`, or the aggregate
/// `blas` feature.  The legacy [`families`] BLAS modules remain compatibility
/// re-exports; [`generated`] paths remain ABI-shaped transitional access.
#[cfg(any(
    feature = "blas-level1",
    feature = "blas-level2",
    feature = "blas-level3",
    feature = "raw-family-blas-level1",
    feature = "raw-family-blas-level2",
    feature = "raw-family-blas-level3"
))]
pub mod blas;

/// Canonical reviewed raw scalar special-function namespaces.
///
/// Enable one of `special-elementary`, `special-gamma`, `special-beta`,
/// `special-error`, or `special-airy`, or the aggregate `special`. The
/// remaining public special subfamilies continue to be available through their
/// compatibility modules until they receive the same source-hash and
/// documentation review.
#[cfg(any(
    feature = "special-elementary",
    feature = "special-gamma",
    feature = "special-beta",
    feature = "special-error",
    feature = "special-airy",
    feature = "batch-a-special",
    feature = "raw-family-special-elementary",
    feature = "raw-family-special-gamma",
    feature = "raw-family-special-beta",
    feature = "raw-family-special-error",
    feature = "raw-family-special-airy",
    feature = "raw-family-batch-a-special"
))]
pub mod special;

/// Canonical generated linear-algebra declarations.
#[cfg(any(
    feature = "batch-a-linear-algebra",
    feature = "batch-b-linear-algebra",
    feature = "raw-family-batch-a-linear-algebra",
    feature = "raw-family-batch-b-linear-algebra"
))]
pub mod linear_algebra;

/// Canonical Batch A eigenvalue declarations.
#[cfg(feature = "batch-a-eigen")]
#[path = "batch_a/eigen.rs"]
pub mod eigen;

/// Canonical Batch A approximation declarations.
#[cfg(feature = "batch-a-approximation")]
#[path = "batch_a/approximation.rs"]
pub mod approximation;

/// Canonical Batch A interpolation declarations.
#[cfg(feature = "batch-a-interpolation")]
#[path = "batch_a/interpolation.rs"]
pub mod interpolation;

/// Canonical Batch A statistics declarations.
#[cfg(feature = "batch-a-statistics")]
#[path = "batch_a/statistics.rs"]
pub mod statistics;

/// Generated raw declarations grouped by safe public family rather than ABI
/// shape. These modules are the preferred dependency of narrow safe features.
#[cfg(any(
    feature = "raw-family-blas-level1",
    feature = "raw-family-blas-level2",
    feature = "raw-family-blas-level3",
    feature = "raw-family-special-elementary",
    feature = "raw-family-special-gamma",
    feature = "raw-family-special-beta",
    feature = "raw-family-special-error",
    feature = "raw-family-special-airy",
    feature = "raw-family-special-bessel",
    feature = "raw-family-special-integrals",
    feature = "raw-family-special-polynomials",
    feature = "raw-family-fftpack-real",
    feature = "raw-family-pchip",
    feature = "raw-family-bspline",
    feature = "raw-family-piecewise-polynomial"
))]
pub mod families;

/// Hand-reviewed scalar declarations for the expanded real special-function
/// family.
#[cfg(feature = "raw-family-special-scalar-expanded")]
pub mod special_scalar_expanded;

/// Hand-reviewed real FFTPACK declarations for the plan-based safe API.
#[cfg(any(
    feature = "raw-family-fftpack-real",
    feature = "raw-family-batch-a-fftpack"
))]
pub mod fftpack;

/// Hand-reviewed standard real-array declarations for the selected complex
/// FFTPACK plan family.
#[cfg(feature = "raw-family-fftpack-complex")]
pub mod fftpack_complex;

/// Hand-reviewed declaration for the focused Cartesian FISHPACK driver.
#[cfg(feature = "raw-family-fishpack-cartesian-2d")]
pub mod fishpack_cartesian_2d;

/// Hand-reviewed declaration for the focused structured FISHPACK `POIS3D`
/// driver.
#[cfg(feature = "raw-family-fishpack-pois3d")]
pub mod fishpack_pois3d;

/// Canonical raw PDE namespace.
///
/// Only reviewed FISHPACK drivers are re-exported here. The historical focused
/// modules remain compatibility paths and no duplicate FFI declarations are
/// introduced.
#[cfg(any(
    feature = "raw-family-fishpack-cartesian-2d",
    feature = "raw-family-fishpack-pois3d",
    feature = "raw-family-batch-a-fishpack"
))]
pub mod pde {
    /// Canonical reviewed FISHPACK driver namespace.
    pub mod fishpack {
        #[cfg(feature = "raw-family-fishpack-cartesian-2d")]
        pub use crate::fishpack_cartesian_2d::hwscrt;
        #[cfg(feature = "raw-family-fishpack-pois3d")]
        pub use crate::fishpack_pois3d::pois3d;
        #[cfg(feature = "raw-family-batch-a-fishpack")]
        #[path = "../../batch_a/pde_fishpack.rs"]
        mod batch_a;
        #[cfg(feature = "raw-family-batch-a-fishpack")]
        pub use batch_a::numerical;
    }
}

/// Hand-reviewed LINPACK general-band factorization and solve declarations.
#[cfg(any(
    feature = "raw-family-banded-linear-systems",
    feature = "raw-family-batch-a-linear-algebra"
))]
pub mod banded;

/// Hand-reviewed PCHIP and piecewise-cubic Hermite declarations.
#[cfg(any(
    feature = "raw-family-pchip",
    feature = "raw-family-batch-a-interpolation"
))]
pub mod pchip;

/// Hand-reviewed declarations for the restricted safe B-spline facade.
#[cfg(any(
    feature = "raw-family-bspline",
    feature = "raw-family-batch-a-interpolation"
))]
pub mod bspline;

/// Hand-reviewed declarations for PP-form evaluation, integration, and
/// B-spline conversion.
#[cfg(any(
    feature = "raw-family-piecewise-polynomial",
    feature = "raw-family-batch-a-interpolation"
))]
pub mod piecewise_polynomial;

/// Hand-reviewed callback declarations for the focused safe QUADPACK surface.
///
/// These declarations remain separate from the broadly gated callback batch:
/// only the eight routines whose callback and workspace contracts are tested
/// by `slatec::quadrature` are available here.
#[cfg(any(
    feature = "raw-ffi-quadrature",
    feature = "raw-family-quadrature-basic",
    feature = "raw-family-quadrature-breakpoints",
    feature = "raw-family-quadrature-weighted",
    feature = "raw-family-quadrature-oscillatory",
    feature = "raw-family-quadrature-fourier",
    feature = "raw-family-quadrature-nonadaptive",
    feature = "raw-family-batch-a-quadrature",
    feature = "raw-family-batch-b-quadrature"
))]
pub mod quadrature;

/// Hand-reviewed scalar callback declarations for the focused FZERO family.
///
/// This narrow module remains separate from the general callback batch.
#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
pub mod roots;

/// Hand-reviewed nonlinear callback and Jacobian-check declarations.
///
/// This narrow module exposes `SNSQE`, `DNSQE`, `SNSQ`, `DNSQ`, `CHKDER`, and
/// `DCKDER` for the validated GNU MinGW ABI profile. The declarations remain
/// unsafe because the caller owns callback lifetime, vector and matrix
/// storage, workspace, and runtime serialization.
#[cfg(any(
    feature = "raw-ffi-nonlinear",
    feature = "raw-family-nonlinear-easy",
    feature = "raw-family-nonlinear-expert",
    feature = "raw-family-nonlinear-jacobian-check",
    feature = "raw-family-batch-a-nonlinear"
))]
pub mod nonlinear;

/// Hand-reviewed `SDRIV3`/`DDRIV3` declarations for the safe RHS-only ODE
/// session feature.
#[cfg(any(
    feature = "raw-ffi-ode",
    feature = "raw-family-ode-sdrive-expert",
    feature = "raw-family-batch-a-ode",
    feature = "raw-family-batch-b-ode"
))]
pub mod ode;

/// Hand-reviewed `SDASSL`/`DDASSL` declarations for the restricted safe
/// residual-only DAE session feature.
#[cfg(feature = "raw-family-dassl")]
pub mod dassl;

/// Hand-reviewed `SPLP`/`DSPLP` declarations for the in-memory-only safe
/// linear-programming feature.
#[cfg(feature = "raw-family-optimization-linear-programming-in-memory")]
pub mod linear_programming;

/// Hand-reviewed declarations for nonlinear least-squares easy and expert
/// drivers.
///
/// This narrow module contains `SNLS1E`, `DNLS1E`, `SNLS1`, `DNLS1`, `SCOV`,
/// and `DCOV`.
/// It remains separate from the broad callback batch: safe callers must still
/// uphold callback lifetime, rectangular Jacobian, workspace, and process-wide
/// runtime invariants.
#[cfg(any(
    feature = "raw-ffi-least-squares",
    feature = "raw-family-least-squares-nonlinear-easy",
    feature = "raw-family-least-squares-nonlinear-expert",
    feature = "raw-family-least-squares-covariance"
))]
pub mod least_squares;

/// Hand-reviewed declarations for constrained linear least squares.
///
/// This narrow module exposes `WNNLS`/`DWNNLS`, `SBOLS`/`DBOLS`,
/// `LSEI`/`DLSEI`, and `SBOCLS`/`DBOCLS`. The
/// routines mutate their augmented matrix and work arrays, so the declarations
/// remain unsafe.
#[cfg(any(
    feature = "raw-ffi-linear-least-squares",
    feature = "raw-family-least-squares-linear-nonnegative",
    feature = "raw-family-least-squares-linear-bounded",
    feature = "raw-family-least-squares-linear-constrained",
    feature = "raw-family-least-squares-linear-bounded-constrained",
    feature = "raw-family-batch-a-approximation"
))]
pub mod linear_least_squares;

/// Minimal internal-facing legacy-error controls required by reviewed native
/// families with recoverable status messages.
///
/// `DNLS1`/`SNLS1` and `SDRIV3`/`DDRIV3` report selected meaningful completion
/// states through `XERMSG`. Safe wrappers use these declarations privately to
/// make documented recoverable messages return through their native status
/// values while preserving and restoring the process-global control flag.
#[cfg(any(
    feature = "raw-family-least-squares-nonlinear-easy",
    feature = "raw-family-least-squares-nonlinear-expert",
    feature = "raw-family-least-squares-covariance",
    feature = "raw-family-least-squares-linear-bounded",
    feature = "raw-family-least-squares-linear-constrained",
    feature = "raw-family-least-squares-linear-bounded-constrained",
    feature = "raw-family-ode-sdrive-expert",
    feature = "raw-family-dassl",
    feature = "raw-family-optimization-linear-programming-in-memory",
    feature = "raw-family-pchip",
    feature = "raw-family-bspline",
    feature = "raw-family-piecewise-polynomial",
    feature = "raw-family-special-scalar-expanded"
))]
pub mod legacy_error;
