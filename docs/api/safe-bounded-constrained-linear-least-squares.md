# Safe bounded constrained linear least squares

The hosted `least-squares-linear-bounded-constrained` feature exposes the
original SLATEC `DBOCLS` (double precision) and `SBOCLS` (single precision)
drivers as `solve_bounded_constrained_least_squares` and
`solve_bounded_constrained_least_squares_f32`.

## Actual native model

The native driver minimizes

```text
||E x - f||₂
```

while placing closed bounds on both the original variables `x` and the
auxiliary expressions `y = Cx`. A fixed auxiliary bound encodes `Cx = d`; an
interval encodes `lower <= Cx <= upper`. Thus this is not a generic merger of
`DBOLS` and `DLSEI`, and it is not linear programming. The safe API exposes
the latter bounds through `BoundedLinearConstraints`, whose matrix is `C` and
whose `VariableBounds` apply to each corresponding expression.

`VariableBounds` hides native `IND`: `Lower`, `Upper`, `Between`, and
`Fixed` map to closed one- or two-sided bounds, while `Unbounded` is the only
representation of no bound. Rust infinity is never passed to Fortran.

## Storage and workspace

The caller’s column-major `MatrixRef` inputs remain immutable. The wrapper
constructs zero-initialized owned column-major native storage:

```text
MDW = MCON + max(MROWS, NCOLS)
W   = MDW * (NCOLS + MCON + 1)
X   = 2 * (NCOLS + MCON) + 2
RW  = 6 * NCOLS + 5 * MCON
IW  = 2 * (NCOLS + MCON)
```

`C` is copied into the first `MCON` rows. Objective rows `[E | f]` follow;
the right-hand side is column `NCOLS + 1` in the original Fortran numbering.
The following `MCON` columns are native auxiliary-variable storage. All
arithmetic and Fortran integer conversions are checked.

The safe options select only the reviewed default native filter. `DBOCLS` and
`SBOCLS` write nested option data, so the wrapper owns a 17-element `IOPT`
array even though its first entry is the terminating `99`. The recursive
legacy option language, scaling controls, preprocessing controls, and raw
workspaces remain deferred.

## Results, feasibility, and runtime

`objective_residual_norm` is native `RNORM`, `||E x-f||₂`.
`constraint_residual_norm` is native `RNORMC`: under the default native
filter it reports the amount by which bounds on `Cx` had to be relaxed to
make the transformed constraint system reachable. A nonzero value produces
`ConstraintFeasibility::BoundsRelaxed`; it is not silently treated as exact
constraint satisfaction.

Native `MODE >= 0` is `Converged`; the documented `MODE = -22` iteration
termination preserves its approximate solution as `MaximumIterations`.
Other source-defined negative ranges are contained as native-contract errors.

This is a `std` + `alloc`, hosted-native API. Calls serialize the native
driver’s saved state and scope only documented level-one legacy XERROR
behavior, restoring it before return. The API is not a bare-metal or `no_std`
native-runtime claim.

## Deferred scope

`SPLP`/`DSPLP` linear programming, dual multipliers, active-set state,
sensitivity analysis, constrained covariance, sparse matrices, warm starts,
and arbitrary user-defined objectives remain deferred.

See [`bounded_constrained_fit.rs`](../../examples/least_squares/bounded_constrained_fit.rs),
[`active_bound_and_constraint.rs`](../../examples/least_squares/active_bound_and_constraint.rs),
and [`infeasible_bounds_and_constraints.rs`](../../examples/least_squares/infeasible_bounds_and_constraints.rs).
