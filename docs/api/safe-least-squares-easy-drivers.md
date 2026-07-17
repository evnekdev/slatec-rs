# Safe nonlinear least-squares easy drivers

The `least-squares-nonlinear-easy` feature exposes residual-only wrappers over
the original SLATEC `DNLS1E` and `SNLS1E` drivers. They minimize
`1/2 * sum_i r_i(x)^2`; they do not solve a square nonlinear equation system.

The wrapper selects the documented `IOPT = 1` forward-difference mode and
`NPRINT = 0`. It validates `M >= N`, contains Rust callback panics and
non-finite residuals, allocates `IW[N]` and `WA[N*(M+5)+M]`, and serializes
calls because the validated GNU Fortran runtime is process-global. Negative
callback flags, observer modes, analytic Jacobians, scaling controls, and
covariance estimation is not part of this easy-driver feature. The separate
`least-squares-nonlinear-expert` feature exposes reviewed
`SNLS1`/`DNLS1` controls and dense analytic Jacobians.

The internal wrapper snapshots the legacy `XGETF` control, temporarily selects
the documented nonfatal level-one policy while the driver returns a numerical
completion status, and restores the previous value before returning. This is
not a public legacy-error configuration API.

Only GNU Fortran `x86_64-w64-mingw32` with the validated
`ffi-profile-gnu-mingw-x86_64` profile is supported. The module requires
`std` and `alloc`; its public types do not imply bare-metal availability.

See the generated [function index](function-index.md) and the maintained
[linear fitting example](../../examples/least_squares/linear_fit.rs).
