# Prompt 05 — Domain surveys A: linear algebra, nonlinear equations, optimization and sparse methods

This is a standalone research task for the `slatec-rs` documentation knowledge base. No Rust implementation.

## Task

Create detailed domain surveys for:

1. dense and structured linear algebra;
2. nonlinear equations and root finding;
3. optimization and nonlinear least squares;
4. sparse linear algebra and iterative methods.

For each domain describe:

- mathematical problem classes;
- major routine families;
- user-facing versus internal routines;
- provenance and imported packages;
- precision variants;
- important dependencies;
- algorithm families;
- limitations and assumptions;
- relation to modern libraries;
- preliminary FFI risks;
- tentative safe-Rust API groupings;
- candidate future raw and safe crate boundaries.

The Rust sections are proposals and must be labelled as such.

## Required files

```text
docs/domains/linear-algebra.md
docs/domains/nonlinear-equations.md
docs/domains/optimization.md
docs/domains/sparse-methods.md
```

Use tables of representative routine families, but do not attempt an exhaustive routine catalogue unless authoritative indexes permit it.

## Deliverable

Return a downloadable ZIP containing only the four requested files. Preserve repository-relative paths and do not edit GitHub.
