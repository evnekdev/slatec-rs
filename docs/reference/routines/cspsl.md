# CSPSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a complex symmetric system using the factors obtained from CSPFA.

## Description

CSISL solves the complex symmetric system A * X = B using the factors computed by CSPFA. On Entry AP COMPLEX(N*(N+1)/2) the output from CSPFA. N INTEGER the order of the matrix A . KVPT INTEGER(N) the pivot vector from CSPFA. B COMPLEX(N) the right hand side vector. On Return B the solution vector X . Error Condition A division by zero may occur if CSPCO has set RCOND .EQ. 0.0 or CSPFA has set INFO .NE. 0 . To compute INVERSE(A) * C where C is a matrix with P columns CALL CSPFA(AP,N,KVPT,INFO) IF (INFO .NE. 0) GO TO ... DO 10 J = 1, P CALL CSPSL(AP,N,KVPT,C(1,J)) 10 CONTINUE

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2C1`
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

- Canonical provider: `lin/cspsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cspsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cspsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
