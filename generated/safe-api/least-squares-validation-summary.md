# Safe nonlinear least-squares validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified public and subsidiary records: 18
- Easy safe wrappers: 2 (`SNLS1E`, `DNLS1E`)
- Expert safe wrappers: 4 (`SNLS1`, `DNLS1`, finite-difference and dense analytic Jacobian modes)
- Covariance safe wrappers: 6 (`SCOV`, `DCOV`, finite-difference and dense analytic Jacobian modes)
- Deferred public routines: 0
- Dimensions: native `M >= N` is prevalidated; residual and parameter dimensions remain distinct
- Legacy errors: scoped `XGETF`/`XSETF(0)` lets documented level-one numerical completion statuses return, then restores the prior process-global control
- State: callback calls serialize; nested callback-based families are rejected
- Semantic hash: `15b921b865dfe2825855a8711c92ad37906a9474d14fb80b3240604512d7b3ea`

The original SLATEC Fortran routines remain the numerical implementation.
