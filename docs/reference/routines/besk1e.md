# BESK1E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one.

## Description

BESK1E(X) computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order one for real argument X .GT. 0.0, i.e., EXP(X)*K1(X). Series for BK1 on the interval 0. to 4.00000D+00 with weighted error 7.02E-18 log weighted error 17.15 significant figures required 16.73 decimal places required 17.67 Series for AK1 on the interval 1.25000D-01 to 5.00000D-01 with weighted error 6.06E-17 log weighted error 16.22 significant figures required 15.41 decimal places required 16.83 Series for AK12 on the interval 0. to 1.25000D-01 with weighted error 2.58E-17 log weighted error 16.59 significant figures required 15.22 decimal places required 17.16

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
- Safe Rust paths: `slatec::special::bessel::bessel_k1_scaled_f32`

## Providers

- Canonical provider: `fnlib/besk1e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/besk1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/besk1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
