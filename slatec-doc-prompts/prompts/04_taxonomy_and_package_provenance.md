# Prompt 04 — Mathematical taxonomy, subpackages and provenance

This is a standalone documentation task for `slatec-rs`. No Rust implementation is requested.

## Task

Build a rigorous taxonomy showing how SLATEC is organized mathematically and historically.

Research and explain:

- SLATEC table-of-contents groupings;
- GAMS classification codes used by SLATEC routines;
- major mathematical domains;
- imported or incorporated packages;
- original SLATEC contributions not cleanly attributable to standalone packages;
- differences between provenance-based and function-based grouping;
- overlaps and ambiguous routine ownership;
- preliminary implications for future domain-specific Rust crates.

At minimum cover:

- BLAS;
- LINPACK;
- EISPACK;
- FFTPACK;
- QUADPACK;
- special-function collections;
- interpolation/spline collections;
- ODE/BVP/DAE collections;
- optimization and nonlinear-equation routines;
- sparse methods;
- statistics and utility routines.

## Required files

```text
docs/taxonomy/gams-overview.md
docs/taxonomy/slatec-subpackages.md
docs/taxonomy/gams-to-domain-map.md
docs/packages/overview.md
docs/packages/imported-packages.md
metadata/domains.toml
metadata/packages.toml
```

## Metadata requirements

`domains.toml` should define stable domain IDs, descriptions and tentative dependency layers.

`packages.toml` should define package IDs, provenance, included routine families when verified, and authoritative documentation sources.

Do not pretend uncertain package boundaries are exact. Add confidence levels and notes.

## Deliverable

Return a downloadable ZIP containing only the requested files, preserving paths. Do not modify GitHub directly.
