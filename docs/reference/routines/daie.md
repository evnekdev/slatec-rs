# DAIE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument.

## Description

DAIE(X) calculates the Airy function or the exponentially scaled Airy function depending on the value of the argument. The function and argument are both double precision. Evaluate AI(X) for X .LE. 0.0 and AI(X)*EXP(ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for AIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 8.37E-33 log weighted error 32.08 significant figures required 30.87 decimal places required 32.63 Series for AIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 7.47E-34 log weighted error 33.13 significant figures required 31.50 decimal places required 33.68 Series for AIP1 on the interval 1.25000E-01 to 1.00000E+00 with weighted error 3.69E-32 log weighted error 31.43 significant figures required 29.55 decimal places required 32.31 Series for AIP2 on the interval 0. to 1.25000E-01 with weighted error 3.48E-32 log weighted error 31.46 significant figures required 28.74 decimal places required 32.24

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
- GAMS classifications: `C10D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::airy::airy_ai_scaled`

## Providers

- Canonical provider: `fnlib/daie.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/daie.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/daie.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::families::daie`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::airy::airy_ai_scaled`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
