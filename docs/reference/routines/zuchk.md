# ZUCHK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SERI, ZUOIK, ZUNK1, ZUNK2, ZUNI1, ZUNI2 and ZKSCL

## Description

Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*D1MACH(1)/TOL. THE TEST IS MADE TO SEE IF THE MAGNITUDE OF THE REAL OR IMAGINARY PART WOULD UNDERFLOW WHEN Y IS SCALED (BY TOL) TO ITS PROPER VALUE. Y IS ACCEPTED IF THE UNDERFLOW IS AT LEAST ONE PRECISION BELOW THE MAGNITUDE OF THE LARGEST COMPONENT; OTHERWISE THE PHASE ANGLE DOES NOT HAVE ABSOLUTE ACCURACY AND AN UNDERFLOW IS ASSUMED.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f64`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `ZKSCL, ZUNI1, ZUNI2, ZUNK1, ZUNK2, ZUOIK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/zuchk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zuchk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zuchk.f) — `verified_cached`
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
| `YR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YI` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ASCLE` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*D1MACH(1)/TOL. | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*D1MACH(1)/TOL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*D1MACH(1)/TOL. | Y ENTERS AS A SCALED QUANTITY WHOSE MAGNITUDE IS GREATER THAN EXP(-ALIM)=ASCLE=1.0E+3*D1MACH(1)/TOL. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-scalar-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
