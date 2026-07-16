# slatec-rs

Workspace skeleton for Rust bindings and safe wrappers around selected SLATEC numerical routines.

## Intended layers

- `slatec-sys`: raw, unsafe FFI declarations and native build integration.
- `slatec-core`: shared safe types, errors, and workspace abstractions.
- `slatec`: safe public API grouped by numerical domain.
- `slatec-tools`: development utilities such as metadata and dependency-graph generation.

No Rust implementation has been added yet.
