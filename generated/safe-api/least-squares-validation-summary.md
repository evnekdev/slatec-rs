# Safe nonlinear least-squares easy-driver validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified public and subsidiary records: 18
- Safe wrappers: 2 (`SNLS1E`, `DNLS1E`)
- Deferred public routines: 4 (`SNLS1`, `DNLS1`, `SCOV`, `DCOV`)
- Dimensions: native `M >= N` is prevalidated; residual and parameter dimensions remain distinct
- Finite differences: `IOPT = 1`; no analytic Jacobian, cancellation, or observer callback is exposed
- Workspace: checked `IW[N]` and `WA[N*(M+5)+M]` allocations
- Legacy errors: scoped `XGETF`/`XSETF(0)` lets documented level-one numerical completion statuses return, then restores the prior process-global control
- State: callback calls serialize; nested callback-based families are rejected
- Semantic hash: `f15dc7fb6c2817bfb480af6dd3b19c46f8e160218eaff2482bce89458b773834`

The original SLATEC Fortran routines remain the numerical implementation.
