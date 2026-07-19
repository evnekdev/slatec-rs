# DERFC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the complementary error function.

## Description

DERFC(X) calculates the double precision complementary error function for double precision argument X. Series for ERF on the interval 0. to 1.00000E+00 with weighted Error 1.28E-32 log weighted Error 31.89 significant figures required 31.05 decimal places required 32.55 Series for ERC2 on the interval 2.50000E-01 to 1.00000E+00 with weighted Error 2.67E-32 log weighted Error 31.57 significant figures required 30.31 decimal places required 32.42 Series for ERFC on the interval 0. to 2.50000E-01 with weighted error 1.53E-31 log weighted error 30.82 significant figures required 29.47 decimal places required 31.70

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
- GAMS classifications: `C8A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::error_functions::erfc`

## Providers

- Canonical provider: `fnlib/derfc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/derfc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/derfc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
