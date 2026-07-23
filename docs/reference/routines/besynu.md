# BESYNU

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BESY

## Description

Abstract BESYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. Equations of the references are implemented on small orders DNU for Y/SUB(DNU)/(X) and Y/SUB(DNU+1)/(X). Forward recursion with the three term recursion relation generates higher orders FNU+I-1, I=1,...,N. To start the recursion FNU is normalized to the interval -0.5.LE.DNU.LT.0.5. A special form of the power series is implemented on 0.LT.X.LE.X1 while the Miller algorithm for the K Bessel function in terms of the confluent hypergeometric function U(FNU+0.5,2*FNU+1,I*X) is implemented on X1.LT.X.LE.X Here I is the complex number SQRT(-1.). For X.GT.X2, the asymptotic expansion for large X is used. When FNU is a half odd integer, a special formula for DNU=-0.5 and DNU+1.0=0.5 is used to start the recursion. BESYNU assumes that a significant digit SINH(X) function is available. Description of Arguments

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BESY`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/besynu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/besynu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/besynu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/besynu.f) — `verified_cached`
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
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Abstract BESYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Abstract BESYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract BESYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Abstract BESYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
