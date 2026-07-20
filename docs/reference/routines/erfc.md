# ERFC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the complementary error function.

## Description

ERFC(X) calculates the single precision complementary error function for single precision argument X. Series for ERF on the interval 0. to 1.00000D+00 with weighted error 7.10E-18 log weighted error 17.15 significant figures required 16.31 decimal places required 17.71 Series for ERFC on the interval 0. to 2.50000D-01 with weighted error 4.81E-17 log weighted error 16.32 approx significant figures required 15.0 Series for ERC2 on the interval 2.50000D-01 to 1.00000D+00 with weighted error 5.22E-17 log weighted error 16.28 approx significant figures required 15.0 decimal places required 16.96

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C8A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::error_functions::erfc_f32`

## Providers

- Canonical provider: `fnlib/erfc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/erfc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/erfc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::special::error::erfc`
- Current legacy Rust paths: `slatec_sys::families::special_error::erfc`
- Public declaration feature: `special-error`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::error_functions::erfc_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
