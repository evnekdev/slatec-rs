#![no_std]

//! Raw declarations generated from the selected, compiler-observed SLATEC corpus.
//!
//! The crate deliberately performs no download, native compilation, linking,
//! or safe conversion in `build.rs`. Native provider selection and automatic
//! hosted linking belong to `slatec-src`; an external-backend consumer can use
//! these declarations without any hidden native build directive.

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
    feature = "raw-family-bspline"
))]
pub mod families;

/// Hand-reviewed scalar declarations for the expanded real special-function
/// family.
#[cfg(feature = "raw-family-special-scalar-expanded")]
pub mod special_scalar_expanded;

/// Hand-reviewed real FFTPACK declarations for the plan-based safe API.
#[cfg(feature = "raw-family-fftpack-real")]
pub mod fftpack;

/// Hand-reviewed PCHIP and piecewise-cubic Hermite declarations.
#[cfg(feature = "raw-family-pchip")]
pub mod pchip;

/// Hand-reviewed declarations for the restricted safe B-spline facade.
#[cfg(feature = "raw-family-bspline")]
pub mod bspline;

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
    feature = "raw-family-quadrature-nonadaptive"
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
    feature = "raw-family-nonlinear-jacobian-check"
))]
pub mod nonlinear;

/// Hand-reviewed `SDRIV3`/`DDRIV3` declarations for the safe RHS-only ODE
/// session feature.
#[cfg(any(feature = "raw-ffi-ode", feature = "raw-family-ode-sdrive-expert"))]
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
    feature = "raw-family-least-squares-linear-bounded-constrained"
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
    feature = "raw-family-special-scalar-expanded"
))]
pub mod legacy_error;
