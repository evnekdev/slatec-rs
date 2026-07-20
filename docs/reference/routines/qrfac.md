# QRFAC

[Family: Shared numerical utilities](../families/shared-numerical-utilities.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1, SNLS1E, SNSQ and SNSQE

## Description

This subroutine uses Householder transformations with column pivoting (optional) to compute a QR factorization of the M by N matrix A. That is, QRFAC determines an orthogonal matrix Q, a permutation matrix P, and an upper trapezoidal matrix R with diagonal elements of nonincreasing magnitude, such that A*P = Q*R. The Householder transformation for column K, K = 1,2,...,MIN(M,N), is of the form T I - (1/U(K))*U*U where U has zeros in the first K-1 positions. The form of this transformation and the method of pivoting first appeared in the corresponding LINPACK subroutine. The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. N is a positive integer input variable set to the number of columns of A. A is an M by N array. On input A contains the matrix for which the QR factorization is to be computed. On output the strict upper trapezoidal part of A contains the strict upper trapezoidal part of R, and the lower trapezoidal part of A contains a factored form of Q (the non-trivial elements of the U vectors described above). LDA is a positive integer input variable not less than M which specifies the leading dimension of the array A. PIVOT is a logical input variable. If pivot is set .TRUE., then column pivoting is enforced. If pivot is set .FALSE., then no column pivoting is done. IPVT is an integer output array of length LIPVT. IPVT defines the permutation matrix P such that A*P = Q*R. Column J of P is column IPVT(J) of the identity matrix. If pivot is .FALSE., IPVT is not referenced. LIPVT is a positive integer input variable. If PIVOT is .FALSE., then LIPVT may be as small as 1. If PIVOT is .TRUE., then LIPVT must be at least N. SIGMA is an output array of length N which contains the diagonal elements of R. ACNORM is an output array of length N which contains the norms of the corresponding columns of the input matrix A. If this information is not needed, then ACNORM can coincide with SIGMA. WA is a work array of length N. If pivot is .FALSE., then WA can coincide with SIGMA.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `multiple-parent-families`
- Secondary families: `Approximation, Nonlinear equations`
- Family evidence: `parent_inheritance` (`medium`)
- Parent-family evidence: `SNLS1, SNLS1E, SNSQ, SNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qrfac.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qrfac.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qrfac.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qrfac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Shared numerical utilities](../families/shared-numerical-utilities.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine uses Householder transformations with column pivoting (optional) to compute a QR factorization of the M by N matrix A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | This subroutine uses Householder transformations with column pivoting (optional) to compute a QR factorization of the M by N matrix A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDA, *) | This subroutine uses Householder transformations with column pivoting (optional) to compute a QR factorization of the M by N matrix A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PIVOT` | input | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPVT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIPVT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SIGMA` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ACNORM` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-logical`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
