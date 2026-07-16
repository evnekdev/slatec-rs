# Preliminary GAMS-to-domain map

## Scope

This is a project mapping from historical GAMS classes to proposed stable `slatec-rs` documentation and crate-planning domains. It is deliberately many-to-many and preliminary. The authoritative historical classifications remain those in the SLATEC table of contents and routine prologues ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Mapping principles

1. Preserve each original GAMS code verbatim.
2. Map leaf codes to one or more project domains.
3. Do not derive package provenance from GAMS.
4. Keep support dependencies separate from mathematical ownership.
5. Permit cross-domain routines and facade-level re-exports.
6. Revise mappings after complete routine and dependency extraction.

## Proposed map

| GAMS branch | Historical subject | Primary project domain ID | Possible secondary domain | Notes |
|---|---|---|---|---|
| `A` | arithmetic, error analysis | `numeric-support` | `special-functions` | extended-range arithmetic can support special functions |
| `C` | elementary and special functions | `special-functions` | `probability-statistics` | probability distributions may call special functions |
| `D1` | vector/matrix operations | `linear-algebra-kernels` | `sparse-linear-algebra` | dense BLAS and sparse kernels must be distinguished |
| `D2-D9` | factorization, solve, eigen, least squares, updates | `dense-linear-algebra` | `sparse-linear-algebra` | storage and algorithm family determine final boundary |
| sparse `D` branches | sparse systems and iterative methods | `sparse-linear-algebra` | `linear-algebra-kernels` | SLAP uses low-level vector operations |
| `E` | interpolation | `interpolation` | `approximation` | PCHIP is a provenance subset, not all interpolation |
| `F` | nonlinear equations | `nonlinear-equations` | `optimization` | nonlinear least squares may straddle both |
| `G` | optimization | `optimization` | `nonlinear-equations` | root/least-squares dependencies can cross |
| `H` | differentiation and integration | `quadrature` | `numeric-support` | numerical differentiation may later become a separate subdomain |
| `I` ODE/DAE | initial-value differential equations | `ode-dae` | `nonlinear-equations`, `dense-linear-algebra` | implicit solvers rely on nonlinear and linear solves |
| `I` BVP | boundary-value problems | `boundary-value-problems` | `ode-dae`, `linear-algebra-kernels` | exact GAMS leaves require ingestion |
| `I` PDE/integral equations | PDE-related and integral equations | `pde-integral-equations` | `transforms`, `dense-linear-algebra` | FISHPACK belongs here by function |
| `J` | integral transforms | `transforms` | `numeric-support` | FFTPACK is a package within this domain |
| `K` | approximation | `approximation` | `interpolation`, `optimization` | curve fitting may depend on least squares |
| `L` | statistics and probability | `probability-statistics` | `special-functions` | distribution functions often use gamma/beta/error functions |
| `N` | data handling | `data-utilities` | `numeric-support` | sorting/index utilities may be shared |
| `R` | service routines | `runtime-support` | all | error handling and machine constants are cross-cutting |
| `Z` | other | `uncategorized` | any | temporary classification pending routine review |

## Tentative dependency layers

The metadata uses these conceptual layers:

- **Layer 0 — runtime foundation:** machine constants, error handling and basic data utilities.
- **Layer 1 — numerical kernels:** scalar support, BLAS-like operations and transform primitives.
- **Layer 2 — numerical methods:** dense/sparse solvers, special functions, interpolation, quadrature, nonlinear equations and optimization.
- **Layer 3 — systems and composite solvers:** ODE/DAE, BVP, PDE and higher workflows.
- **Layer 4 — facade:** user-oriented aggregation; not defined by this prompt.

These are `slatec-rs` proposals, not historical SLATEC architecture.

## Cross-cutting examples

- QUADPACK belongs historically to package `quadpack` and functionally to `quadrature`; it may depend on machine constants and error infrastructure.
- FFTPACK belongs historically to package `fftpack` and functionally to `transforms`; its Netlib co-location with FISHPACK does not alter that.
- PCHIP belongs historically to package `pchip` and functionally to `interpolation`.
- BLAS belongs historically to package `blas` and functionally to `linear-algebra-kernels`; it also forms a dependency layer for higher linear algebra.
- SLAP belongs historically to package `slap` and functionally to `sparse-linear-algebra`, while depending on kernels and runtime support.

## Open mapping issues

- Exact GAMS leaves for ODE, DAE, BVP, PDE and integral-equation families must be generated from the complete catalogue.
- Statistics and probability may warrant separate domains once routine counts and dependencies are known.
- Numerical differentiation may belong with quadrature or numeric support.
- Approximation, interpolation, nonlinear least squares and optimization have genuine overlaps.
- Special functions may need subdomains for elementary, orthogonal-polynomial, Bessel/Airy, elliptic and combinatorial functions.
- Package-level crates and function-level crates may coexist; the preferred public architecture is not yet decided.

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`gams-classification`](https://gams.nist.gov/)
- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
