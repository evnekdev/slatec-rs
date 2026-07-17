# Safe equality and inequality constrained linear least squares

The `least-squares-linear-constrained` feature exposes the reviewed SLATEC
drivers `LSEI` (single precision) and `DLSEI` (double precision) through
`slatec::constrained_least_squares`.  This is a hosted `std` and `alloc` API:
it copies input blocks, allocates the native matrix and work arrays, serializes
the validated GNU MinGW native runtime, and restores the legacy XERROR setting
before releasing that lock.  It is API `no_std`-separable when disabled, but it
is not a bare-metal native-backend claim.

## Problem and block order

The Rust problem represents the reviewed native problem directly:

```text
minimize ||A x - b||₂
subject to E x = f
           G x >= h
```

`ConstrainedLeastSquaresProblem` holds `A,b`, the optional equality block
`E,f`, and the optional lower-sided inequality block `G,h` separately.  The
wrapper makes an initialized, mutable, column-major native augmented matrix in
the exact `DLSEI` order: equality rows, objective rows, then inequality rows;
the final column is each row's right-hand side.  Caller-owned slices are never
passed as mutable storage.  Optional blocks use zero row counts, which the
reviewed driver permits.

`GreaterEqualConstraints` deliberately names the native direction.  No
unmentioned sign reversal is performed.  A returned inequality slack is
`Gx - h`, so a non-negative slack means that row is satisfied.

## Results and statuses

`RNORME` is returned as `equality_residual_norm = ||f - E x||₂`; `RNORML` is
returned as `objective_residual_norm = ||b - A x||₂`.  The two rank outputs
remain distinct: `IP(1)` is the equality rank and `IP(2)` is the reduced
objective rank.  These ranks describe numerical factorization decisions, not
a uniqueness guarantee.

`MODE = 0` yields `Converged`.  `MODE = 1` yields a structured result with
`EqualityConstraintsContradictory`: the source documents its generalized
inverse solution and both residual norms.  `MODE = 2` (inequalities
contradictory) and `MODE = 3` (equality and inequality blocks jointly
contradictory) return errors because the driver leaves solution output
undefined.  `MODE = 4` is treated as a native-contract violation after safe
shape and workspace validation.

## Controls and workspace

`ConstrainedLeastSquaresOptions::rank_tolerance` is optional and, when set,
is supplied to both reviewed rank controls (`PRGOPT` keys 4 and 5).  `None`
keeps the SLATEC default.  The historical covariance option and the raw
linked-list option language are deferred.

For `ME` equality rows, `MA` objective rows, `MG` inequality rows, and `N`
variables, the wrapper allocates exactly:

```text
W:  (ME + MA + MG) * (N + 1) elements, leading dimension ME + MA + MG
WS: 2 * (ME + N) + max(MA + MG, N) + (MG + 2) * (N + 7) elements
IP: MG + 2 * N + 2 elements
```

All additions, multiplications, and GNU Fortran `INTEGER` conversions are
checked before the native call.

## Scope

This feature does not add mixed variable bounds with general constraints,
dual multipliers, active-set exposure, sparse matrices, warm starts,
constrained covariance, quadratic programming, or linear programming.
The only validated native profile is GNU Fortran `x86_64-w64-mingw32`.
