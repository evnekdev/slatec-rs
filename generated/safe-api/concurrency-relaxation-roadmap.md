# Concurrency relaxation roadmap

This report changes no runtime behavior. Every hosted native wrapper remains protected by the process-wide reentrant lock, and `BackendDependent` remains a provider qualification rather than a parallel-safety promise. Independent Rust buffers and `Send` values do not prove native reentrancy.

## Outcomes

- `candidate_backend_dependent_parallel`: 28 wrappers
- `not_candidate_mutable_native_state`: 160 wrappers

## Required evidence

A future relaxation must have a complete logical-statement source scan, GNU Fortran oracle agreement, bidirectional source/object reconciliation, an exact retained object closure, no reachable XERROR or Fortran-I/O state, and a documented compiler/provider/runtime concurrency contract. Family locks are considered only where mutable state is proved family-local. No current record is promoted to native parallel execution.

Storage layout remains orthogonal: packed storage uses `matrixpacked`, conventional rectangular storage may use standard dense crates, and exact-layout slices/views remain valid without implicit repacking or transpose. The existing LP paging and unit-lifecycle deferral is unchanged.
