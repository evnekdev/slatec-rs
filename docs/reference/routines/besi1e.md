# BESI1E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one.

## Description

BESI1E(X) calculates the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for real argument X; i.e., EXP(-ABS(X))*I1(X). Series for BI1 on the interval 0. to 9.00000D+00 with weighted error 2.40E-17 log weighted error 16.62 significant figures required 16.23 decimal places required 17.14 Series for AI1 on the interval 1.25000D-01 to 3.33333D-01 with weighted error 6.98E-17 log weighted error 16.16 significant figures required 14.53 decimal places required 16.82 Series for AI12 on the interval 0. to 1.25000D-01 with weighted error 3.55E-17 log weighted error 16.45 significant figures required 14.69 decimal places required 17.12

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
- GAMS classifications: `C10B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::bessel::bessel_i1_scaled_f32`

## Providers

- Canonical provider: `fnlib/besi1e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/besi1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/besi1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::besi1e`
- Current legacy Rust paths: `slatec_sys::families::besi1e`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::special::bessel::bessel_i1_scaled_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
