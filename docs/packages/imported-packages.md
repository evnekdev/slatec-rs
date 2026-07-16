# Imported and incorporated packages

## Scope

This catalogue summarizes major packages visibly represented in SLATEC. “Imported” does not necessarily mean byte-for-byte inclusion; exact revision and modification status must be established by source comparison.

## Package catalogue

| Package | Functional role | Evidence in SLATEC | Boundary confidence | Principal authoritative sources | Key uncertainty |
|---|---|---|---|---|---|
| BLAS | vector and matrix kernels | Netlib says `slatec/lin` contains the BLAS subset; standard routines appear there | high | [`netlib-slatec-index`](https://www.netlib.org/slatec/), [`slatec-lin`](https://www.netlib.org/slatec/lin/), [`netlib-blas`](https://www.netlib.org/blas/) | exact BLAS generation and SLATEC-local variants |
| LINPACK | dense/banded factorizations, solves, condition estimates, QR/SVD and updates | explicitly named in `slatec/lin` | high | [`slatec-lin`](https://www.netlib.org/slatec/lin/), [`netlib-linpack`](https://www.netlib.org/linpack/) | exact source revision and local changes |
| EISPACK | eigenvalue/eigenvector pipelines | explicitly named in `slatec/lin`; recognizable EISPACK routines and authors | high | [`slatec-lin`](https://www.netlib.org/slatec/lin/), [`netlib-eispack`](https://www.netlib.org/eispack/) | pipeline membership and modified copies |
| SLAP | iterative sparse linear-system solvers and storage utilities | explicitly named in `slatec/lin`; routines describe SLAP formats and solvers | high | [`slatec-lin`](https://www.netlib.org/slatec/lin/), [`netlib-slap`](https://www.netlib.org/slap/) | boundary with generic sparse and BLAS-like helpers |
| FFTPACK | Fourier, sine and cosine transforms | explicitly included in `slatec/fishfft` | high | [`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/), [`netlib-fftpack`](https://www.netlib.org/fftpack/) | exact FFTPACK revision |
| FISHPACK | separable elliptic PDE solvers | explicitly included in `slatec/fishfft` | high | [`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/), [`netlib-fishpack`](https://www.netlib.org/fishpack/) | split from shared directory and dependencies on transforms |
| QUADPACK | one-dimensional automatic quadrature | routine families and canonical Netlib package align with SLATEC integration categories | high | [`netlib-quadpack`](https://www.netlib.org/quadpack/), [`quadpack-manual`](https://doi.org/10.1007/978-3-642-61786-7), [`slatec-toc`](https://www.netlib.org/slatec/toc) | exact incorporated set and revisions |
| FNLIB | elementary and special functions | explicitly identified as `slatec/fnlib` | verified at collection level | [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/), [`netlib-slatec-index`](https://www.netlib.org/slatec/) | Netlib states relationship to `/fn` is unresolved |
| PCHIP | piecewise cubic Hermite interpolation | explicitly identified as `slatec/pchip` | verified at collection level | [`slatec-pchip`](https://www.netlib.org/slatec/pchip/), [`netlib-slatec-index`](https://www.netlib.org/slatec/) | exact upstream release and local edits |
| error package | historical `XERROR`/`XERMSG` support | Netlib provides `slatec/err` for old SLATEC error routines used by other packages | high as support collection | [`netlib-slatec-index`](https://www.netlib.org/slatec/), SLATEC Guide | relationship between old and 4.1 error subsystems |
| machine constants | `I1MACH`, `R1MACH`, `D1MACH` and machine hooks | official SLATEC index describes machine-specific support | verified as subsystem | [`netlib-slatec-index`](https://www.netlib.org/slatec/), SLATEC Guide | not a standalone mathematical package |

## Special-function provenance

The GAMS `C` category contains several historical strands:

- FNLIB elementary and special functions;
- complex Bessel and Airy sequence routines;
- orthogonal-polynomial and angular-momentum functions;
- Carlson elliptic integrals;
- functions connected to probability and statistics;
- individually attributed routines.

Therefore, assigning all special functions to `fnlib` would be incorrect. Each routine needs prologue authorship, references and source-history review ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/)).

## Interpolation and spline provenance

PCHIP is an identifiable package, but GAMS `E` includes interpolation outside PCHIP. Spline, smoothing, polynomial and multidimensional routines must remain unassigned or be mapped to separately verified families until their source records are inspected ([`slatec-pchip`](https://www.netlib.org/slatec/pchip/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

## ODE, BVP and DAE provenance

SLATEC’s differential-equation holdings include multiple solver families rather than one universal package. Candidate families should be identified from driver names, prologue references and original reports. Until that work is complete, `metadata/packages.toml` records broad candidate collections with medium or low confidence rather than exact routine lists.

## Optimization and nonlinear equations

Similar caution applies to optimization and nonlinear-equation routines. Shared least-squares, linear-algebra and error-handling dependencies do not prove common package ownership. Function-based domains may be established before package lineage is fully resolved.

## Sparse methods

SLAP is the clearest named sparse package, but sparse storage, matrix-vector operations and direct sparse methods may include routines outside SLAP. A source-derived membership list is required before a package boundary is used for building or licensing.

## Statistics and utilities

The GAMS `L`, `N` and `R` categories are functional classifications. They do not by themselves identify packages. Statistical distributions often depend on special functions, while service routines are shared by many packages.

## Unresolved verification work

1. Download and checksum the complete SLATEC archive and each upstream package snapshot.
2. Generate normalized Fortran diffs with comments and fixed-form layout handled carefully.
3. Compare routine prologue authors, dates and revision histories.
4. Reconcile package indexes with the SLATEC table of contents.
5. Identify renamed and generated precision variants.
6. Record dependency edges crossing candidate package boundaries.
7. Review file-level notices and original documentation rights.

## Sources

All package URLs above are authoritative Netlib indexes or original package documentation registered in `metadata/sources.toml`.
