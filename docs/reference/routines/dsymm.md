# DSYMM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform one of the matrix-matrix operations.

## Description

DSYMM performs one of the matrix-matrix operations

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B6`
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

- Canonical provider: `lin/dsymm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsymm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dsymm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DSYMM](https://www.netlib.org/slatec/lin/dsymm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `SIDE` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry,  SIDE  specifies whether  the  symmetric matrix  A appears on the  left or right  in the  operation as follows: = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. 'L' or 'l'  and is  n otherwise. 'L' or 'l',  the  m by m  part of the array  A  must contain the  symmetric matrix,  such that 'R' or 'r',  the  n by n  part of the array  A  must contain the  symmetric matrix,  such that 'L' or 'l'  then |
| 2 | `UPLO` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of  the  symmetric  matrix   A  is  to  be referenced as follows: 'U' or 'u'   Only the upper triangular part of the symmetric matrix is to be referenced. 'L' or 'l'   Only the lower triangular part of the symmetric matrix is to be referenced. Unchanged on exit. 'U' or 'u', the leading m by m upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'U' or 'u', the leading n by n upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry,  M  specifies the number of rows of the matrix  C. must be at least zero. Unchanged on exit. 'L' or 'l'  and is  n otherwise. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, N specifies the number of columns of the matrix C. must be at least zero. Unchanged on exit. |
| 5 | `ALPHA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a symmetric matrix and  B and DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. |
| 6 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDA, *) | is a symmetric matrix and  B and DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. |
| 7 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, n ). Unchanged on exit. |
| 8 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDB, *) | DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry, the leading  m by n part of the array  B  must contain the matrix B. Unchanged on exit. |
| 9 | `LDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. |
| 10 | `BETA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a symmetric matrix and  B and DOUBLE PRECISION. On entry,  BETA  specifies the scalar  beta.  When  BETA  is supplied as zero then C need not be set on input. Unchanged on exit. is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n updated matrix. |
| 11 | `C` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDC, *) | = alpha*A*B + beta*C, or = alpha*B*A + beta*C, are  m by n matrices. Parameters ========== = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n updated matrix. |
| 12 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. |

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

Canonical Rust path: `slatec_sys::blas::level3::dsymm`. Native symbol: `dsymm_`. Declaration feature: `blas-level3`. Provider feature: `blas-level3`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::dsymm`
- Public declaration feature: `blas-level3`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
