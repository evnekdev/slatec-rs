# DBSGQ8

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBFQAD

## Description

Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. Description of Arguments

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBFQAD`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbsgq8.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbsgq8.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbsgq8.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbsgq8.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `FUN` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ID` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INBV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ERR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ANS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Abstract **** A DOUBLE PRECISION routine **** DBSGQ8, a modification of GAUS8, integrates the product of FUN(X) by the ID-th derivative of a spline DBVALU(XT,BC,N,KK,ID,X,INBV,WORK) between limits A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
