# slatec

Safe, family-featured Rust APIs over validated original SLATEC Fortran routines.

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

The hosted `ode-sdrive-expert` feature provides owned real explicit-IVP
sessions over original `SDRIV3`/`DDRIV3`. Its first scope has only a
panic-contained RHS callback and same-direction continuation; event roots,
Jacobians, mass matrices, DAEs, and interpolation are deliberately deferred.

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

The hosted `bspline` feature provides owned `f32` and `f64` `BSpline` curves
from validated nondecreasing knots, coefficients, and order. It calls reviewed
SLATEC `BVALU`/`DBVALU` for value or derivative evaluation and
`BSQAD`/`DBSQAD` for scalar definite integration, with no hidden sorting,
coefficient conversion, or caller-managed workspace. See the
[B-spline guide](../../docs/api/safe-bspline.md).

The hosted `piecewise-polynomial` feature provides owned `f32` and `f64`
right-Taylor PP curves over reviewed `PPVAL`/`DPPVAL` evaluation and
`PPQAD`/`DPPQAD` exact integration. With `bspline`, it also performs exact
`BSPPP`/`DBSPPP` B-spline-to-PP conversion. Breakpoints are strictly
increasing and never reordered; native extrapolation is not used. PCHIP and
PP-to-B-spline conversion remain deferred. See the
[piecewise-polynomial guide](../../docs/api/safe-piecewise-polynomial.md).

The hosted `special-scalar-expanded` feature provides real scalar
logarithmic/Spence integrals and Carlson symmetric elliptic integrals in both
precisions. It has no caller workspace or external numerical dependency;
native saved initialization and XERROR keep it globally serialized. Complex,
sequence, arbitrary-workspace, and translated special-function APIs remain
deferred. See the [special-functions guide](../../docs/api/safe-special-functions.md).

Native concurrency is conservative. APIs that use the hosted legacy runtime
are process-serialized; the existing `no_std`/`alloc` BLAS and Jacobian-check
features remain backend-dependent and do not promise parallel native safety.
Matrix and vector storage is passed only under each function's documented
Fortran layout contract, with no implicit repacking or transposition. See the
repository [runtime concurrency and storage policy](../../docs/architecture/runtime-concurrency-and-storage-policy.md).

Native implementations are selected explicitly with `prebuilt`, `source-build`, `system`, or `external-backend`. No redistributable prebuilt provider is currently available because historical source rights remain unresolved.
