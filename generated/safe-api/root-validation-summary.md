# Safe-root validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Classified candidates: 14
- Reviewed safe wrappers: 6
- Deferred candidates: 8
- Scalar callback policy: shared scoped thread-local trampoline; panics and non-finite results are contained
- Polynomial policy: owned `Complex32` inputs and outputs; exact private workspaces; `CPZERO`/`RPZERO` retain documented partial roots on their iteration limit, while QR nonconvergence remains an error because source does not promise partial roots
- Concurrency policy: root native calls serialize through the process-wide runtime lock
- Semantic hash: `c9b926d46c9af81fe41bd457beb7b3bf558a99f7a2d0b4f2296738681673adb3`

The original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.
