# CGEMM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Multiply a complex general matrix by a complex general matrix.

## Description

CGEMM performs one of the matrix-matrix operations C := alpha*op( A )*op( B ) + beta*C, where op( X ) is one of op( X ) = X or op( X ) = X' or op( X ) = conjg( X' ), alpha and beta are scalars, and A, B and C are matrices, with op( A ) an m by k matrix, op( B ) a k by n matrix and C an m by n matrix. Parameters ========== TRANSA - CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: TRANSA = 'N' or 'n', op( A ) = A. TRANSA = 'T' or 't', op( A ) = A'. TRANSA = 'C' or 'c', op( A ) = conjg( A' ). Unchanged on exit. TRANSB - CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: TRANSB = 'N' or 'n', op( B ) = B. TRANSB = 'T' or 't', op( B ) = B'. TRANSB = 'C' or 'c', op( B ) = conjg( B' ). Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of the matrix op( A ) and of the matrix C. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix op( B ) and the number of columns of the matrix C. N must be at least zero. Unchanged on exit. K - INTEGER. On entry, K specifies the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least zero. Unchanged on exit. ALPHA - COMPLEX . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. A - COMPLEX array of DIMENSION ( LDA, ka ), where ka is k when TRANSA = 'N' or 'n', and is m otherwise. Before entry with TRANSA = 'N' or 'n', the leading m by k part of the array A must contain the matrix A, otherwise the leading k by m part of the array A must contain the matrix A. Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When TRANSA = 'N' or 'n' then LDA must be at least max( 1, m ), otherwise LDA must be at least max( 1, k ). Unchanged on exit. B - COMPLEX array of DIMENSION ( LDB, kb ), where kb is n when TRANSB = 'N' or 'n', and is k otherwise. Before entry with TRANSB = 'N' or 'n', the leading k by n part of the array B must contain the matrix B, otherwise the leading n by k part of the array B must contain the matrix B. Unchanged on exit. LDB - INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. When TRANSB = 'N' or 'n' then LDB must be at least max( 1, k ), otherwise LDB must be at least max( 1, n ). Unchanged on exit. BETA - COMPLEX . On entry, BETA specifies the scalar beta. When BETA is supplied as zero then C need not be set on input. Unchanged on exit. C - COMPLEX array of DIMENSION ( LDC, n ). Before entry, the leading m by n part of the array C must contain the matrix C, except when beta is zero, in which case C need not be set on entry. On exit, the array C is overwritten by the m by n matrix ( alpha*op( A )*op( B ) + beta*C ). LDC - INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, m ). Unchanged on exit.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
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

- Canonical provider: `lin/cgemm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cgemm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cgemm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CGEMM](https://www.netlib.org/slatec/lin/cgemm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `TRANSA` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `TRANSB` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `ALPHA` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `B` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDB, *) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `LDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `BETA` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `C` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDC, *) | Array argument classified by fixed-form executable read/write analysis. |
| 13 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

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

Canonical Rust path: `slatec_sys::blas::level3::cgemm`. Native symbol: `cgemm_`. Declaration feature: `blas-level3`. Provider feature: `blas-level3`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::cgemm`
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
