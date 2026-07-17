# Safe nonlinear least-squares covariance validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Public safe wrappers: 6 (`SCOV`, `DCOV`, forward-difference and dense analytic Jacobians)
- Native mathematical result: `RSS/(M-N) * inverse(J^T J)` for `M > N`; unscaled `inverse(J^T J)` for `M = N`
- Matrix layout: upper native triangle is expanded deterministically to a full symmetric column-major `N x N` matrix
- Rank: unpivoted QR, no native threshold or permutation; singular `INFO=2` returns `RankDeficient` rather than a generalized inverse
- State: no solver factorization is consumed; callbacks evaluate a fresh residual/Jacobian at final parameters
- Scaling: `ResidualVariance` requires `M > N`; known variance and pseudoinverse policies are deferred
- Legacy errors: `XGETF`/`XSETF(0)` is scoped across success and singular return paths then restored
- Semantic hash: `15b921b865dfe2825855a8711c92ad37906a9474d14fb80b3240604512d7b3ea`

The original SLATEC Fortran routines remain the numerical implementation.
