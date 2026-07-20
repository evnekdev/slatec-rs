# CATAN2

[Family: Elementary and transcendental functions](../families/elementary-and-transcendental-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the complex arc tangent in the proper quadrant.

## Description

CATAN2(CSN,CCS) calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI). The result is in units of radians and the real part is between -PI and +PI.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Elementary and transcendental functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C4A`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fnlib/catan2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/catan2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/catan2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Elementary and transcendental functions](../families/elementary-and-transcendental-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `CSN` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CATAN2(CSN,CCS) calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CCS` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CATAN2(CSN,CCS) calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut crate::Complex32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:complex32(mut_complex32,mut_complex32)`.

### ABI and safety

Canonical path: `slatec_sys::special::complex::catan2`. Native symbol: `catan2_`. Feature: `special-complex`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:complex32(mut_complex32,mut_complex32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::complex::catan2`
- Compatibility aliases: `none`
- Public declaration feature: `special-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
