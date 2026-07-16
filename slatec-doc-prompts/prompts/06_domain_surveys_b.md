# Prompt 06 — Domain surveys B: quadrature, differential equations, interpolation, transforms, special functions and statistics

This is a standalone research task for the `slatec-rs` documentation knowledge base. Do not implement Rust code.

## Task

Create detailed domain surveys for:

- numerical quadrature and cubature;
- ordinary differential equations, differential-algebraic equations and boundary-value problems;
- interpolation, approximation and splines;
- Fourier and related transforms;
- special functions;
- probability and statistics;
- general utilities not naturally owned elsewhere.

For each domain cover:

- mathematical scope;
- principal routine families;
- package provenance;
- public versus subsidiary routines;
- precision variants;
- key algorithms;
- dependency patterns;
- common workspaces and callbacks;
- numerical limitations;
- modern equivalents;
- FFI and safe-wrapper considerations;
- tentative crate ownership.

## Required files

```text
docs/domains/quadrature.md
docs/domains/differential-equations.md
docs/domains/interpolation.md
docs/domains/transforms.md
docs/domains/special-functions.md
docs/domains/statistics.md
docs/domains/utilities.md
docs/domains/overview.md
```

The overview should cross-link all domain pages and summarize known inter-domain dependencies.

## Deliverable

Return a downloadable ZIP containing only the requested files, preserving repository-relative paths. Do not modify GitHub.
