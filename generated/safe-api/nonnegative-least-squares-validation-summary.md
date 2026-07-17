# Safe weighted nonnegative linear least squares

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Reviewed public wrappers: 2 (`DWNNLS`, `WNNLS`).
- Contract: exact equality block `E x = f`, least-squares block `min ||A x-b||₂`, with a native free/nonnegative variable partition.
- Program options: safe wrapper supplies `PRGOPT(1)=1`; WNNLS historical weighting is internal equality handling, not a public user-weight API.
- Storage: immutable caller matrices are copied to owned column-major `W(MDW,N+1)` before native mutation.
- Runtime: calls serialize saved machine-constant and legacy-error support for `ffi-profile-gnu-mingw-x86_64`.
- Deferred: bounded, broader constrained, and linear-programming drivers remain out of scope.
