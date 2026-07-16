# Prompt 09 — Map the collected SLATEC knowledge to a future Rust crate architecture

This is a standalone documentation and architecture task for `slatec-rs`. Do not implement code.

## Inputs

The user may upload a ZIP containing the documentation produced by earlier prompts. Read all relevant files before forming conclusions.

## Task

Using the collected evidence, propose an incremental Rust crate architecture for SLATEC.

Cover:

- initial monolithic `slatec-sys` bootstrap;
- transition to internal modules;
- transition to separate native archives;
- eventual domain-specific `*-sys` crates;
- safe wrapper crates;
- facade crates;
- one-owner rule for Fortran symbols;
- handling shared support routines;
- BLAS/LAPACK backend strategy;
- feature flags versus separate crates;
- avoidance of cyclic dependencies;
- callback and global-state boundaries;
- metadata-driven source selection;
- versioning and publication order.

Every architectural decision must cite the documentation evidence that motivates it. Clearly mark proposals rather than historical facts.

## Required files

```text
docs/rust/crate-architecture.md
docs/rust/ffi-design.md
docs/rust/callback-safety.md
docs/rust/migration-roadmap.md
docs/rust/open-questions.md
```

Include at least two alternative architectures and explain why one is preferred.

## Deliverable

Return a downloadable ZIP containing only the five requested files. Do not edit GitHub.
