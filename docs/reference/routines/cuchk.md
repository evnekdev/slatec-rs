# CUCHK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

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

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
