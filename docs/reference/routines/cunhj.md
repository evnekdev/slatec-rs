# CUNHJ

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. (2/3)*FNU*ZETA**1.5 = ZETA1-ZETA2, ZETA1=0.5*FNU*CLOG((1+W)/(1-W)), ZETA2=FNU*W FOR SCALING PURPOSES IN AIRY FUNCTIONS FROM CAIRY OR CBIRY. MCONJ=SIGN OF AIMAG(Z), BUT IS AMBIGUOUS WHEN Z IS REAL AND MUST BE SPECIFIED. IPMTR=0 RETURNS ALL PARAMETERS. IPMTR= 1 COMPUTES ALL EXCEPT ASUM AND BSUM.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `CBESI, CBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cunhj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cunhj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cunhj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cunhj.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `Z` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPMTR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IPMTR=0 RETURNS ALL PARAMETERS. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PHI` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ARG` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ZETA1` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | (2/3)*FNU*ZETA**1.5 = ZETA1-ZETA2, ZETA1=0.5*FNU*CLOG((1+W)/(1-W)), ZETA2=FNU*W FOR SCALING PURPOSES IN AIRY FUNCTIONS FROM CAIRY OR CBIRY. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ZETA2` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | (2/3)*FNU*ZETA**1.5 = ZETA1-ZETA2, ZETA1=0.5*FNU*CLOG((1+W)/(1-W)), ZETA2=FNU*W FOR SCALING PURPOSES IN AIRY FUNCTIONS FROM CAIRY OR CBIRY. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ASUM` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BSUM` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CUNHJ COMPUTES PARAMETERS FOR BESSEL FUNCTIONS C(FNU,Z) = J(FNU,Z), Y(FNU,Z) OR H(I,FNU,Z) I=1,2 FOR LARGE ORDERS FNU BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION C(FNU,Z)=C1*PHI*( ASUM*AIRY(ARG) + C2*BSUM*DAIRY(ARG) ) FOR PROPER CHOICES OF C1, C2, AIRY AND DAIRY WHERE AIRY IS AN AIRY FUNCTION AND DAIRY IS ITS DERIVATIVE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
