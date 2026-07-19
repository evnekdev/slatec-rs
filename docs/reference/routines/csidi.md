# CSIDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant and inverse of a complex symmetric matrix using the factors from CSIFA.

## Description

CSIDI computes the determinant and inverse of a complex symmetric matrix using the factors from CSIFA. On Entry A COMPLEX(LDA,N) the output from CSIFA. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . KVPT INTEGER(N) the pivot vector from CSIFA. WORK COMPLEX(N) work vector. Contents destroyed. JOB INTEGER JOB has the decimal expansion AB where If B .NE. 0, the inverse is computed, If A .NE. 0, the determinant is computed, For example, JOB = 11 gives both. On Return Variables not requested by JOB are not used. A contains the upper triangle of the inverse of the original matrix. The strict lower triangle is never referenced. DET COMPLEX(2) determinant of original matrix. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 or DET(1) = 0.0. Error Condition A division by zero may occur if the inverse is requested and CSICO has set RCOND .EQ. 0.0 or CSIFA has set INFO .NE. 0 .

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

- Canonical provider: `lin/csidi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/csidi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/csidi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
