# DDAWS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute Dawson's function.

## Description

DDAWS(X) calculates the double precision Dawson's integral for double precision argument X. Series for DAW on the interval 0. to 1.00000E+00 with weighted error 8.95E-32 log weighted error 31.05 significant figures required 30.41 decimal places required 31.71 Series for DAW2 on the interval 0. to 1.60000E+01 with weighted error 1.61E-32 log weighted error 31.79 significant figures required 31.40 decimal places required 32.62 Series for DAWA on the interval 0. to 6.25000E-02 with weighted error 1.97E-32 log weighted error 31.71 significant figures required 29.79 decimal places required 32.64

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C8C`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::elementary::dawson`

## Providers

- Canonical provider: `fnlib/ddaws.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/ddaws.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/ddaws.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::special::elementary::ddaws`
- Current legacy Rust paths: `slatec_sys::families::special_elementary::ddaws`
- Public declaration feature: `special-elementary`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::elementary::dawson`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
