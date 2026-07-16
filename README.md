# slatec-rs

Workspace skeleton for Rust bindings and safe wrappers around selected SLATEC numerical routines.

## Intended layers

- `slatec-sys`: raw, unsafe FFI declarations and native build integration.
- `slatec-core`: shared safe types, errors, and workspace abstractions.
- `slatec`: safe public API grouped by numerical domain.
- `slatec-tools`: development utilities such as metadata and dependency-graph generation.

No Rust implementation has been added yet.

## Canonical corpus preparation

`slatec-tools` provides the `slatec-corpus` command for acquiring and inventorying the policy-v1 SLATEC archive without adding its source bytes to Git. See [`docs/extraction/corpus-tooling.md`](docs/extraction/corpus-tooling.md) for the offline workflow, evidence layout, and rights boundary.
