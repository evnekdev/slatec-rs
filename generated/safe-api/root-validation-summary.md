# Safe scalar-root validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified candidates: 14
- Reviewed safe wrappers: 2
- Deferred candidates: 12
- Callback policy: shared scoped thread-local trampoline; panics and non-finite results are contained
- Concurrency policy: root and quadrature native calls serialize; nested callback calls are rejected
- Polynomial policy: complex-array candidates remain deferred pending a complete safe complex/workspace/residual contract
- Semantic hash: `6e675656de996d8aaeb95ab6a37950d0220a138ef9a3f96f2c9a1710beaf2fe8`

The original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.
