//! Permanent safe-API roadmap.
//!
//! This module is documentation only: roadmap namespaces that have no
//! callable items are placeholders, not promises of future signatures or
//! release dates. Source and native-state audits may change a status.
//!
//! ## Status legend
//!
//! - **Implemented**: the reviewed initial safe scope exists.
//! - **Partial**: useful public functionality exists, but material reviewed
//!   work remains.
//! - **Planned**: a future family is intended, subject to audit.
//! - **Deferred**: a known safety, ABI, state, storage, or branch blocker
//!   intentionally postpones the family.
//! - **Unavailable**: the expected selected-snapshot capability is absent.
//!
//! ## Path stability
//!
//! The high-level domains are frozen: `roadmap`, [`crate::linear_algebra`],
//! [`crate::special`], [`crate::integration`], [`crate::equations`],
//! [`crate::least_squares`], [`crate::differential_equations`],
//! [`crate::optimization`], [`crate::transforms`], and
//! [`crate::interpolation`]. Their leaves are reserved unless explicitly
//! marked provisional in the generated roadmap metadata.
//!
//! ## Current surface
//!
//! The checked-in metadata reports 226 safe functions: 58 core-only, 2
//! alloc-only, and 166 hosted `std` functions. Structural modules do not add
//! safe functions, native routines, or capabilities.
//!
//! ## Domain tree
//!
//! ```text
//! linear_algebra::{blas::{level1, level2, level3}, dense, banded, packed, eigen,
//!   sparse::{iterative, direct, operators}}
//! special::{elementary, gamma, beta, error, airy, bessel,
//!   integrals::{exponential, logarithmic, spence, trigonometric},
//!   elliptic::carlson, probability, polynomials}
//! integration::{quadrature::{basic, breakpoints, weighted, oscillatory, fourier,
//!   nonadaptive}, integral_equations}
//! equations::{roots::{scalar, polynomial}, nonlinear::{easy, expert, jacobian_check}}
//! least_squares::{nonlinear::{easy, expert}, covariance,
//!   linear::{nonnegative, bounded, constrained, bounded_constrained}}
//! differential_equations::{ode::{sdrive, runge_kutta, adams},
//!   dae::dassl::{residual, user_jacobian, banded}, boundary_value, pde}
//! optimization::{linear_programming, unconstrained, constrained, nonlinear}
//! transforms::fft::{real, complex, multidimensional}
//! interpolation::{pchip, bspline, piecewise_polynomial, divided_differences,
//!   chebyshev, approximation}
//! ```
//!
//! ## Current order of work
//!
//! Future milestones should first complete focused native inventories and
//! state/ABI audits, then add an explicit feature, source closure, safe API,
//! tests, and metadata. The generated files
//! `public-module-roadmap.json` and `public-module-tree.md` provide the
//! detailed feature, blocker, and next-milestone records.
