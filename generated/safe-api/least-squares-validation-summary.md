# Safe nonlinear least-squares easy-driver validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified public and subsidiary records: 18
- Easy safe wrappers: 2 (`SNLS1E`, `DNLS1E`)
- Expert safe wrappers: 4 (`SNLS1`, `DNLS1`, finite-difference and dense analytic Jacobian modes)
- Deferred public routines: 2 (`SCOV`, `DCOV`)
- Dimensions: native `M >= N` is prevalidated; residual and parameter dimensions remain distinct
- Expert workspace: checked `FJAC[M*N]`, `IPVT[N]`, `DIAG[N]`, `QTF[N]`, `WA1..WA3[N]`, and `WA4[M]` allocations
- Legacy errors: scoped `XGETF`/`XSETF(0)` lets documented level-one numerical completion statuses return, then restores the prior process-global control
- State: callback calls serialize; nested callback-based families are rejected
- Semantic hash: `21bbc4a397dd1edd29029bded99dabd0f2347af0d821addf518f0d5eb6900b09`

The original SLATEC Fortran routines remain the numerical implementation.
