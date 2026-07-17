# Safe nonlinear least-squares covariance estimation

The hosted `std` feature `least-squares-covariance` exposes the original
SLATEC `DCOV` (double precision) and `SCOV` (single precision) routines. It
does not implement a covariance algorithm in Rust. Both native routines
evaluate the residual vector and Jacobian at the supplied parameter point,
then factor the Jacobian without column pivoting.

For a full-column-rank Jacobian \(J\) with \(M\) residuals and \(N\)
parameters, the native matrix is

\[
  \frac{\sum_i r_i^2}{M-N}(J^T J)^{-1}, \quad M \ne N.
\]

When `M == N`, `SCOV`/`DCOV` return the unscaled \((J^T J)^{-1}\). The safe
default, `CovarianceScaling::ResidualVariance`, therefore requires `M > N`.
`CovarianceScaling::Native` permits the square case and preserves that native
convention. A caller-supplied known-variance scale is intentionally deferred:
the raw routine does not retain the unscaled inverse in the zero-residual
overdetermined case.

## Entry points and source mapping

- `estimate_covariance` and `estimate_covariance_f32` use a dense analytic
  Jacobian (`IOPT = 2`). They wrap `DCOV` and `SCOV` respectively.
- `estimate_covariance_finite_difference` and its f32 form select `IOPT = 1`.
  The native `DFDJC3`/`FDJAC3` subsidiary temporarily perturbs `X`, so the
  Rust facade copies the caller's parameter slice into private mutable native
  storage and restores no caller memory.
- `covariance_from_expert_fit` and its f32 form accept an eligible
  `DNLS1`/`SNLS1` result, but still call the residual and Jacobian closures at
  that result's parameters. `SCOV`/`DCOV` do **not** consume solver QR arrays.

The analytic Jacobian is the checked rectangular column-major `JacobianMut`:
`J[row, column] = d residual[row] / d parameter[column]`. `M >= N` is
required, all values must be finite, and the native workspaces are allocated
internally as `FVEC[M]`, `R[M*N]`, `WA1[N]`, `WA2[N]`, `WA3[N]`, and `WA4[M]`.

## Rank, layout, and statistical interpretation

`SCOV`/`DCOV` use an exact-zero diagonal check after unpivoted QR. They expose
no numerical rank threshold, generalized inverse, or pivot permutation. A
successful `CovarianceResult` therefore reports `rank == parameter_count` and
the identity permutation. A singular Jacobian returns `CovarianceError::RankDeficient`;
it is never represented as a misleading full-rank covariance matrix.

The Rust result expands the source routine's upper triangle into a full,
symmetric, column-major `parameter_count × parameter_count` allocation.
`standard_errors()` returns the square roots of its diagonal. `correlation_matrix()`
returns an error when any diagonal variance is zero, rather than dividing by
zero. Residual-variance scaling assumes independent residuals with common
variance and a meaningful local linear approximation; it is not a confidence
interval or an uncertainty guarantee.

## Runtime behavior

The callbacks use the shared contained SLATEC runtime: Rust panics and
non-finite residuals/Jacobian entries are recorded, never unwind through
Fortran, and nested callback-based SLATEC calls are rejected. Calls serialize
because the validated GNU Fortran `x86_64-w64-mingw32` profile has global
legacy-error state. A level-one `XERMSG` report for a singular Jacobian is
scoped as recoverable, and the old error setting is restored before the safe
call returns.

This is an `alloc`-using, `std`-requiring hosted native API. It is not a
bare-metal support claim. The only validated backend is GNU Fortran
`x86_64-w64-mingw32` with `ffi-profile-gnu-mingw-x86_64`.

See the maintained [linear example](../../examples/least_squares/covariance_linear_fit.rs),
[nonlinear example](../../examples/least_squares/covariance_nonlinear_fit.rs),
and [rank-deficient example](../../examples/least_squares/covariance_rank_deficient.rs).
