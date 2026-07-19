# BESK0E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero.

## Description

BESK0E(X) computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order zero for real argument X .GT. 0.0, i.e., EXP(X)*K0(X). Series for BK0 on the interval 0. to 4.00000D+00 with weighted error 3.57E-19 log weighted error 18.45 significant figures required 17.99 decimal places required 18.97 Series for AK0 on the interval 1.25000D-01 to 5.00000D-01 with weighted error 5.34E-17 log weighted error 16.27 significant figures required 14.92 decimal places required 16.89 Series for AK02 on the interval 0. to 1.25000D-01 with weighted error 2.34E-17 log weighted error 16.63 significant figures required 14.67 decimal places required 17.20

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
- Safe Rust paths: `slatec::special::bessel::bessel_k0_scaled_f32`

## Providers

- Canonical provider: `fnlib/besk0e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/besk0e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/besk0e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
