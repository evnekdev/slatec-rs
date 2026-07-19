# DSISL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a real symmetric system using the factors obtained from SSIFA.

## Description

DSISL solves the double precision symmetric system A * X = B using the factors computed by DSIFA. On Entry A DOUBLE PRECISION(LDA,N) the output from DSIFA. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . KPVT INTEGER(N) the pivot vector from DSIFA. B DOUBLE PRECISION(N) the right hand side vector. On Return B the solution vector X . Error Condition A division by zero may occur if DSICO has set RCOND .EQ. 0.0 or DSIFA has set INFO .NE. 0 . To compute INVERSE(A) * C where C is a matrix with P columns CALL DSIFA(A,LDA,N,KPVT,INFO) IF (INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL DSISL(A,LDA,N,KPVT,C(1,J)) 10 CONTINUE

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2B1A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/dsisl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsisl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dsisl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
