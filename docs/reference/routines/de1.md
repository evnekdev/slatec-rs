# DE1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponential integral E1(X).

## Description

DE1 calculates the double precision exponential integral, E1(X), for positive double precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X, E1(X) = -Ei(-X) or Ei(X) = -E1(-X). Series for AE10 on the interval -3.12500E-02 to 0. with weighted error 4.62E-32 log weighted error 31.34 significant figures required 29.70 decimal places required 32.18 Series for AE11 on the interval -1.25000E-01 to -3.12500E-02 with weighted error 2.22E-32 log weighted error 31.65 significant figures required 30.75 decimal places required 32.54 Series for AE12 on the interval -2.50000E-01 to -1.25000E-01 with weighted error 5.19E-32 log weighted error 31.28 significant figures required 30.82 decimal places required 32.09 Series for E11 on the interval -4.00000E+00 to -1.00000E+00 with weighted error 8.49E-34 log weighted error 33.07 significant figures required 34.13 decimal places required 33.80 Series for E12 on the interval -1.00000E+00 to 1.00000E+00 with weighted error 8.08E-33 log weighted error 32.09 approx significant figures required 30.4 decimal places required 32.79 Series for AE13 on the interval 2.50000E-01 to 1.00000E+00 with weighted error 6.65E-32 log weighted error 31.18 significant figures required 30.69 decimal places required 32.03 Series for AE14 on the interval 0. to 2.50000E-01 with weighted error 5.07E-32 log weighted error 31.30 significant figures required 30.40 decimal places required 32.20

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
- GAMS classifications: `C5`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::integrals::exponential_integral_e1`

## Providers

- Canonical provider: `fnlib/de1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/de1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/de1.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::families::de1`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::integrals::exponential_integral_e1`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
