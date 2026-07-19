# BESJ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.

## Description

Abstract BESJ computes an N member sequence of J Bessel functions J/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values of (NU,X) not covered by one of these formulae, the order is incremented or decremented by integer values into a region where one of the formulae apply. Backward recursion is applied to reduce orders by integer values except where the entire sequence lies in the oscillatory region. In this case forward recursion is stable and values from the asymptotic expansion for X to infinity start the recursion when it is efficient to do so. Leading terms of the series and uniform expansion are tested for underflow. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all members are set to zero. Overflow cannot occur. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10A3`
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

- Canonical provider: `main-src/src/besj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/besj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/besj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/besj.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
