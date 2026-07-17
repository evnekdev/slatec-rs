# Expert nonlinear validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Expert wrappers: 4 (`SNSQ`/`DNSQ`, finite-difference and dense user Jacobian)
- Jacobian-check wrappers: 2 (`CHKDER`/`DCKDER`)
- Candidate and subsidiary records: 25
- Native status distinctions: `INFO=1..5` retained
- Evaluation counters: native `NFEV`/`NJEV` cross-checked with contained callback counts
- Workspace: exact checked `FJAC`, packed `R`, `DIAG`, `QTF`, and four work vectors
- Cancellation and `NPRINT` observer callbacks: deferred
- Semantic hash: `7a079c518adf815a85956727710211f2e3a2ccd1f3fdf0c9eeed8f47b528804c`

No numerical algorithm or selected Fortran source is copied into the safe API.
