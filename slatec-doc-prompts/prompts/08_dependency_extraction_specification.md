# Prompt 08 — Dependency extraction and validation specification

This is a standalone planning/documentation task for `slatec-rs`. Do not implement the full parser unless specifically needed to validate examples.

## Task

Create a detailed specification for extracting and validating the SLATEC routine dependency graph from official sources.

Cover:

- direct dependencies listed in prologues;
- dependencies inferred from source calls;
- Netlib tree and “plus dependencies” data;
- external library calls;
- intrinsic functions;
- precision-paired routines;
- alternate entries and aliases;
- common blocks and shared state;
- missing or conditional routines;
- cycles and strongly connected components;
- duplicate symbols;
- one-routine/one-owning-crate constraints;
- source-file versus symbol ownership;
- validation against linker behaviour;
- outputs needed for Rust crate splitting.

Define canonical output formats for:

- direct dependency graph;
- transitive closure;
- unresolved references;
- strongly connected components;
- candidate ownership mapping;
- generated Markdown summaries.

## Required files

```text
docs/architecture/dependency-extraction-spec.md
docs/rust/domain-boundaries.md
docs/rust/native-build-strategy.md
metadata/dependency-schema.md
metadata/dependency-schema.example.toml
```

Include a worked example using a small routine family such as adaptive quadrature, but verify actual names and dependencies from official sources.

## Recommendation section

State which parts should be performed in ChatGPT and which require Codex. The final complete extraction should be recommended for Codex High because it requires repository-wide parsing, file generation and validation.

## Deliverable

Return a downloadable ZIP containing only the requested files. Do not modify GitHub directly.
