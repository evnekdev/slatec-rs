# PCHKT

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute B-spline knot sequence for PCHBS.

## Description

Set a knot sequence for the B-spline representation of a PCH function with breakpoints X. All knots will be at least double. Endknots are set as: (1) quadruple knots at endpoints if KNOTYP=0; (2) extrapolate the length of end interval if KNOTYP=1; (3) periodic if KNOTYP=2. Input arguments: N, X, KNOTYP. Output arguments: T. Restrictions/assumptions: 1. N.GE.2 . (not checked) 2. X(i).LT.X(i+1), i=1,...,N . (not checked) 3. 0.LE.KNOTYP.LE.2 . (Acts like KNOTYP=0 for any other value.)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `pchip/pchkt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchkt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [PCHIP](../families/pchip.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Input arguments: N, X, KNOTYP. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Set a knot sequence for the B-spline representation of a PCH function with breakpoints X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KNOTYP` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Endknots are set as: (1) quadruple knots at endpoints if KNOTYP=0; (2) extrapolate the length of end interval if KNOTYP=1; (3) periodic if KNOTYP=2. | Endknots are set as: (1) quadruple knots at endpoints if KNOTYP=0; (2) extrapolate the length of end interval if KNOTYP=1; (3) periodic if KNOTYP=2. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Output arguments: T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
