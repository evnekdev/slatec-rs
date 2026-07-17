# Safe adaptive quadrature validation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`
- Profile: `ffi-profile-gnu-mingw-x86_64`
- Reviewed safe wrappers: 8
- Deferred quadrature interfaces: 50
- Callback policy: scoped thread-local trampoline; panics and non-finite values are contained
- Concurrency policy: native calls serialize; nested callback integration is rejected
- Workspace policy: safe API allocates `LIMIT` integers and `4 * LIMIT` numeric values
- Semantic hash: `1eb8fedde36d0dfcede4459abc791e810c59e9008590692cfea866c7dc5bb5c3`

The original SLATEC Fortran routines remain the numerical implementation. Native execution evidence is profile-specific; detailed binaries and logs remain ignored.
