# DBSYNU

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBESY

## Description

Abstract **** A DOUBLE PRECISION routine **** DBSYNU computes N member sequences of Y Bessel functions Y/SUB(FNU+I-1)/(X), I=1,N for non-negative orders FNU and positive X. Equations of the references are implemented on small orders DNU for Y/SUB(DNU)/(X) and Y/SUB(DNU+1)/(X). Forward recursion with the three term recursion relation generates higher orders FNU+I-1, I=1,...,N. To start the recursion FNU is normalized to the interval -0.5.LE.DNU.LT.0.5. A special form of the power series is implemented on 0.LT.X.LE.X1 while the Miller algorithm for the K Bessel function in terms of the confluent hypergeometric function U(FNU+0.5,2*FNU+1,I*X) is implemented on X1.LT.X.LE.X Here I is the complex number SQRT(-1.). For X.GT.X2, the asymptotic expansion for large X is used. When FNU is a half odd integer, a special formula for DNU=-0.5 and DNU+1.0=0.5 is used to start the recursion. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in DOUBLE PRECISION arithmetic. DBSYNU assumes that a significant digit SINH function is available. Description of Arguments

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBESY`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbsynu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbsynu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbsynu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbsynu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
