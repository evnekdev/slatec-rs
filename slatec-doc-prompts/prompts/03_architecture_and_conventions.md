# Prompt 03 — SLATEC architecture, conventions and support infrastructure

This is a standalone task for the documentation phase of `slatec-rs`. Do not implement Rust code.

## Task

Distil the official SLATEC guide, source layout and representative routine prologues into a detailed architecture reference.

Cover:

- overall library architecture;
- source-file organization and distribution model;
- user-callable versus subsidiary routines;
- routine prologue structure;
- naming conventions;
- single, double, complex and integer variants;
- argument-passing conventions;
- workspace arrays;
- callback patterns;
- common blocks and global state where present;
- machine constants (`D1MACH`, `R1MACH`, `I1MACH`, related routines);
- error handling (`XERROR`, `XERMSG`, related infrastructure);
- portability assumptions;
- Fortran standard and compiler-era considerations;
- direct and transitive dependency concepts;
- static-library linking implications.

Use representative routine examples, but do not create individual routine pages yet.

## Required files

```text
docs/architecture/library-architecture.md
docs/architecture/source-organization.md
docs/architecture/routine-prologues.md
docs/architecture/naming-and-precision.md
docs/architecture/error-handling.md
docs/architecture/machine-constants.md
docs/architecture/dependency-model.md
```

## Required distinctions

For every topic, distinguish:

- what the official material states;
- what can be inferred from source inspection;
- implications for future Rust FFI;
- open questions requiring later experiments.

## Quality bar

This should function as a technical reference, not a superficial overview. Cite source files and manuals precisely. Avoid claiming all routines follow a convention if exceptions exist.

## Deliverable

Return a downloadable ZIP containing only the seven requested files. Preserve repository-relative paths. Do not edit GitHub.
