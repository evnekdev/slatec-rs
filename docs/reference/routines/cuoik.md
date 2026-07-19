# CUOIK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESH, CBESI and CBESK

## Description

CUOIK COMPUTES THE LEADING TERMS OF THE UNIFORM ASYMPTOTIC EXPANSIONS FOR THE I AND K FUNCTIONS AND COMPARES THEM (IN LOGARITHMIC FORM) TO ALIM AND ELIM FOR OVER AND UNDERFLOW WHERE ALIM.LT.ELIM. IF THE MAGNITUDE, BASED ON THE LEADING EXPONENTIAL, IS LESS THAN ALIM OR GREATER THAN -ALIM, THEN THE RESULT IS ON SCALE. IF NOT, THEN A REFINED TEST USING OTHER MULTIPLIERS (IN LOGARITHMIC FORM) IS MADE BASED ON ELIM. HERE EXP(-ELIM)=SMALLEST MACHINE NUMBER*1.0E+3 AND EXP(-ALIM)= EXP(-ELIM)/TOL IKFLG=1 MEANS THE I SEQUENCE IS TESTED =2 MEANS THE K SEQUENCE IS TESTED NUF = 0 MEANS THE LAST MEMBER OF THE SEQUENCE IS ON SCALE =-1 MEANS AN OVERFLOW WOULD OCCUR IKFLG=1 AND NUF.GT.0 MEANS THE LAST NUF Y VALUES WERE SET TO ZERO THE FIRST N-NUF VALUES MUST BE SET BY ANOTHER ROUTINE IKFLG=2 AND NUF.EQ.N MEANS ALL Y VALUES WERE SET TO ZERO IKFLG=2 AND 0.LT.NUF.LT.N NOT CONSIDERED. Y MUST BE SET BY ANOTHER ROUTINE

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `CBESH, CBESI, CBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cuoik.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cuoik.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cuoik.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cuoik.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
