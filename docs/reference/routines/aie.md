# AIE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument.

## Description

AIE(X) computes the exponentially scaled Airy function for non-negative X. It evaluates AI(X) for X .LE. 0.0 and EXP(ZETA)*AI(X) for X .GE. 0.0 where ZETA = (2.0/3.0)*(X**1.5). Series for AIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.09E-19 log weighted error 18.96 significant figures required 17.76 decimal places required 19.44 Series for AIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.51E-17 log weighted error 16.82 significant figures required 15.19 decimal places required 17.27 Series for AIP on the interval 0. to 1.00000D+00 with weighted error 5.10E-17 log weighted error 16.29 significant figures required 14.41 decimal places required 17.06

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
- GAMS classifications: `C10D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::airy::airy_ai_scaled_f32`

## Providers

- Canonical provider: `fnlib/aie.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/aie.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/aie.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
