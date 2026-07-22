# slatec

Safe, family-featured Rust APIs over validated original SLATEC Fortran routines.

Use this crate when an owned, checked Rust interface exists for the required
family. The safe surface is intentionally selective and is much smaller than
the 812-routine raw API in `slatec-sys`. A native family requires exactly one
explicit backend (`source-build`, `system`, or `external-backend`); builds never
download source, and the unresolved-rights `prebuilt` mode remains unavailable.
GNU MinGW on `x86_64-pc-windows-gnu` is the strongest validated native profile.

The hosted `least-squares-nonlinear-easy` family provides residual-only,
finite-difference nonlinear least-squares fitting through the original
`SNLS1E` and `DNLS1E` easy drivers. The separate
`least-squares-nonlinear-expert` feature provides reviewed `SNLS1` and
`DNLS1` finite-difference and dense analytic-Jacobian modes. Both require
`std`, an explicit native backend, and the validated GNU MinGW profile;
the independent `least-squares-covariance` feature provides reviewed `SCOV`
and `DCOV` covariance estimation under the same hosted-profile requirement.
The distinct `least-squares-linear-nonnegative` feature wraps `WNNLS` and
`DWNNLS` for equality-constrained linear least squares with selected
nonnegative variables; it also requires `std`, an explicit backend, and the
validated GNU MinGW profile.
The separate `least-squares-linear-bounded` feature wraps `SBOLS` and `DBOLS`
for dense linear least squares with typed closed per-variable bounds under the
same hosted-profile requirement; it does not provide general equalities,
inequalities, or linear programming.
`least-squares-linear-bounded-constrained` separately wraps `SBOCLS` and
`DBOCLS`: it bounds original variables and the constraint expressions `C x`,
and therefore is neither a generic combination of the other drivers nor a
linear-programming interface.

The hosted `ode-sdrive-expert` feature provides owned continuation sessions
over reviewed `SDRIV1`/`DDRIV1`, `SDRIV2`/`DDRIV2`, `CDRIV1`/`CDRIV2`, and
the existing expert `SDRIV3`/`DDRIV3` drivers. `Driv2Session` and
`ComplexDriv2Session` expose zero-based indexed root events; all callback
sessions are panic-contained, process-serialized, and preserve same-direction
continuation workspace. Jacobians, mass matrices, DAEs, interpolation, and
`CDRIV3` remain deliberately deferred.

The `quadrature-piecewise-polynomial` feature adds `DPFQAD` multiplicative
integration over the checked `PiecewisePolynomial<f64>` representation. The
`nonlinear-systems` feature adds scalar-equation `SOS`/`DSOS` solvers with
typed termination reports. Both reuse the existing callback runtime and
require `std`, an explicit native backend, and the validated GNU MinGW
profile.

The hosted `dassl` feature separately provides owned real residual-only DAE
sessions over `SDASSL`/`DDASSL` for index-1 `G(t, y, y') = 0` problems. It
requires caller-supplied sufficiently consistent `(y, y')`, uses internal
dense finite differences, contains residual errors and panics, and keeps all
native calls process-serialized. User Jacobians, banded/sparse storage,
events, and automatic initial-condition calculation remain deferred; see the
[DASSL guide](../../docs/api/safe-dassl.md).

The hosted `optimization-linear-programming-in-memory` feature provides typed
owned sparse-column LPs over original `SPLP`/`DSPLP`. It rejects problems that
would require paging and never exposes Fortran-unit, save/restore, or printing
controls. Optimal results provide checked typed basis records, source-audited
row multipliers and reduced costs, and independently recomputed KKT
diagnostics; reviewed controls are limited to iteration count, feasibility
tolerances, and pricing.

The hosted `approximation-polynomial-fitting` feature provides immutable
`f32`/`f64` weighted polynomial least-squares models over `POLFIT`/`DPOLFT`.
It supports source-defined degree selection, value and derivative evaluation,
and origin power coefficients without exposing native workspace. It is a
fitting API, not global interpolation; see the
[polynomial-fitting guide](../../docs/api/safe-polynomial-fitting.md).

The hosted `fftpack-real` feature provides reusable `f32` plans over the
reviewed real FFTPACK families: periodic packed real FFT, easy Fourier series,
full sine/cosine, and quarter-wave sine/cosine transforms. Each plan owns its
initialized work array and preserves the original in-place storage and native
normalization. The selected snapshot contains no reviewed `f64` real FFTPACK
counterparts. See the [real FFTPACK guide](../../docs/api/safe-real-fftpack.md).

The hosted `fftpack-complex` feature provides a one-dimensional in-place
`ComplexFftPlan32` over public `num_complex::Complex32`. It calls the reviewed
standard real-array `CFFTI1/CFFTF1/CFFTB1` interface, preserves the native
negative/positive exponent directions and `N` round-trip scale, and has no
`Complex64` plan because the selected snapshot contains no complex `f64`
roots. See the [complex FFTPACK guide](../../docs/api/safe-complex-fftpack.md).

The hosted `fishpack-cartesian-2d` feature provides an owned `f32` Cartesian
Poisson/Helmholtz finite-difference solve over reviewed `HWSCRT`. It has typed
periodic, value, derivative, and mixed edge conditions; explicit perturbation
and non-uniqueness reporting; private checked workspace; and process-global
native serialization. Three-dimensional and non-Cartesian FISHPACK families
remain deferred. See the [Cartesian FISHPACK guide](../../docs/api/safe-fishpack-cartesian-2d.md).

The hosted `fishpack-pois3d` feature provides a separate owned `f32` facade
over the structured discrete `POIS3D` system. It has typed transverse
ghost-node rules plus checked cyclic or noncyclic third-axis coefficients; it
is not an arbitrary six-face three-dimensional PDE interface. See the
[structured POIS3D guide](../../docs/api/safe-fishpack-pois3d.md).

The hosted `banded-linear-systems` feature provides compact general-band
`f32`/`f64` LU factors, reusable direct and transpose solves, `SGBCO`/`DGBCO`
reciprocal 1-norm condition estimates, and `SGBDI`/`DGBDI` base-ten scaled
determinant metadata. Input is copied only into the private expanded LINPACK
layout; no dense conversion or matrix dependency is introduced. Exact singular
pivots remain errors, while a successful zero reciprocal estimate is preserved
as a numerical diagnostic. See the [banded-system guide](../../docs/api/safe-banded-linear-systems.md).

The hosted `pchip` feature provides owned `f32` and `f64`
`PiecewiseCubicHermite` curves over reviewed PCHIP. Monotone, controlled, and
PCHSP endpoint derivative construction feed native value/derivative evaluation
and definite integration. Knots remain strictly ordered as supplied, native
endpoint extrapolation is opt-in, and the shared XERROR runtime keeps calls
process-serialized. See the [PCHIP guide](../../docs/api/safe-pchip.md).

The hosted `bspline` feature provides owned `f32` and `f64` `BSpline` curves.
Reviewed `BINTK`/`DBINTK` construct exact interpolants from strictly increasing
nodes, values, a complete caller-supplied knot sequence, and order;
`BVALU`/`DBVALU` evaluate values or derivatives and `BSQAD`/`DBSQAD` integrate.
There is no hidden knot-generation policy, sorting, coefficient conversion, or
caller-managed workspace. See the [B-spline guide](../../docs/api/safe-bspline.md).
The additive `bspline-cubic-interpolation` feature supplies typed fixed-cubic
`BINT4`/`DBINT4` construction with explicit endpoint derivative and
knot-placement policies.

The hosted `roots-polynomial` feature supplies owned single-precision complex
polynomial roots over `RPZERO`/`CPZERO` and `RPQR79`/`CPQR79`. Iterative
drivers preserve documented best roots on their iteration limit; companion-QR
nonconvergence is an error because the source does not promise partial output.
See the [roots guide](../../docs/api/safe-roots.md).

The hosted `piecewise-polynomial` feature provides owned `f32` and `f64`
right-Taylor PP curves over reviewed `PPVAL`/`DPPVAL` evaluation and
`PPQAD`/`DPPQAD` exact integration. With `bspline`, it also performs exact
`BSPPP`/`DBSPPP` B-spline-to-PP conversion. Breakpoints are strictly
increasing and never reordered; native extrapolation is not used. PCHIP and
PP-to-B-spline conversion remain deferred. See the
[piecewise-polynomial guide](../../docs/api/safe-piecewise-polynomial.md).

The hosted `tabulated-data` feature provides `TabulatedData<f32>` and
`TabulatedData<f64>` for finite, strictly increasing sampled data. It creates
private Newton polynomial representations with `POLINT`/`DPLINT`, evaluates
values and derivatives with `POLYVL`/`DPOLVL`, derives Taylor coefficients
with `POLCOF`/`DPOLCF`, and integrates arbitrary-spacing samples with
`AVINT`/`DAVINT`. It has no callback, caller-owned workspace, or implicit
sorting policy. See the [tabulated-data guide](../../docs/api/safe-tabulated-data.md).

The hosted `special-scalar-expanded` feature provides real scalar
logarithmic/Spence integrals and Carlson symmetric elliptic integrals in both
precisions. It has no caller workspace or external numerical dependency;
native saved initialization and XERROR keep it globally serialized. Complex,
sequence, arbitrary-workspace, and translated special-function APIs remain
deferred. See the [special-functions guide](../../docs/api/safe-special-functions.md).

The narrower `special-airy` feature exposes checked `f64` and `f32` real Ai
and Bi values plus their explicitly exponentially scaled variants through
`slatec::special::airy`. On non-positive inputs the scaled values equal their
unscaled counterparts. On positive inputs Ai is multiplied by
`exp(2 x^(3/2) / 3)` and Bi by `exp(-2 x^(3/2) / 3)`. The selected FNLIB set
has no separately promoted real derivative driver; complex Amos drivers and
Airy subsidiaries remain deferred. See the [Airy guide](../../docs/api/raw-special-airy.md).

Native concurrency is conservative. APIs that use the hosted legacy runtime
are process-serialized; the existing `no_std`/`alloc` BLAS and Jacobian-check
features remain backend-dependent and do not promise parallel native safety.
Matrix and vector storage is passed only under each function's documented
Fortran layout contract, with no implicit repacking or transposition. See the
repository [runtime concurrency and storage policy](../../docs/architecture/runtime-concurrency-and-storage-policy.md).

Native implementations are selected explicitly with `prebuilt`, `source-build`, `system`, or `external-backend`. No redistributable prebuilt provider is currently available because historical source rights remain unresolved.

Safe family features do not create a numerical implementation root. On the
supported GNU MinGW source-build release profile, each safe operation is laid
out to retain only its wrapper, checked shared support, and genuine native
closure. BLAS Level 1 is isolated per operation and precision; sampled Level
2/3 operations provide the same regression boundary. See the
[safe-facade link-granularity policy](../../docs/architecture/safe-facade-link-granularity.md).
