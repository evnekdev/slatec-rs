# SSYRK

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform symmetric rank k update of a real symmetric matrix.

## Description

SSYRK performs one of the symmetric rank k operations

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
- GAMS classifications: `D1B6`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level3::ssyrk`

## Providers

- Canonical provider: `lin/ssyrk.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssyrk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssyrk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SSYRK](https://www.netlib.org/slatec/lin/ssyrk.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `UPLO` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of the  array  C  is to be  referenced  as follows: 'U' or 'u'   Only the  upper triangular part of  C is to be referenced. 'L' or 'l'   Only the  lower triangular part of  C is to be referenced. Unchanged on exit. 'U' or 'u',  the leading  n by n upper triangular part of the array C must contain the upper triangular part  of the  symmetric matrix  and the strictly lower triangular part of C is not referenced.  On exit, the upper triangular part of the array  C is overwritten by the upper triangular part of the updated matrix. 'L' or 'l',  the leading  n by n lower triangular part of the array C must contain the lower triangular part  of the  symmetric matrix  and the strictly upper triangular part of C is not referenced.  On exit, the lower triangular part of the array  C is overwritten by the lower triangular part of the updated matrix. |
| 2 | `TRANS` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry,  TRANS  specifies the operation to be performed as follows: = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. 'N' or 'n',  K  specifies  the number of  columns   of  the   matrix   A,   and  on   entry   with 'T' or 't' or 'C' or 'c',  K  specifies  the  number 'N' or 'n',  and is  n  otherwise. 'N' or 'n',  the  leading  n by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then  LDA must be at least  max( 1, n ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = alpha*A*A' + beta*C. = alpha*A*A' + beta*C. INTEGER. must be must be at least zero. at least zero. Unchanged on exit. Unchanged on exit. contain  the matrix A. Unchanged on exit. |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. must be at least zero. Unchanged on exit. 'N' or 'n',  and is  n  otherwise. contain  the matrix A. Unchanged on exit. |
| 5 | `ALPHA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. |
| 6 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDA, *) | must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. |
| 7 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDA specifies the first dimension of A as declared |
| 8 | `BETA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== REAL            . On entry, BETA specifies the scalar beta. Unchanged on exit. |
| 9 | `C` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDC, *) | = alpha*A*A' + beta*C, or = alpha*A'*A + beta*C, is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. Unchanged on exit. Unchanged on exit. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDC, n ). |
| 10 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, n ). Unchanged on exit. |

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

Canonical Rust path: `slatec_sys::blas::level3::ssyrk`. Native symbol: `ssyrk_`. Declaration feature: `blas-level3`. Provider feature: `blas-level3`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::ssyrk`
- Public declaration feature: `blas-level3`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::blas::level3::ssyrk`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
