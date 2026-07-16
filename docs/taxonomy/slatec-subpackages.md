# SLATEC subpackages and collections

## Scope

SLATEC is both a common library and an aggregation of identifiable packages, smaller collections and individually contributed routines. This page records package-level structure only where official distribution records, source metadata or original package documentation support it.

## Distribution-level subsets identified by Netlib

The current Netlib SLATEC index says that several subsets were removed from the main `slatec/src` directory to make it more accessible:

| Netlib SLATEC subset | Contents stated by Netlib | Related upstream collection | Boundary confidence |
|---|---|---|---|
| `slatec/lin` | BLAS, LINPACK, EISPACK and SLAP subset | `/blas`, `/linpack`, `/eispack`, `/slap` | high for the combined directory; routine-level ownership still requires verification |
| `slatec/fishfft` | FISHPACK and FFTPACK subset | `/fishpack`, `/fftpack` | high for the combined directory; exact split requires routine comparison |
| `slatec/fnlib` | FNLIB subset | `/fn` described as another version | high that it is called FNLIB; uncertain lineage/version relationship |
| `slatec/pchip` | piecewise cubic Hermite approximation | `/pchip` | high at collection level |

Source: [`netlib-slatec-index`](https://www.netlib.org/slatec/).

The index explicitly states that Netlib did not know which of `slatec/fnlib` or `/fn` was more recent or bug-free. That is direct evidence that public directory proximity is not enough to infer a clean upstream/downstream relationship ([`netlib-slatec-index`](https://www.netlib.org/slatec/)).

## Major identifiable collections

### BLAS

The Basic Linear Algebra Subprograms supply vector and matrix kernels. SLATEC’s linear subset contains routines identifiable as BLAS by standard names and by the official BLAS collection, including AXPY, COPY, DOT, NRM2, SCAL, SWAP, GEMV, GEMM and triangular operations. The SLATEC table of contents classifies these by mathematical operation under `D1`, not by the package label “BLAS” ([`netlib-blas`](https://www.netlib.org/blas/); [`slatec-toc`](https://www.netlib.org/slatec/toc); [`slatec-lin`](https://www.netlib.org/slatec/lin/)).

**Boundary warning:** SLATEC’s linear subset also contains non-BLAS helpers, integer analogues, extended-accumulation operations and sparse kernels.

### LINPACK

LINPACK is represented by dense and banded system solvers, decompositions, condition estimation, determinant/inverse routines, QR operations, SVD and update/downdate routines. Standard families include `*GEFA/*GESL`, `*POFA/*POSL`, `*QRDC/*QRSL` and `*SVDC`, but exact membership must be checked against the canonical LINPACK index and source provenance ([`netlib-linpack`](https://www.netlib.org/linpack/); [`slatec-lin`](https://www.netlib.org/slatec/lin/)).

### EISPACK

EISPACK contributes eigenvalue and eigenvector pipelines for real and complex matrix classes, including balancing, reductions, eigensolvers and back-transformation routines. Names such as `BALANC`, `HQR`, `TQL*`, `RG`, `RS`, `CG` and associated helpers appear in the SLATEC linear subset, but pipeline membership and revision identity require source comparison ([`netlib-eispack`](https://www.netlib.org/eispack/); [`slatec-lin`](https://www.netlib.org/slatec/lin/)).

### SLAP

The SLATEC Linear Algebra Package (SLAP) contains iterative sparse linear-system methods, sparse storage conversion and matrix-vector kernels. Netlib’s SLATEC index includes SLAP in `slatec/lin`, and the linear subset labels routines such as `DBCG`, `DSMV`, `DSMTV` and `DS2Y` as SLAP operations ([`netlib-slatec-index`](https://www.netlib.org/slatec/); [`slatec-lin`](https://www.netlib.org/slatec/lin/); [`netlib-slap`](https://www.netlib.org/slap/)).

### FISHPACK and FFTPACK

Netlib groups these in `slatec/fishfft`, but they address different domains. FISHPACK solves separable elliptic partial differential equations and related problems; FFTPACK provides real, complex, sine, cosine and quarter-wave transforms ([`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/); [`netlib-fishpack`](https://www.netlib.org/fishpack/); [`netlib-fftpack`](https://www.netlib.org/fftpack/)).

**Boundary warning:** sharing a relocated Netlib directory is a distribution convenience, not a proposal that FFT and elliptic PDE APIs belong in one Rust crate.

### QUADPACK

QUADPACK supplies adaptive and non-adaptive one-dimensional quadrature families, including general-purpose, weighted, singular, oscillatory and improper-integral drivers. Its routines occupy integration branches under GAMS `H`, while historical provenance is identified by the QUADPACK package and original manual ([`netlib-quadpack`](https://www.netlib.org/quadpack/); [`quadpack-manual`](https://doi.org/10.1007/978-3-642-61786-7); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

### FNLIB and other special-function collections

`slatec/fnlib` is explicitly identified as FNLIB. It includes many elementary and special functions evaluated through Chebyshev expansions and supporting utilities. SLATEC also contains special-function families with different provenance, including complex Bessel/Airy sequences associated with Amos algorithms and other individually attributed routines. “Special functions” is therefore a functional domain, not a single package ([`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

### PCHIP and interpolation collections

The relocated PCHIP subset covers piecewise cubic Hermite interpolation. SLATEC’s broader interpolation category also includes polynomial, spline, smoothing and multidimensional interpolation routines that are not automatically part of PCHIP ([`slatec-pchip`](https://www.netlib.org/slatec/pchip/); [`slatec-toc`](https://www.netlib.org/slatec/toc)).

### Differential-equation collections

SLATEC includes ODE, boundary-value and differential-algebraic equation families, but not all are one package. Historically recognizable families include variants of DEPAC/DEBDF, DASSL, boundary-value solvers and specialized PDE-related collections. Routine-level package claims require source prologues and original reports, so this page does not assign exact membership prematurely ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`slatec-guide`](https://www.netlib.org/slatec/guide)).

### Optimization and nonlinear-equation collections

The GAMS `F` and `G` branches include scalar root finding, nonlinear systems, least squares and constrained/unconstrained optimization. Some families are traceable to named algorithms or reports; others are individual SLATEC contributions. Function-based grouping is safer at this stage than asserting a single package boundary ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Original and unattributed SLATEC content

The absence of a package label does not mean a routine was authored by a central SLATEC team. SLATEC accepted contributions from participating institutions and incorporated software from multiple origins. Exact provenance can be:

1. named standalone package;
2. smaller routine family associated with a paper or author;
3. SLATEC-adapted version of external software;
4. independently submitted routine;
5. support infrastructure created or standardized for the common library;
6. unknown or conflicting.

**Project policy:** use `provenance_kind = "package" | "family" | "individual" | "support" | "unknown"` rather than forcing every routine into a package.

## Unresolved questions

- Exact routine inventories for each incorporated package have not yet been diffed against upstream archives.
- SLATEC-specific modifications, renamed routines and precision conversions remain to be catalogued.
- FNLIB versus Netlib `fn` chronology is explicitly unresolved by the Netlib index.
- The boundaries among SLAP, generic sparse utilities and BLAS-like sparse kernels require routine-level analysis.
- ODE/BVP/DAE and optimization family names require confirmation from source prologues and original publications.

## Sources

- [`netlib-slatec-index`](https://www.netlib.org/slatec/)
- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-lin`](https://www.netlib.org/slatec/lin/)
- [`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/)
- [`slatec-fnlib`](https://www.netlib.org/slatec/fnlib/)
- [`slatec-pchip`](https://www.netlib.org/slatec/pchip/)
- Official Netlib package indexes linked above.
