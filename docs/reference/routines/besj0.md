# BESJ0

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the Bessel function of the first kind of order zero.

## Description

BESJ0(X) calculates the Bessel function of the first kind of order zero for real argument X. Series for BJ0 on the interval 0. to 1.60000D+01 with weighted error 7.47E-18 log weighted error 17.13 significant figures required 16.98 decimal places required 17.68 Series for BM0 on the interval 0. to 6.25000D-02 with weighted error 4.98E-17 log weighted error 16.30 significant figures required 14.97 decimal places required 16.96 Series for BTH0 on the interval 0. to 6.25000D-02 with weighted error 3.67E-17 log weighted error 16.44 significant figures required 15.53 decimal places required 17.13

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
- GAMS classifications: `C10A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::bessel::bessel_j0_f32`

## Providers

- Canonical provider: `fnlib/besj0.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/besj0.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/besj0.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::besj0`
- Current legacy Rust paths: `slatec_sys::families::besj0`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::special::bessel::bessel_j0_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
