# Mathematical domain overview

## Purpose

This index links the domain surveys and summarizes their known dependencies. Domains are functional views for documentation and future safe APIs. They are not claims about historical package ownership.

## Domain surveys

| Domain | Page | Major provenance/package families |
|---|---|---|
| Dense and structured linear algebra | [Linear algebra](linear-algebra.md) | BLAS, LINPACK, EISPACK and SLATEC-specific drivers |
| Sparse linear algebra | [Sparse methods](sparse-methods.md) | SLAP and sparse support |
| Nonlinear equations | [Nonlinear equations](nonlinear-equations.md) | MINPACK-derived and independent scalar/polynomial families |
| Optimization and least squares | [Optimization](optimization.md) | MINPACK-derived NLS, constrained least-squares families, SPLP |
| Quadrature | [Quadrature](quadrature.md) | QUADPACK and additional Gaussian/spline helpers |
| ODE, DAE, BVP and PDE | [Differential equations](differential-equations.md) | DASSL, ODE/DEPAC families, BVP families, FISHPACK |
| Interpolation and approximation | [Interpolation](interpolation.md) | PCHIP, BSPLINE and fitting families |
| Integral transforms | [Transforms](transforms.md) | FFTPACK |
| Elementary and special functions | [Special functions](special-functions.md) | FNLIB, Amos-associated routines and other families |
| Probability and statistics | [Statistics](statistics.md) | cross-domain distribution, RNG and fitting routines |
| Utilities and runtime support | [Utilities](utilities.md) | machine constants, error handling, sorting and I/O |

## Dependency summary

The following arrows describe typical implementation dependencies, not strict crate requirements:

```text
runtime support
  -> nearly every domain

linear-algebra kernels
  -> dense linear algebra
  -> sparse methods
  -> nonlinear equations
  -> optimization
  -> differential equations
  -> interpolation/fitting

special functions
  -> probability/statistics
  -> selected quadrature and differential-equation applications

transforms
  -> FISHPACK / separable PDE solvers

nonlinear equations
  -> implicit ODE/DAE and nonlinear BVP workflows

optimization / least squares
  -> interpolation and regression

interpolation representations
  -> spline differentiation and integration
```

Cross-domain edges must ultimately be generated from source call graphs. The current map is conceptual and based on official purpose/provenance documentation.

## Raw versus safe organization

**Project proposal:**

- raw crates should tend to preserve package provenance and linkable dependency closures;
- safe crates should tend to follow mathematical user workflows;
- a facade can re-export functionality without erasing provenance;
- runtime support should be linked exactly once where possible;
- source-level metadata should retain GAMS codes, package IDs and dependency edges independently.

## Common FFI themes

Across domains, the recurring risks are:

- Fortran symbol and complex ABI variation;
- caller-sized work arrays;
- one-based indices;
- column-major and packed storage;
- callback trampolines;
- state preserved in work arrays or global error machinery;
- status codes whose meanings are routine-specific;
- input arrays overwritten with factors, coefficients or solver state;
- precision families that are irregular rather than generic.

Safe wrappers should allocate workspaces internally, validate dimensions, preserve raw status, model stateful solvers as objects and prevent Rust unwinding across Fortran.

## Functional overlaps

- Quadrature and interpolation share spline-integration routines.
- Probability distribution functions are largely special functions.
- Regression spans linear algebra, optimization, approximation and statistics.
- FISHPACK belongs functionally to PDEs but depends on FFTPACK transforms.
- DAE/implicit ODE solvers depend on nonlinear and linear system machinery.
- Sparse LP belongs to optimization while sharing sparse storage infrastructure.
- Error handling and machine constants are universal support, not mathematical domains.

## Evidence status

These surveys are representative rather than exhaustive. Exact routine catalogues, package boundaries and crate closures await automated ingestion of:

- the SLATEC source archive;
- table-of-contents records;
- source prologues;
- direct/transitive dependency resources;
- standalone package indexes and source comparisons.

## Open architectural questions

- Should some large safe domains be split by problem class or precision?
- Can raw package crates avoid duplicate symbols when linked together?
- Which strongly connected components force larger native libraries?
- Can callback support and error handling be made thread-safe?
- Which historical utilities should be replaced by Rust implementations versus wrapped directly?
- How should optional external BLAS/LAPACK backends coexist with SLATEC compatibility code?
- What minimum supported compiler/ABI matrix is realistic?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- Existing architecture, taxonomy and package-provenance pages in this repository.
