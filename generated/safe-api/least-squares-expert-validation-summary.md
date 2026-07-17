# Safe expert nonlinear least-squares validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Expert candidate and subsidiary records: 16
- Expert safe wrappers: 4 (`SNLS1`, `DNLS1`; `IOPT = 1` finite differences and `IOPT = 2` dense analytic Jacobians)
- Deferred records: 6 (`SCOV`/`DCOV`, `IOPT = 3`, observer callbacks, cancellation, and covariance policy)
- Dimensions: `M >= N`, `LDFJAC = M`, and all `M*N` storage arithmetic are checked
- Controls: checked `FTOL`, `XTOL`, `GTOL`, `MAXFEV`, `EPSFCN`, `MODE`/`DIAG`, and `FACTOR`; `NPRINT = 0`
- Counts: native `NFEV` and `NJEV` are retained and checked against contained callback counts
- State: native calls serialize; panic, non-finite output, and nested callback entry are contained; scoped `XGETF`/`XSETF(0)` is restored on every Rust return path
- Semantic hash: `21bbc4a397dd1edd29029bded99dabd0f2347af0d821addf518f0d5eb6900b09`

The original SLATEC Fortran routines remain the numerical implementation.
