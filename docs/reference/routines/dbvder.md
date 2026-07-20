# DBVDER

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBVSUP

## Description

********************************************************************** NFC = Number of base solution vectors NCOMP = Number of components per solution vector 1 -- Nonzero particular solution INHOMO = 2 or 3 -- Zero particular solution 0 -- Inhomogeneous vector term G(X) identically zero IGOFX = 1 -- Inhomogeneous vector term G(X) not identically zero G = Inhomogeneous vector term G(X) XSAV = Previous value of X C = Normalization factor for the particular solution 0 ( if NEQIVP = 0 ) IVP = Number of differential equations integrated due to the original boundary value problem ( if NEQIVP .GT. 0 ) NOFST - For problems with auxiliary initial value equations, NOFST communicates to the routine DFMAT how to access the dependent variables corresponding to this initial value problem. For example, during any call to DFMAT, the first dependent variable for the initial value problem is in position Y(NOFST + 1). See example in SAND77-1328. **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbvder.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvder.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvder.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvder.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ********************************************************************** NFC = Number of base solution vectors NCOMP = Number of components per solution vector 1 -- Nonzero particular solution INHOMO = 2 or 3 -- Zero particular solution 0 -- Inhomogeneous vector term G(X) identically zero IGOFX = 1 -- Inhomogeneous vector term G(X) not identically zero G = Inhomogeneous vector term G(X) XSAV = Previous value of X C = Normalization factor for the particular solution 0 ( if NEQIVP = 0 ) IVP = Number of differential equations integrated due to the original boundary value problem ( if NEQIVP .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | For example, during any call to DFMAT, the first dependent variable for the initial value problem is in position Y(NOFST + 1). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YP` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `G` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | ********************************************************************** NFC = Number of base solution vectors NCOMP = Number of components per solution vector 1 -- Nonzero particular solution INHOMO = 2 or 3 -- Zero particular solution 0 -- Inhomogeneous vector term G(X) identically zero IGOFX = 1 -- Inhomogeneous vector term G(X) not identically zero G = Inhomogeneous vector term G(X) XSAV = Previous value of X C = Normalization factor for the particular solution 0 ( if NEQIVP = 0 ) IVP = Number of differential equations integrated due to the original boundary value problem ( if NEQIVP .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
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
