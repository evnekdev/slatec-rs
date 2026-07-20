# DBI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Evaluate the Bairy function (the Airy function of the second kind).

## Description

DBI(X) calculates the double precision Airy function of the second kind for double precision argument X. Series for BIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.45E-32 log weighted error 31.84 significant figures required 30.85 decimal places required 32.40 Series for BIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.29E-33 log weighted error 32.89 significant figures required 31.48 decimal places required 33.45 Series for BIF2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 6.08E-32 log weighted error 31.22 approx significant figures required 30.8 decimal places required 31.80 Series for BIG2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 4.91E-33 log weighted error 32.31 approx significant figures required 31.6 decimal places required 32.90

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
- Safe Rust paths: `slatec::special::airy::airy_bi`

## Providers

- Canonical provider: `fnlib/dbi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::special::airy::dbi`
- Current legacy Rust paths: `slatec_sys::families::special_airy::dbi`
- Public declaration feature: `special-airy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::airy::airy_bi`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
