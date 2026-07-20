# DBSI1E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one.

## Description

DBSI1E(X) calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for double precision argument X. The result is I1(X) multiplied by EXP(-ABS(X)). Series for BI1 on the interval 0. to 9.00000E+00 with weighted error 1.44E-32 log weighted error 31.84 significant figures required 31.45 decimal places required 32.46 Series for AI1 on the interval 1.25000E-01 to 3.33333E-01 with weighted error 2.81E-32 log weighted error 31.55 significant figures required 29.93 decimal places required 32.38 Series for AI12 on the interval 0. to 1.25000E-01 with weighted error 1.83E-32 log weighted error 31.74 significant figures required 29.97 decimal places required 32.66

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
- GAMS classifications: `C10B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::bessel::bessel_i1_scaled`

## Providers

- Canonical provider: `fnlib/dbsi1e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbsi1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbsi1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::special::numerical::dbsi1e`
- Current legacy Rust paths: `slatec_sys::families::dbsi1e`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::special::bessel::bessel_i1_scaled`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
