# ZAIRY

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the Airy function Ai(z) or its derivative dAi/dz for complex argument z. A scaling option is available to help avoid underflow and overflow.

## Description

***A DOUBLE PRECISION ROUTINE*** On KODE=1, ZAIRY computes the complex Airy function Ai(z) or its derivative dAi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(zeta)*Ai(z) or exp(zeta)*dAi/dz is provided to remove the exponential decay in -pi/3<arg(z) <pi/3 and the exponential growth in pi/3<abs(arg(z))<pi where zeta=(2/3)*z**(3/2). While the Airy functions Ai(z) and dAi/dz are analytic in the whole z-plane, the corresponding scaled functions defined for KODE=2 have a cut along the negative real axis.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f64`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/zairy.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zairy.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zairy.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/zairy.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
