# POLINT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Produce the polynomial which interpolates a set of discrete data points.

## Description

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Abstract Subroutine POLINT is designed to produce the polynomial which interpolates the data (X(I),Y(I)), I=1,...,N. POLINT sets up information in the array C which can be used by subroutine POLYVL to evaluate the polynomial and its derivatives and by subroutine POLCOF to produce the coefficients. Formal Parameters N - the number of data points (N .GE. 1) X - the array of abscissas (all of which must be distinct) Y - the array of ordinates C - an array of information used by subroutines ******* Dimensioning Information ******* Arrays X,Y, and C must be dimensioned at least N in the calling program.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1B`
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

- Canonical provider: `main-src/src/polint.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/polint.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/polint.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/polint.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
