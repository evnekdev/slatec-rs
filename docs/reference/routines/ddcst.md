# DDCST

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

DDCST sets coefficients used by the core integrator DDSTP.

## Description

DDCST is called by DDNTL. The array EL determines the basic method. The array TQ is involved in adjusting the step size in relation to truncation error. EL and TQ depend upon MINT, and are calculated for orders 1 to MAXORD(.LE. 12). For each order NQ, the coefficients EL are calculated from the generating polynomial: L(T) = EL(1,NQ) + EL(2,NQ)*T + ... + EL(NQ+1,NQ)*T**NQ. For the implicit Adams methods, L(T) is given by dL/dT = (1+T)*(2+T)* ... *(NQ-1+T)/K, L(-1) = 0, where K = factorial(NQ-1). For the Gear methods, L(T) = (1+T)*(2+T)* ... *(NQ+T)/K, where K = factorial(NQ)*(1 + 1/2 + ... + 1/NQ). For each order NQ, there are three components of TQ.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `ode-dae-families`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ddcst.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddcst.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddcst.f) — `verified_cached`
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
| `MAXORD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | EL and TQ depend upon MINT, and are calculated for orders 1 to MAXORD(.LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MINT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | EL and TQ depend upon MINT, and are calculated for orders 1 to MAXORD(.LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ISWFLG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (13, 12) | The array EL determines the basic method. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TQ` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (3, 12) | The array TQ is involved in adjusting the step size in relation to truncation error. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
