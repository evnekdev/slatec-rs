# DWUPDT

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DNLS1 and DNLS1E

## Description

Given an N by N upper triangular matrix R, this subroutine computes the QR decomposition of the matrix formed when a row is added to R. If the row is specified by the vector W, then DWUPDT determines an orthogonal matrix Q such that when the N+1 by N matrix composed of R augmented by W is premultiplied by (Q TRANSPOSE), the resulting matrix is upper trapezoidal. The orthogonal matrix Q is the product of N transformations G(1)*G(2)* ... *G(N) where G(I) is a Givens rotation in the (I,N+1) plane which eliminates elements in the I-th plane. DWUPDT also computes the product (Q TRANSPOSE)*C where C is the (N+1)-vector (b,alpha). Q itself is not accumulated, rather the information to recover the G rotations is supplied. The subroutine statement is SUBROUTINE DWUPDT(N,R,LDR,W,B,ALPHA,COS,SIN) where N is a positive integer input variable set to the order of R. R is an N by N array. On input the upper triangular part of R must contain the matrix to be updated. On output R contains the updated triangular matrix. LDR is a positive integer input variable not less than N which specifies the leading dimension of the array R. W is an input array of length N which must contain the row vector to be added to R. B is an array of length N. On input B must contain the first N elements of the vector C. On output B contains the first N elements of the vector (Q TRANSPOSE)*C. ALPHA is a variable. On input ALPHA must contain the (N+1)-st element of the vector C. On output ALPHA contains the (N+1)-st element of the vector (Q TRANSPOSE)*C. COS is an output array of length N which contains the cosines of the transforming Givens rotations. SIN is an output array of length N which contains the sines of the transforming Givens rotations. **********

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DNLS1, DNLS1E`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dwupdt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dwupdt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dwupdt.f) — `verified_cached`
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
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Given an N by N upper triangular matrix R, this subroutine computes the QR decomposition of the matrix formed when a row is added to R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDR, *) | Given an N by N upper triangular matrix R, this subroutine computes the QR decomposition of the matrix formed when a row is added to R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDR` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The subroutine statement is SUBROUTINE DWUPDT(N,R,LDR,W,B,ALPHA,COS,SIN) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | If the row is specified by the vector W, then DWUPDT determines an orthogonal matrix Q such that when the N+1 by N matrix composed of R augmented by W is premultiplied by (Q TRANSPOSE), the resulting matrix is upper trapezoidal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DWUPDT also computes the product (Q TRANSPOSE)*C where C is the (N+1)-vector (b,alpha). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALPHA` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DWUPDT also computes the product (Q TRANSPOSE)*C where C is the (N+1)-vector (b,alpha). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `COS` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE DWUPDT(N,R,LDR,W,B,ALPHA,COS,SIN) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SIN` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The subroutine statement is SUBROUTINE DWUPDT(N,R,LDR,W,B,ALPHA,COS,SIN) where N is a positive integer input variable set to the order of R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
