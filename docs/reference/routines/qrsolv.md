# QRSOLV

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1 and SNLS1E

## Description

Given an M by N matrix A, an N by N diagonal matrix D, and an M-vector B, the problem is to determine an X which solves the system A*X = B , D*X = 0 , in the least squares sense. This subroutine completes the solution of the problem if it is provided with the necessary information from the QR factorization, with column pivoting, of A. That is, if A*P = Q*R, where P is a permutation matrix, Q has orthogonal columns, and R is an upper triangular matrix with diagonal elements of nonincreasing magnitude, then QRSOLV expects the full upper triangle of R, the permutation matrix P, and the first N components of (Q TRANSPOSE)*B. The system A*X = B, D*X = 0, is then equivalent to T T R*Z = Q *B , P *D*P*Z = 0 , where X = P*Z. If this system does not have full rank, then a least squares solution is obtained. On output QRSOLV also provides an upper triangular matrix S such that T T T P *(A *A + D*D)*P = S *S . S is computed within QRSOLV and may be of separate interest. The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. R is an N by N array. On input the full upper triangle must contain the full upper triangle of the matrix R. On output the full upper triangle is unaltered, and the strict lower triangle contains the strict upper triangle (transposed) of the upper triangular matrix S. LDR is a positive integer input variable not less than N which specifies the leading dimension of the array R. IPVT is an integer input array of length N which defines the permutation matrix P such that A*P = Q*R. Column J of P is column IPVT(J) of the identity matrix. DIAG is an input array of length N which must contain the diagonal elements of the matrix D. QTB is an input array of length N which must contain the first N elements of the vector (Q TRANSPOSE)*B. X is an output array of length N which contains the least squares solution of the system A*X = B, D*X = 0. SIGMA is an output array of length N which contains the diagonal elements of the upper triangular matrix S. WA is a work array of length N.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SNLS1, SNLS1E`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qrsolv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qrsolv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qrsolv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an M by N matrix A, an N by N diagonal matrix D, and an M-vector B, the problem is to determine an X which solves the system A*X = B , D*X = 0 , in the least squares sense. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDR, *) | That is, if A*P = Q*R, where P is a permutation matrix, Q has orthogonal columns, and R is an upper triangular matrix with diagonal elements of nonincreasing magnitude, then QRSOLV expects the full upper triangle of R, the permutation matrix P, and the first N components of (Q TRANSPOSE)*B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDR` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPVT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DIAG` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QTB` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Given an M by N matrix A, an N by N diagonal matrix D, and an M-vector B, the problem is to determine an X which solves the system A*X = B , D*X = 0 , in the least squares sense. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SIGMA` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE QRSOLV(N,R,LDR,IPVT,DIAG,QTB,X,SIGMA,WA) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
