# Safe bounded linear least squares

The hosted `least-squares-linear-bounded` feature exposes the original SLATEC
`DBOLS` (double precision) and `SBOLS` (single precision) drivers as
`solve_bounded_least_squares` and `solve_bounded_least_squares_f32`. This is
dense linear least squares with independent closed variable bounds. It is not
nonlinear fitting, the `WNNLS` equality/nonnegative interface, general linear
constraint solving, quadratic programming, or linear programming.

## Mathematical and storage contract

The reviewed driver solves

```text
minimize ||A x - b||₂ subject to per-variable closed bounds.
```

`VariableBounds` maps the native `IND` encoding without exposing raw integers:
`Lower` uses `IND=1`, `Upper` uses `IND=2`, `Between` and `Fixed` use `IND=3`,
and `Unbounded` uses `IND=4`. The source accepts equal two-sided endpoints, so
`Fixed(value)` is a validated representation. Rust infinities are never sent to
Fortran: absence of a bound is represented solely by `Unbounded`.

`MatrixRef` is a checked column-major view. The wrapper copies its immutable
matrix and RHS into native `W(MROWS,NCOLS+1)`, placing the RHS in the final
column, because `SBOLS`/`DBOLS` overwrites `W`, `BL`, `BU`, `IND`, and all work
arrays. The exact internal allocations are `W=M*(N+1)`, `BL/BU/IND=N`,
`RW=5*N`, and `IW=2*N`, all through checked arithmetic.

## Options, results, and rank

The safe options retain only reviewed column scaling (`Nominal`,
`EuclideanColumns`, or `Identity`) and a positive iteration cap. They compile
the documented legacy `IOPT` protocol internally; raw sequential accumulation,
user-supplied scaling storage, rank-tolerance tuning, debug options, and the
`SBOLSM`/`DBOLSM` expert workspaces remain deferred.

`residual_norm` is native `RNORM`, the Euclidean residual length, not a square
or RMS measure. Native `MODE >= 0` becomes `Converged`; `MODE=-22` becomes a
result with `MaximumIterations` and an approximate solution. The routines use
a private dependence test but do not return a stable rank or active-bound map,
so this API does not claim uniqueness for rank-deficient problems.

## Runtime and support

This is a hosted `std` + `alloc` API for the validated GNU Fortran
`x86_64-w64-mingw32` profile. Calls serialize saved native state and snapshot,
temporarily configure, and restore legacy XERROR behavior only for the
documented recoverable iteration-limit route. It is not a `no_std` or
bare-metal native-runtime claim.

## Deferred scope

`SPLP`/`DSPLP` remain deferred, along with arbitrary general inequalities,
sparse storage, warm starts, dual values, sensitivity analysis, quadratic
programming, and linear programming. Combined bounds on variables and bounds
on linear expressions are available separately through
`least-squares-linear-bounded-constrained`.

See [`examples/least_squares/bounded_fit.rs`](../../examples/least_squares/bounded_fit.rs),
[`mixed_bounds.rs`](../../examples/least_squares/mixed_bounds.rs), and
[`fixed_parameter_fit.rs`](../../examples/least_squares/fixed_parameter_fit.rs).
