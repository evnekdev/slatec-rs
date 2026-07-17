# Safe nonlinear easy-driver validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified candidates: 8
- Reviewed safe wrappers: 2
- Deferred candidates: 2
- Jacobian policy: `IOPT = 2` finite differences only; the ABI `JAC` argument is never called
- Workspace formula: `(3*N*N + 13*N)/2` checked elements
- Callback policy: shared scoped vector trampoline; panic and non-finite results are recorded while finite sentinel residuals preserve in-process containment
- Cancellation policy: not exposed; the historical negative-`IFLAG` path reaches fatal XERROR for this profile
- Concurrency policy: native calls serialize; cross-family callback nesting is rejected
- Semantic hash: `9775b0af7f3c1d3de11020e677f9b3edb94a9b678541ec8c6acb17e5e5cf07b3`

The original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.
