# SSMV

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP Column Format Sparse Matrix Vector Product. Routine to calculate the sparse matrix vector product: Y = A*X.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM REAL X(N), Y(N), A(NELT) CALL SSMV(N, X, Y, NELT, IA, JA, A, ISYM ) =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/ssmv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssmv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SSMV](https://www.netlib.org/slatec/lin/ssmv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Order of the Matrix. NELT+1, where  N  is the number of columns in  the |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | IN       Real X(N). The vector that should be multiplied by the matrix. |
| 3 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | A*X. OUT      Real Y(N). The product of the matrix and the vector. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of Non-Zeros stored in A. zeros in the matrix. Here is an example of the  SLAP Column  storage format for a |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer IA(NELT). zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer JA(NELT). th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| With  the SLAP  format  the "inner  loops" of  this  routine should vectorize   on machines with   hardware  support  for vector gather/scatter operations.  Your compiler may require |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | IN       Real A(NELT). These arrays should hold the matrix A in the SLAP Column format.  See "Description", below. zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 compiler directive  to  convince   it that there  are  no implicit vector  dependencies.  Compiler directives  for the Alliant FX/Fortran and CRI CFT/CFT77 compilers  are supplied with the standard SLAP distribution. Cautions: This   routine   assumes  that  the matrix A is stored in SLAP Column format.  It does not check  for  this (for  speed)  and evil, ugly, ornery and nasty things  will happen if the matrix data  structure  is,  in fact, not SLAP Column.  Beware of the wrong data structure!!! |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::ssmv`. Native symbol: `ssmv_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::ssmv`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
