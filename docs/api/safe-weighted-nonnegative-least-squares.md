# Safe weighted nonnegative linear least squares

The hosted `least-squares-linear-nonnegative` feature exposes
`solve_nonnegative_least_squares` and its `_f32` counterpart over original
SLATEC `DWNNLS` and `WNNLS`. These are constrained **linear** least-squares
drivers, not nonlinear least squares, bound-constrained least squares, or
linear programming.

## Mathematical contract

The reviewed routines solve

```text
minimize ||A x - b||₂, subject to E x = f,
```

with a selected subset of components constrained to `xᵢ >= 0`. The equality
block is optional. The historical “weighted” name refers to the native
driver's internal handling of equality equations; this safe API does not claim
caller-supplied statistical weights.

`WNNLS` requires all free variables before its nonnegative variables. The
Rust `VariableConstraint` slice accepts either ordering. The wrapper performs
a stable private column permutation, copies caller matrices into owned mutable
Fortran storage, and restores the solution to the caller's order. It never
overwrites caller data.

## Storage, status, and runtime

`MatrixRef` requires column-major storage and a checked leading dimension.
The wrapper creates `W(MDW,N+1)` with equality rows first and least-squares
rows second, allocates the exact `WORK[ME+MA+5*N]` and `IWORK[ME+MA+N]`
arrays, and sets the no-options `PRGOPT(1)=1` sentinel.

The reviewed `MODE` interface supplies only normal completion, an active-set
iteration limit, and usage/storage error. It does not return a structured
rank or infeasibility classification; the safe result therefore does not
invent one. Separate native and recomputed least-squares/equality norms make
the residual convention explicit.

Calls are serialized because the selected GNU MinGW profile reaches saved
machine-constant and legacy-error support. This is a hosted `std` + `alloc`
API for `x86_64-w64-mingw32`, not a bare-metal support claim.

## Deferred drivers

`SBOLS`/`DBOLS` are exposed separately through the bounded linear
least-squares feature; they have a different per-variable bound model.
`SBOCLS`/`DBOCLS` and `SPLP`/`DSPLP` remain deferred because
their broader equality, inequality, workspace, or linear-program contracts
are not implied by this narrow `WNNLS`/`DWNNLS` facade.

See the generated [function index](function-index.md), compact argument map,
and the `examples/least_squares/` examples for exact Rust-to-Fortran mapping.
