# CUCHK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SERI, CUOIK, CUNK1, CUNK2, CUNI1, CUNI2 and CKSCL

## Description

Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. THE TEST IS MADE TO SEE IF THE MAGNITUDE OF THE REAL OR IMAGINARY PART WOULD UNDER FLOW WHEN Y IS SCALED (BY TOL) TO ITS PROPER VALUE. Y IS ACCEPTED IF THE UNDERFLOW IS AT LEAST ONE PRECISION BELOW THE MAGNITUDE OF THE LARGEST COMPONENT; OTHERWISE THE PHASE ANGLE DOES NOT HAVE ABSOLUTE ACCURACY AND AN UNDERFLOW IS ASSUMED.

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
- Parent-family evidence: `CKSCL, CUNI1, CUNI2, CUNK1, CUNK2, CUOIK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cuchk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cuchk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cuchk.f) — `verified_cached`
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
| `Y` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ASCLE` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*R1MACH(1)/TOL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
