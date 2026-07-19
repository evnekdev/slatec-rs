# DAWS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute Dawson's function.

## Description

DAWS(X) calculates Dawson's integral for real argument X. Series for DAW on the interval 0. to 1.00000D+00 with weighted error 3.83E-17 log weighted error 16.42 significant figures required 15.78 decimal places required 16.97 Series for DAW2 on the interval 0. to 1.60000D+01 with weighted error 5.17E-17 log weighted error 16.29 significant figures required 15.90 decimal places required 17.02 Series for DAWA on the interval 0. to 6.25000D-02 with weighted error 2.24E-17 log weighted error 16.65 significant figures required 14.73 decimal places required 17.36

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
- Safe Rust paths: `slatec::special::elementary::dawson_f32`

## Providers

- Canonical provider: `fnlib/daws.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/daws.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/daws.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
