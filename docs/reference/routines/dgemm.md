# DGEMM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform one of the matrix-matrix operations.

## Description

DGEMM performs one of the matrix-matrix operations

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level3::dgemm, slatec::blas::level3::dgemm_contiguous`

## Providers

- Canonical provider: `lin/dgemm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dgemm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dgemm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DGEMM](https://www.netlib.org/slatec/lin/dgemm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `TRANSA` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( A ) = A. 'T' or 't',  op( A ) = A'. 'C' or 'c',  op( A ) = A'. Unchanged on exit. 'N' or 'n',  and is  m  otherwise. 'N' or 'n',  the leading  m by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then |
| 2 | `TRANSB` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( B ) = B. 'T' or 't',  op( B ) = B'. 'C' or 'c',  op( B ) = B'. Unchanged on exit. 'N' or 'n',  and is  k  otherwise. 'N' or 'n',  the leading  k by n part of the array  B  must contain the matrix  B,  otherwise 'N' or 'n' then |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry,  M  specifies  the number  of rows  of the  matrix must  be at least  zero. Unchanged on exit. contain  the matrix A. Unchanged on exit. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry,  N  specifies the number  of columns of the matrix must be at least zero. Unchanged on exit. 'N' or 'n',  and is  k  otherwise. contain  the matrix B. Unchanged on exit. |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry,  K  specifies  the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least  zero. Unchanged on exit. 'N' or 'n',  and is  m  otherwise. contain  the matrix A. Unchanged on exit. contain  the matrix B. Unchanged on exit. |
| 6 | `ALPHA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are scalars, and A, B and C are matrices, with op( A ) an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. Parameters ========== DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDA, *) | must  be at least  zero. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. |
| 8 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. |
| 9 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDB, *) | must be at least zero. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDB, kb ), where kb is contain  the matrix B. Unchanged on exit. |
| 10 | `LDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDB specifies the first dimension of B as declared must be at least  max( 1, k ), otherwise  LDB must be at least  max( 1, n ). Unchanged on exit. |
| 11 | `BETA` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | are scalars, and A, B and C are matrices, with op( A ) an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. Parameters ========== DOUBLE PRECISION. On entry,  BETA  specifies the scalar  beta.  When  BETA  is supplied as zero then C need not be set on input. Unchanged on exit. is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n  matrix ( alpha*op( A )*op( B ) + beta*C ). |
| 12 | `C` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDC, *) | = alpha*op( A )*op( B ) + beta*C, where  op( X ) is one of op( X ) = X   or   op( X ) = X', must  be at least  zero. Unchanged on exit. must be at least zero. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n  matrix ( alpha*op( A )*op( B ) + beta*C ). |
| 13 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. |

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

Canonical Rust path: `slatec_sys::blas::level3::dgemm`. Native symbol: `dgemm_`. Declaration feature: `blas-level3`. Provider feature: `blas-level3`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::dgemm`
- Public declaration feature: `blas-level3`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::blas::level3::dgemm`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
