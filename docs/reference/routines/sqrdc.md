# SQRDC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Use Householder transformations to compute the QR factorization of an N by P matrix. Column pivoting is a users option.

## Description

SQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. Column pivoting based on the 2-norms of the reduced columns may be performed at the user's option. On Entry X REAL(LDX,P), where LDX .GE. N. X contains the matrix whose decomposition is to be computed. LDX INTEGER. LDX is the leading dimension of the array X. N INTEGER. N is the number of rows of the matrix X. P INTEGER. P is the number of columns of the matrix X. JPVT INTEGER(P). JPVT contains integers that control the selection of the pivot columns. The K-th column X(K) of X is placed in one of three classes according to the value of JPVT(K). If JPVT(K) .GT. 0, then X(K) is an initial column. If JPVT(K) .EQ. 0, then X(K) is a free column. If JPVT(K) .LT. 0, then X(K) is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end. Both initial and final columns are frozen in place during the computation and only free columns are moved. At the K-th stage of the reduction, if X(K) is occupied by a free column, it is interchanged with the free column of largest reduced norm. JPVT is not referenced if JOB .EQ. 0. WORK REAL(P). WORK is a work array. WORK is not referenced if JOB .EQ. 0. JOB INTEGER. JOB is an integer that initiates column pivoting. If JOB .EQ. 0, no pivoting is done. If JOB .NE. 0, pivoting is done. On Return X X contains in its upper triangle the upper triangular matrix R of the QR factorization. Below its diagonal X contains information from which the orthogonal part of the decomposition can be recovered. Note that if pivoting has been requested, the decomposition is not that of the original matrix X but that of X with its columns permuted as described by JPVT. QRAUX REAL(P). QRAUX contains further information required to recover the orthogonal part of the decomposition. JPVT JPVT(K) contains the index of the column of the original matrix that has been interchanged into the K-th column, if pivoting was requested.

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

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDX, *) | SQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDX` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry X REAL(LDX,P), where LDX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `P` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QRAUX` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | QRAUX REAL(P). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JPVT` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | JPVT INTEGER(P). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK REAL(P). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JPVT is not referenced if JOB .EQ. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::sqrdc`. Native symbol: `sqrdc_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sqrdc`
- Compatibility aliases: `none`
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
