# BFQAD

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the integral of a product of a function and a derivative of a B-spline.

## Description

Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). (X1,X2) must be a subinterval of T(K) .LE. X .le. T(N+1). An integration routine BSGQ8 (a modification of GAUS8), integrates the product on subintervals of (X1,X2) formed by included (distinct) knots. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/bfqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bfqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bfqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bfqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `support_unit_minimal`
- Description provenance: `source_prologue`
- Assessment: the support identity records its role, side-effect boundary, and non-public disposition
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BCOEF` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ID` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X1` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X2` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Abstract BFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `QUAD` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `unsupported-abi`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
