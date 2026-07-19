# CNBSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a complex band system using the factors computed by CNBCO or CNBFA.

## Description

CNBSL solves the complex band system A * X = B or CTRANS(A) * X = B using the factors computed by CNBCO or CNBFA. On Entry ABE COMPLEX(LDA, NC) the output from CNBCO or CNBFA. NC must be .GE. 2*ML+MU+1 . LDA INTEGER the leading dimension of the array ABE . N INTEGER the order of the original matrix. ML INTEGER number of diagonals below the main diagonal. MU INTEGER number of diagonals above the main diagonal. IPVT INTEGER(N) the pivot vector from CNBCO or CNBFA. B COMPLEX(N) the right hand side vector. JOB INTEGER = 0 to solve A*X = B . = nonzero to solve CTRANS(A)*X = B , where CTRANS(A) is the conjugate transpose. On Return B the solution vector X . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA. It will not occur if the subroutines are called correctly and if CNBCO has set RCOND .GT. 0.0 or CNBFA has set INFO .EQ. 0 . To compute INVERSE(A) * C where C is a matrix with P columns CALL CNBCO(ABE,LDA,N,ML,MU,IPVT,RCOND,Z) IF (RCOND is too small) GO TO ... DO 10 J = 1, P CALL CNBSL(ABE,LDA,N,ML,MU,IPVT,C(1,J),0) 10 CONTINUE

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
- GAMS classifications: `D2C2`
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

- Canonical provider: `main-src/src/cnbsl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cnbsl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cnbsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cnbsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
