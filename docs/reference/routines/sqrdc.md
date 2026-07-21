# SQRDC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Use Householder transformations to compute the QR factorization of an N by P matrix. Column pivoting is a users option.

## Description

SQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. Column pivoting based on the 2-norms of the reduced columns may be performed at the user's option. On Entry

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D5`
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

- Canonical provider: `lin/sqrdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sqrdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sqrdc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SQRDC](https://www.netlib.org/slatec/lin/sqrdc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDX, *) | REAL(LDX,P), where LDX .GE. N. contains the matrix whose decomposition is to be computed. is an initial column. is a free column. is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end.  Both initial and final columns are frozen in place during the computation and only free columns are moved.  At the K-th stage of the reduction, if X(K) is occupied by a free column, it is interchanged with the free column of largest reduced norm.  JPVT is not referenced if contains in its upper triangle the upper contains in its upper triangle the upper triangular matrix R of the QR factorization. triangular matrix R of the QR factorization. Below its diagonal X contains information from Below its diagonal X contains information from which the orthogonal part of the decomposition which the orthogonal part of the decomposition can be recovered.  Note that if pivoting has can be recovered.  Note that if pivoting has been requested, the decomposition is not that been requested, the decomposition is not that of the original matrix X but that of X of the original matrix X but that of X with its columns permuted as described by JPVT. with its columns permuted as described by JPVT. |
| 2 | `LDX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. is the leading dimension of the array X. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a users option. INTEGER. is the number of rows of the matrix X. |
| 4 | `P` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a users option. INTEGER. is the number of columns of the matrix X. |
| 5 | `QRAUX` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(P). contains further information required to recover the orthogonal part of the decomposition. |
| 6 | `JPVT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(P). contains integers that control the selection of the pivot columns.  The K-th column X(K) of X is placed in one of three classes according to the value of JPVT(K). is an initial column. is a free column. is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end.  Both initial and final columns are frozen in place during the computation and only free columns are moved.  At the K-th stage of the reduction, if X(K) is occupied by a free column, it is interchanged with the free column of largest reduced norm.  JPVT is not referenced if contains the index of the column of the contains the index of the column of the original matrix that has been interchanged into original matrix that has been interchanged into the K-th column, if pivoting was requested. the K-th column, if pivoting was requested. |
| 7 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(P). is a work array.  WORK is not referenced if |
| 8 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | .EQ. 0. .EQ. 0. INTEGER. is an integer that initiates column pivoting. If JOB .EQ. 0, no pivoting is done. If JOB .NE. 0, pivoting is done. On Return |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: REAL(P). is a work array.  WORK is not referenced if

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::sqrdc`. Native symbol: `sqrdc_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sqrdc`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
