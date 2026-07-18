# Concurrency relaxation roadmap

This report changes no runtime behavior. Every hosted native wrapper remains protected by the process-wide reentrant lock, and `BackendDependent` remains a provider qualification rather than a parallel-safety promise. Independent Rust buffers and `Send` values do not prove native reentrancy.

## Outcomes

- `candidate_backend_dependent_parallel`: 28 wrappers
- `not_candidate_mutable_native_state`: 162 wrappers

## Required evidence

The 28 BLAS Level 1 records are now qualified `ParallelSafe` only for the exact hash-verified `source-build-gnu-mingw-reviewed` backend and only for independent calls with non-overlapping mutable storage. Their existing direct, `core`-capable dispatch is unchanged. System archives, external linkage, named-but-unselected vendor libraries, and unknown providers remain `BackendDependent`. Every callback, XERROR, ODE, LP, and solver family retains process-global serialization.

Any other future relaxation must have a complete logical-statement source scan, GNU Fortran oracle agreement, bidirectional source/object reconciliation, an exact retained object closure, no reachable XERROR or Fortran-I/O state, and a documented compiler/provider/runtime concurrency contract. Family locks are considered only where mutable state is proved family-local.

Storage layout remains orthogonal: packed storage uses `matrixpacked`, conventional rectangular storage may use standard dense crates, and exact-layout slices/views remain valid without implicit repacking or transpose. LP external paging and unit lifecycle remain deferred; the current safe LP subset preflights resident capacity and contains no file-I/O implementation.
