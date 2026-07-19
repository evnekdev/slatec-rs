# BESI0E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero.

## Description

BESI0E(X) calculates the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero for real argument X; i.e., EXP(-ABS(X))*I0(X). Series for BI0 on the interval 0. to 9.00000D+00 with weighted error 2.46E-18 log weighted error 17.61 significant figures required 17.90 decimal places required 18.15 Series for AI0 on the interval 1.25000D-01 to 3.33333D-01 with weighted error 7.87E-17 log weighted error 16.10 significant figures required 14.69 decimal places required 16.76 Series for AI02 on the interval 0. to 1.25000D-01 with weighted error 3.79E-17 log weighted error 16.42 significant figures required 14.86 decimal places required 17.09

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
- Safe Rust paths: `slatec::special::bessel::bessel_i0_scaled_f32`

## Providers

- Canonical provider: `fnlib/besi0e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/besi0e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/besi0e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
