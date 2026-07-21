#![no_std]
#![allow(unused_doc_comments)]

//! Raw declarations generated from the selected, compiler-observed SLATEC corpus.
//!
//! The crate deliberately performs no download, native compilation, linking,
//! or safe conversion in `build.rs`. Native provider selection and automatic
//! hosted linking belong to `slatec-src`; an external-backend consumer can use
//! these declarations without any hidden native build directive.
//!
//! # Stability policy
//!
//! Reviewed declarations are promoted directly to canonical mathematical
//! modules. ABI-shaped declaration modules are private implementation details,
//! not public API paths. The source-hash-guarded raw API inventory defines
//! reviewed status, documentation, and validation coverage. An
//! evidence-proven ABI correction may change an unsafe signature, because an
//! incorrect FFI declaration is a safety bug. Generated implementation access
//! is transitional and unstable unless a declaration is re-exported from its
//! canonical mathematical module.
//!
//! Semantic documentation completion is evaluated on the rendered canonical
//! item, not on a generated fragment. Its source-first direction evidence,
//! argument-contamination checks, and applicable callback/status/workspace
//! contracts are recorded under `generated/slatec-routines` in the workspace.

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

#[allow(unused_macros)]
macro_rules! public_binding_module {
    ($binding:ident, $public:ident, $path:literal, $description:literal) => {
        #[path = $path]
        mod $binding;

        #[doc = $description]
        pub mod $public {
            #[doc(inline)]
            pub use crate::$binding::*;
        }
    };
}

#[path = "generated/mod.rs"]
mod generated_ffi;

/// Private ABI-shaped declaration re-exports used while generating and
/// validating canonical mathematical modules.
#[path = "generated_compat.rs"]
mod generated;

/// Canonical reviewed raw BLAS namespace.
///
/// Enable `blas-level1`, `blas-level2`, `blas-level3`, or the aggregate
/// `blas` feature.
#[cfg(any(
    feature = "blas-level1",
    feature = "blas-level2",
    feature = "blas-level3",
    feature = "raw-family-blas-level1",
    feature = "raw-family-blas-level2",
    feature = "raw-family-blas-level3",
    feature = "raw-family-blas-complex"
))]
pub mod blas;

/// Canonical reviewed raw scalar special-function namespaces.
///
/// Enable one of `special-elementary`, `special-gamma`, `special-beta`,
/// `special-error`, or `special-airy`, or the aggregate `special`. The
/// remaining public special subfamilies use their canonical mathematical paths.
#[cfg(any(
    feature = "special-elementary",
    feature = "special-gamma",
    feature = "special-beta",
    feature = "special-error",
    feature = "special-airy",
    feature = "special-real",
    feature = "raw-family-special-elementary",
    feature = "raw-family-special-gamma",
    feature = "raw-family-special-beta",
    feature = "raw-family-special-error",
    feature = "raw-family-special-airy",
    feature = "raw-family-special-real",
    feature = "raw-family-special-complex"
))]
public_binding_module!(
    special_bindings,
    special,
    "special.rs",
    "Canonical raw special-function bindings organized by mathematical subfamily."
);

/// Canonical generated linear-algebra declarations.
#[cfg(any(
    feature = "linear-algebra-real",
    feature = "linear-algebra-iterative",
    feature = "linear-algebra-eigen",
    feature = "raw-family-linear-algebra-real",
    feature = "raw-family-linear-algebra-eigen",
    feature = "raw-family-linear-algebra-iterative",
    feature = "raw-family-linear-algebra-complex"
))]
public_binding_module!(
    linear_algebra_bindings,
    linear_algebra,
    "linear_algebra.rs",
    "Canonical raw linear-algebra bindings organized by storage and eigenproblem class."
);

#[cfg(any(
    feature = "raw-family-blas-complex",
    feature = "raw-family-linear-algebra-complex",
    feature = "raw-family-special-complex",
    feature = "raw-family-nonlinear-complex",
    feature = "raw-family-fishpack-complex"
))]
#[path = "batch_c/mod.rs"]
mod abi_bindings;

#[cfg(feature = "approximation-core")]
public_binding_module!(
    approximation_bindings,
    approximation,
    "batch_a/approximation.rs",
    "Canonical raw approximation bindings."
);

#[cfg(feature = "interpolation-general")]
public_binding_module!(
    interpolation_bindings,
    interpolation,
    "batch_a/interpolation.rs",
    "Canonical raw interpolation bindings."
);

#[cfg(feature = "statistics-core")]
public_binding_module!(
    statistics_bindings,
    statistics,
    "batch_a/statistics.rs",
    "Canonical raw probability and statistics bindings."
);

/// Private raw declaration groupings used by canonical family modules.
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
#[path = "families.rs"]
mod families;

/// Private declaration owner for expanded scalar special functions.
#[cfg(feature = "raw-family-special-scalar-expanded")]
#[path = "special_scalar_expanded.rs"]
mod special_scalar_expanded;

/// Hand-reviewed real FFTPACK declarations for the plan-based safe API.
#[cfg(any(
    feature = "raw-family-fftpack-real",
    feature = "raw-family-fftpack-extended-real"
))]
public_binding_module!(
    fftpack_bindings,
    fftpack,
    "fftpack.rs",
    "Canonical raw FFTPACK bindings."
);

/// Private declaration owner for complex FFTPACK routines; canonical paths are
/// nested under [`fftpack`].
#[cfg(feature = "raw-family-fftpack-complex")]
#[path = "fftpack_complex.rs"]
mod fftpack_complex;

/// Private declaration owner for the focused Cartesian FISHPACK driver.
#[cfg(feature = "raw-family-fishpack-cartesian-2d")]
#[path = "fishpack_cartesian_2d.rs"]
mod fishpack_cartesian_2d;

/// Private declaration owner for the focused structured FISHPACK `POIS3D`
/// driver.
#[cfg(feature = "raw-family-fishpack-pois3d")]
#[path = "fishpack_pois3d.rs"]
mod fishpack_pois3d;

/// Canonical raw PDE namespace.
///
/// Only reviewed FISHPACK drivers are re-exported here from their private
/// declaration owners.
#[cfg(any(
    feature = "raw-family-fishpack-cartesian-2d",
    feature = "raw-family-fishpack-pois3d",
    feature = "raw-family-fishpack-general",
    feature = "raw-family-fishpack-complex"
))]
pub mod pde {
    /// Canonical reviewed FISHPACK driver namespace.
    pub mod fishpack {
        #[cfg(feature = "raw-family-fishpack-cartesian-2d")]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwscrt.md"))]
        pub use crate::fishpack_cartesian_2d::hwscrt;
        #[cfg(feature = "raw-family-fishpack-pois3d")]
        #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pois3d.md"))]
        pub use crate::fishpack_pois3d::pois3d;
        #[cfg(feature = "raw-family-fishpack-general")]
        #[path = "../../batch_a/pde_fishpack.rs"]
        mod canonical_bindings;
        #[cfg(feature = "raw-family-fishpack-general")]
        pub use canonical_bindings::*;
        #[cfg(feature = "raw-family-fishpack-complex")]
        pub mod complex {
            pub use crate::abi_bindings::fishpack::*;
        }
    }
}

/// Private declaration owner for general-band factorization and solve routines.
#[cfg(any(
    feature = "raw-family-banded-linear-systems",
    feature = "raw-family-linear-algebra-real"
))]
#[path = "banded.rs"]
mod banded;

/// Private declaration owner for PCHIP routines.
#[cfg(any(
    feature = "raw-family-pchip",
    feature = "raw-family-interpolation-general"
))]
#[path = "pchip.rs"]
mod pchip;

/// Private declaration owner for B-spline routines.
#[cfg(any(
    feature = "raw-family-bspline",
    feature = "raw-family-interpolation-general"
))]
#[path = "bspline.rs"]
mod bspline;

/// Private declaration owner for PP-form evaluation, integration, and B-spline
/// conversion.
#[cfg(any(
    feature = "raw-family-piecewise-polynomial",
    feature = "raw-family-interpolation-general"
))]
#[path = "piecewise_polynomial.rs"]
/// Canonical raw PP-form evaluation, integration, and conversion declarations.
pub mod piecewise_polynomial;

/// Hand-reviewed callback declarations for the focused safe QUADPACK surface.
///
/// These declarations remain separate from the ABI-shaped callback layer:
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
    feature = "raw-family-quadrature-direct",
    feature = "raw-family-quadrature-callbacks"
))]
public_binding_module!(
    quadrature_bindings,
    quadrature,
    "quadrature.rs",
    "Canonical raw numerical quadrature bindings."
);

/// Hand-reviewed scalar callback declarations for the focused FZERO family.
///
/// This narrow module remains separate from the general callback declaration layer.
#[cfg(any(
    feature = "raw-ffi-roots",
    feature = "raw-family-roots-scalar",
    feature = "raw-family-nonlinear-complex"
))]
public_binding_module!(
    roots_bindings,
    roots,
    "roots.rs",
    "Canonical raw scalar root-finding bindings."
);

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
    feature = "raw-family-nonlinear-jacobian",
    feature = "raw-family-nonlinear-systems",
    feature = "raw-family-nonlinear-complex"
))]
public_binding_module!(
    nonlinear_bindings,
    nonlinear,
    "nonlinear.rs",
    "Canonical raw nonlinear-system bindings."
);

/// Hand-reviewed `SDRIV3`/`DDRIV3` declarations for the safe RHS-only ODE
/// session feature.
#[cfg(any(
    feature = "raw-ffi-ode",
    feature = "raw-family-ode-sdrive-expert",
    feature = "raw-family-ode-integration",
    feature = "raw-family-ode-callbacks"
))]
public_binding_module!(
    ode_bindings,
    ode,
    "ode.rs",
    "Canonical raw ordinary-differential-equation bindings."
);

/// Hand-reviewed `SDASSL`/`DDASSL` declarations for the restricted safe
/// residual-only DAE session feature.
#[cfg(feature = "raw-family-dassl")]
public_binding_module!(
    dassl_bindings,
    dassl,
    "dassl.rs",
    "Canonical raw differential-algebraic-equation bindings."
);

/// Hand-reviewed `SPLP`/`DSPLP` declarations for the in-memory-only safe
/// linear-programming feature.
#[cfg(feature = "raw-family-optimization-linear-programming-in-memory")]
public_binding_module!(
    linear_programming_bindings,
    linear_programming,
    "linear_programming.rs",
    "Canonical raw linear-programming bindings."
);

/// Hand-reviewed declarations for nonlinear least-squares easy and expert
/// drivers.
///
/// This narrow module contains `SNLS1E`, `DNLS1E`, `SNLS1`, `DNLS1`, `SCOV`,
/// and `DCOV`.
/// It remains separate from the broad callback declaration layer: safe callers must still
/// uphold callback lifetime, rectangular Jacobian, workspace, and process-wide
/// runtime invariants.
#[cfg(any(
    feature = "raw-ffi-least-squares",
    feature = "raw-family-least-squares-nonlinear-easy",
    feature = "raw-family-least-squares-nonlinear-expert",
    feature = "raw-family-least-squares-covariance"
))]
public_binding_module!(
    least_squares_bindings,
    least_squares,
    "least_squares.rs",
    "Canonical raw nonlinear least-squares bindings."
);

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
    feature = "raw-family-approximation-core"
))]
public_binding_module!(
    linear_least_squares_bindings,
    linear_least_squares,
    "linear_least_squares.rs",
    "Canonical raw constrained linear least-squares bindings."
);

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
public_binding_module!(
    legacy_error_bindings,
    legacy_error,
    "legacy_error.rs",
    "Low-level legacy error-control bindings used by reviewed numerical families."
);
