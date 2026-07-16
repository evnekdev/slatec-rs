# Prompt 07 — Routine metadata schema and representative pilot catalogue

This is a standalone task for `slatec-rs`. The goal is to design and test documentation/metadata conventions before processing all SLATEC routines. Do not implement FFI.

## Task

Design a machine-readable schema for SLATEC routine metadata, then apply it to a carefully chosen pilot set of 20–30 representative routines spanning:

- core support/error handling;
- machine constants;
- BLAS or LINPACK;
- quadrature;
- interpolation;
- nonlinear equations or optimization;
- ODEs;
- special functions;
- transforms;
- sparse methods if available.

For every pilot routine extract and verify, where present:

- routine name;
- source file;
- subroutine/function kind;
- precision/type;
- purpose;
- GAMS code;
- user-callable status;
- origin package;
- call sequence;
- arguments and intent;
- callback signatures;
- work arrays;
- documented errors;
- direct dependencies;
- related routines;
- references;
- authors and revision information;
- candidate future domain crate;
- unresolved ABI questions.

## Required files

```text
metadata/routine-schema.md
metadata/routine-schema.example.toml
metadata/routines-pilot.toml
docs/routines/README.md
docs/routines/index.md
docs/routines/pilot/<one-file-per-routine>.md
```

## Important constraints

- The TOML must be parseable and consistent.
- The Markdown pages should use YAML front matter aligned with the schema.
- Do not fabricate missing argument intent or provenance.
- Mark inferred fields explicitly.
- Cite each routine’s official source/prologue.

## Tool recommendation

For only 20–30 routines, a high-reasoning ChatGPT model is suitable. For the complete catalogue, use Codex with scripts and validation.

## Deliverable

Return a downloadable ZIP containing only the requested paths. Do not edit GitHub.
