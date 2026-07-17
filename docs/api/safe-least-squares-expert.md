# Safe expert nonlinear least-squares drivers

The `least-squares-nonlinear-expert` feature exposes safe wrappers for the
original SLATEC `DNLS1` (double precision) and `SNLS1` (single precision)
Levenberg--Marquardt drivers. They solve `minimize 1/2 * sum_i r_i(x)^2` for
`M` residuals and `N` parameters. This is fitting, not square nonlinear root
finding; the reviewed native contract requires `M >= N`.

`least_squares_expert` and `least_squares_expert_f32` select `IOPT = 1` and
obtain an `M` by `N` Jacobian with SLATEC forward differences.
`least_squares_with_jacobian` and its f32 counterpart select `IOPT = 2` and
receive a checked `JacobianMut` view. Its storage is column-major:
`jacobian[row + column * leading_dimension]`, with `rows = M`, `cols = N`,
and `leading_dimension = M`. A Jacobian closure must fill every logical entry.

The expert option fields map directly to Fortran `FTOL`, `XTOL`, `GTOL`,
`MAXFEV`, `EPSFCN`, `MODE`/`DIAG`, and `FACTOR`. `NPRINT` is always zero, so
legacy observer callbacks are not exposed. The wrapper owns and checks exactly
`FJAC[M*N]`, `IPVT[N]`, `DIAG[N]`, `QTF[N]`, `WA1[N]`, `WA2[N]`, `WA3[N]`, and
`WA4[M]`; no caller workspace or raw array is required.

Residual and Jacobian panics, null or inconsistent native callback views, and
NaN/infinite outputs are contained before they cross the Fortran boundary.
Calls serialize on the process-wide native runtime lock, and nested
callback-bearing SLATEC calls are rejected. The wrapper scopes the level-one
legacy `XERROR` setting used for documented `INFO = 4..8` numerical completion
and restores the previous setting on every normal Rust return path.

Only the validated GNU Fortran `x86_64-w64-mingw32`
`ffi-profile-gnu-mingw-x86_64` backend is supported. This hosted `std` and
`alloc` API is not a bare-metal support claim. `SCOV` and `DCOV`, covariance
estimation are available separately through `least-squares-covariance`; they
recompute the residual and Jacobian at the supplied parameters rather than
consuming this driver's QR buffers. Bounds, robust loss functions,
cancellation, observer callbacks, and sparse Jacobians remain deferred.

See the generated [function index](function-index.md) and the maintained
[finite-difference](../../examples/least_squares/expert_finite_difference.rs)
and [analytic-Jacobian](../../examples/least_squares/expert_analytic_jacobian.rs)
examples.
