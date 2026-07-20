# EXPREL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the relative error exponential (EXP(X)-1)/X.

## Description

Evaluate EXPREL(X) = (EXP(X) - 1.0) / X. For small ABS(X) the Taylor series is used. If X is negative, the reflection formula EXPREL(X) = EXP(X) * EXPREL(ABS(X)) may be used. This reflection formula will be of use when the evaluation for small ABS(X) is done by Chebyshev series rather than Taylor series. EXPREL and X are single precision.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Elementary and transcendental functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C4B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::elementary::exprel_f32`

## Providers

- Canonical provider: `fnlib/exprel.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/exprel.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/exprel.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::special::elementary::exprel`
- Current legacy Rust paths: `slatec_sys::families::special_elementary::exprel`
- Public declaration feature: `special-elementary`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::elementary::exprel_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
