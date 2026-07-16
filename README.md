# slatec-rs

Workspace skeleton for Rust bindings and safe wrappers around selected SLATEC numerical routines.

## Intended layers

- `slatec-sys`: raw, unsafe FFI declarations and native build integration.
- `slatec-core`: shared safe types, errors, and workspace abstractions.
- `slatec`: safe public API grouped by numerical domain.
- `slatec-tools`: development utilities such as metadata and dependency-graph generation.

No Rust implementation has been added yet.

## Canonical corpus preparation

`slatec-tools` provides the `slatec-corpus` command for acquiring and inventorying the policy-v1 `main-src` archive subset without adding its source bytes to Git. The 735 selected `src/*.f` files are reproducible evidence for that subset, not a claim to be the complete SLATEC library. See [`docs/extraction/corpus-tooling.md`](docs/extraction/corpus-tooling.md) for the offline workflow, evidence layout, and rights boundary; the [full-corpus audit policy](docs/source-corpus/corpus-completeness-audit-policy.md) defines the separate relocated-subset audit.

With cached evidence, `slatec-corpus select-full-corpus --offline` derives the
separate complete SLATEC-hosted provider profile. It does not alter the
`main-src` snapshot or make a redistribution conclusion.

Follow-on extraction stages currently include the fixed-form program-unit scanner and the SLATEC prologue parser. Both consume verified local evidence, write compact generated indexes, and keep detailed source-derived text in ignored evidence directories.
