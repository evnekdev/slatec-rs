# BLAS Level 1 backend concurrency audit

- Candidate safe wrappers: 28; native routines: 14.
- The exact hash-verified `source-build` GNU MinGW objects contain no reachable mutable static state, COMMON, XERROR, Fortran I/O, callbacks, or external runtime imports. Independent calls with non-overlapping mutable storage are classified `ParallelSafe` for that backend only.
- `system`, `external-backend`, named vendor libraries without an explicit project profile, and unknown providers remain `BackendDependent`. Provider identity and version are not observable through those Cargo profiles.
- Safe slices prevent overlapping mutable arguments within one call. Cross-call safety still requires non-overlapping mutable storage; read-only buffers may be shared. Positive and negative nonzero increments are supported, zero increments are rejected, and zero-length calls return before FFI.
- The selected Netlib/SLATEC loops are single-threaded. External providers may use worker threads or global controls, so native-call concurrency may oversubscribe a process and is not necessarily faster. `slatec-rs` does not change provider thread counts.
- Runtime behavior is unchanged: BLAS Level 1 remains a direct `core`-capable FFI surface, while callback, XERROR, ODE, and solver families retain the process-global exclusive runtime lock.
- Storage layout and LP paging policies are unchanged.
