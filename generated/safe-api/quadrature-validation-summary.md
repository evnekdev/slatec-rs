# Safe adaptive quadrature validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Reviewed safe wrappers: 20
- Deferred quadrature interfaces: 38
- Callback policy: scoped thread-local trampoline; panics and non-finite values are contained
- Concurrency policy: native calls serialize; nested callback integration is rejected
- Workspace policy: each public family records its exact checked driver formula in the compact index
- Semantic hash: `ed76ddeea2ac3f85d37de9895b5f47c2a42812026c1a9a6e3a5bd783ecfa4ea7`

The original SLATEC Fortran routines remain the numerical implementation. Native execution evidence is profile-specific; detailed binaries and logs remain ignored.
