# Safe nonlinear easy-driver validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified candidates: 8
- Reviewed safe wrappers: 2
- Deferred candidates: 6
- Jacobian policy: `IOPT = 2` finite differences only; the ABI `JAC` argument is never called
- Workspace formula: `(3*N*N + 13*N)/2` checked elements
- Callback policy: shared scoped vector trampoline; panic and non-finite results are recorded while finite sentinel residuals preserve in-process containment
- Cancellation policy: not exposed; the historical negative-`IFLAG` path reaches fatal XERROR for this profile
- Concurrency policy: native calls serialize; cross-family callback nesting is rejected
- Semantic hash: `e52b2dfa866420276e7a013746e069158a1ea2040583d6e53044d04609e2cdf2`

The original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.
