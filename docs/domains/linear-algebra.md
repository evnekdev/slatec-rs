# Dense and structured linear algebra

## Scope

This survey covers SLATEC routines for vector and matrix kernels, dense and banded linear systems, symmetric/Hermitian and positive-definite systems, triangular and tridiagonal systems, least-squares factorizations, singular values, eigenproblems, condition estimation, inversion and factor updates. Sparse iterative methods are treated separately in [Sparse methods](sparse-methods.md).

The authoritative SLATEC Version 4.1 table of contents places elementary operations under GAMS `D1`, linear systems and related decompositions under `D2`, eigenvalue problems in later `D` branches, and identifies user-callable versus subsidiary routines ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Problem classes

SLATEC covers at least the following structured classes:

- general dense real and complex matrices;
- general band matrices;
- triangular matrices in full, band and packed storage;
- real symmetric and complex Hermitian matrices;
- symmetric/Hermitian positive-definite matrices;
- packed and banded symmetric/Hermitian storage;
- tridiagonal systems;
- rectangular least-squares and minimum-norm problems;
- singular-value decomposition;
- real and complex eigenvalue/eigenvector problems;
- rank-one updates, downdates and condition estimation.

These classes are visible in the `D1` and `D2` table-of-contents hierarchy and in the relocated `slatec/lin` distribution subset ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`slatec-lin`](https://www.netlib.org/slatec/lin/)).

## Major routine families

| Family | Representative routines | Role | Provenance | Precision/type pattern |
|---|---|---|---|---|
| Level-1 kernels | `SAXPY/DAXPY/CAXPY`, `SDOT/DDOT`, `SNRM2/DNRM2/SCNRM2`, `SSCAL/DSCAL/CSCAL` | vector operations | BLAS | single, double and complex variants, not always perfectly symmetric |
| Level-2/3 kernels | `SGEMV/DGEMV/CGEMV`, `SGEMM/DGEMM/CGEMM`, triangular and symmetric/Hermitian operations | matrix-vector and matrix-matrix operations | BLAS | real/complex and single/double families |
| General LU | `SGEFA/DGEFA/CGEFA`, `SGESL/DGESL/CGESL`, `*GECO`, `*GEDI` | factor, solve, estimate condition, determinant/inverse | LINPACK | predominantly S/D/C |
| Banded LU | `*GBFA`, `*GBSL`, `*GBCO` | banded general systems | LINPACK | S/D/C |
| Symmetric/Hermitian indefinite | `*SIFA`, `*SISL`, `*SICO`, packed analogues | pivoted symmetric/Hermitian factorization and solve | LINPACK | real symmetric and complex Hermitian variants |
| Positive definite | `*POFA`, `*POSL`, `*POCO`, packed/banded analogues | Cholesky-family operations | LINPACK | S/D/C |
| QR and least squares | `*QRDC`, `*QRSL`, `HFTI`, `LSEI`, `WNNLS` | QR, constrained and unconstrained least squares | LINPACK and other incorporated families | mostly S/D/C for LINPACK; S/D for higher drivers |
| SVD | `SSVDC/DSVDC/CSVDC`, subsidiary `SVD` | singular values and vectors | LINPACK / incorporated support | S/D/C for `*SVDC` |
| Eigenproblems | `BALANC`, `HQR`, `TQL*`, `RG`, `RS`, `CG` and helpers | reductions, iterations, back-transformations and drivers | EISPACK | routines often use historical EISPACK naming rather than regular precision prefixes |
| Factor updates | `*CHUD`, `*CHDD`, `*CHEX`, `*QRDC`, rotation helpers | update/downdate and transformed systems | LINPACK | S/D/C where supplied |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`netlib-blas`](https://www.netlib.org/blas/), [`netlib-linpack`](https://www.netlib.org/linpack/), [`netlib-eispack`](https://www.netlib.org/eispack/).

This is representative, not exhaustive. The table of contents includes integer copies, mixed-precision accumulation routines and SLATEC-specific drivers that do not fit a pure BLAS/LINPACK/EISPACK partition.

## User-facing and internal structure

The table of contents distinguishes user-callable routines from subsidiary routines. Typical user-facing patterns include:

1. **factor then solve**, such as `DGEFA` followed by one or more calls to `DGESL`;
2. **combined driver**, such as `DGEFS`, which solves and may provide error information;
3. **condition-estimating factorization**, such as `DGECO`;
4. **post-factor operations**, such as determinant/inverse or projection routines;
5. **pipeline drivers**, especially in EISPACK, which invoke reductions, eigensolvers and back-transformations.

Subsidiary routines include low-level transformations, norm calculations, sorting, scaling and implementation helpers. They should not automatically become public safe-Rust APIs merely because their symbols are linkable ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Storage and calling conventions

### Column-major arrays and leading dimensions

Fortran matrices are column-major. Two-dimensional arguments commonly carry a leading dimension such as `LDA`, `LDFJAC` or package-specific equivalents. A Rust FFI layer must preserve the physical leading dimension; it cannot infer that a matrix is tightly packed merely from logical row and column counts.

### Structured storage

Banded, packed symmetric/Hermitian and tridiagonal routines use specialized layouts. The same logical matrix can therefore have multiple incompatible representations. Safe wrappers should use distinct view types rather than a shared untyped slice.

### Destructive factorization

Many factorization routines overwrite the matrix with factors and return pivot information in an integer array. Solve routines consume this mutated representation. The safe API should model a factorization object rather than exposing an ordinary matrix after successful factorization.

### Work and pivot arrays

Historical routines often require caller-allocated real, complex or integer work arrays. Some drivers partition a single workspace internally; others expose several work arrays. Exact minimum lengths are routine-specific and must come from the source prologue.

## Algorithm families and assumptions

LINPACK routines use unblocked algorithms designed before modern cache hierarchies. LAPACK was later designed to reorganize dense computations around Level-3 BLAS and modern memory hierarchies; its official documentation explicitly describes LINPACK and EISPACK as predecessor libraries whose memory-access patterns were inefficient on later machines ([LAPACK project](https://www.netlib.org/lapack/)).

**Modern interpretation:** SLATEC’s linear algebra remains valuable for historical compatibility, specialized algorithms, compact dependency closures and reproducing legacy numerical behavior. It should not be presented as a performance replacement for a tuned current BLAS/LAPACK implementation.

EISPACK routines often form algorithm pipelines rather than one-call modern drivers. Users may need to select reductions based on matrix structure and request eigenvectors explicitly.

## Important dependencies

Typical dependencies include:

- BLAS kernels;
- machine constants and scaling helpers;
- error handling such as `XERMSG` or `XERBLA`;
- sorting, rotations and norm helpers;
- package-specific subsidiary routines.

A source-derived dependency graph is required before extracting a routine family into a static library. Merely linking a user-callable driver without its subsidiary closure is insufficient.

## Limitations

- No uniform workspace-query convention comparable to modern LAPACK is assumed.
- Factorization and solve status conventions differ across families.
- Some routines compute determinants or inverses in scaled historical formats.
- Integer widths and Fortran logical/character conventions are compiler-ABI concerns.
- Complex ABI compatibility must be tested per compiler and target.
- Mixed-origin routines may follow different prologue and error conventions.
- Unblocked algorithms may be slow for large modern dense matrices.
- The table of contents documents mathematical purpose, not every aliasing or workspace constraint.

## Relation to modern libraries

LAPACK covers dense and banded systems, least squares, eigenproblems, SVD and related factorizations in real/complex single/double precision, and delegates much work to BLAS. It does not cover general sparse matrices ([LAPACK project](https://www.netlib.org/lapack/)). Current high-level environments usually expose result objects, workspace allocation and structured matrix types rather than raw Fortran work arrays.

**Project interpretation:** safe SLATEC wrappers should use modern API ergonomics while preserving algorithm identity and status details. They should not silently redirect a SLATEC routine to LAPACK, because that would change provenance and potentially numerical behavior.

## Preliminary FFI risks

| Risk | Why it matters | Proposed mitigation |
|---|---|---|
| symbol naming and compiler ABI | Fortran symbol spelling and hidden conventions vary | generated target-specific symbol checks and ABI smoke tests |
| column-major/leading dimension | incorrect strides cause memory corruption | typed matrix views with validated dimensions |
| packed/banded formats | logical shape does not determine physical length | separate storage types with checked constructors |
| overwritten inputs | factorization destroys matrix contents | ownership-taking or explicit mutable APIs |
| pivot/index base | Fortran indices are one-based | convert only in safe layer; retain raw integers in `-sys` |
| complex representation | Fortran complex ABI can vary | compile-and-run ABI probes; avoid assumptions |
| work arrays | undersized workspaces corrupt memory | calculate lengths from verified formulas and allocate internally |
| status and error globals | routines may use both return flags and SLATEC error state | capture both channels and document process-global behavior |
| integer overflow in dimensions | workspace formulas can overflow | checked arithmetic before FFI calls |

## Project proposals for safe Rust APIs

### Kernel layer

Expose BLAS-like operations through typed slices/matrix views, with explicit transpose and conjugation enums. Existing Rust BLAS ecosystems may be preferred for performance; the SLATEC implementation should be selectable when compatibility is required.

### Factorization objects

Candidate types:

```text
LuFactor<T>
CholeskyFactor<T>
SymmetricIndefiniteFactor<T>
QrFactor<T>
Svd<T>
EigenDecomposition<T>
```

Construction would consume or mutably borrow storage and preserve pivots/factors. Methods would expose solve, condition estimate and related operations only when supported by the underlying family.

### Structured views

Candidate checked views:

```text
BandMatrixView
PackedSymmetricView
PackedHermitianView
TriangularView
TridiagonalView
```

These are project proposals, not SLATEC types.

## Candidate crate boundaries

**Raw crates, tentative:**

- `slatec-blas-sys`
- `slatec-linpack-sys`
- `slatec-eispack-sys`
- a small `slatec-linear-support-sys` only if dependency analysis justifies it

**Safe crates, tentative:**

- `slatec-linalg-kernels`
- `slatec-linalg`
- optional provenance modules or feature flags for LINPACK/EISPACK algorithms

A package-based raw split preserves upstream lineage, while a domain-based safe crate gives users a coherent API. Duplicate symbols and cross-package dependencies must be resolved before this split is accepted.

## Open questions and required experiments

- Which exact BLAS/LINPACK/EISPACK revisions occur in SLATEC 4.1?
- Which symbols collide with separately linked system BLAS or LAPACK?
- Are all complex routine ABIs compatible across gfortran, Intel and other supported compilers?
- Which routines retain process-global error state?
- What minimum matrix and workspace lengths are safe for every structured layout?
- Which EISPACK driver pipelines deserve public wrappers?
- Should safe wrappers expose historical determinant/inverse formats or normalize them?
- How close are results to current reference LAPACK for representative conditioned and ill-conditioned cases?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-lin`](https://www.netlib.org/slatec/lin/)
- [`netlib-blas`](https://www.netlib.org/blas/)
- [`netlib-linpack`](https://www.netlib.org/linpack/)
- [`netlib-eispack`](https://www.netlib.org/eispack/)
- [LAPACK project](https://www.netlib.org/lapack/)
