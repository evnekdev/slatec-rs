# SPODI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant and inverse of a certain real symmetric positive definite matrix using the factors computed by SPOCO, SPOFA or SQRDC.

## Description

SPODI computes the determinant and inverse of a certain real symmetric positive definite matrix (see below) using the factors computed by SPOCO, SPOFA or SQRDC. On Entry A REAL(LDA, N) the output A from SPOCO or SPOFA or the output X from SQRDC. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . JOB INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only. On Return A If SPOCO or SPOFA was used to factor A , then SPODI produces the upper half of INVERSE(A) . If SQRDC was used to decompose X , then SPODI produces the upper half of INVERSE(TRANS(X)*X), where TRANS(X) is the transpose. Elements of A below the diagonal are unchanged. If the units digit of JOB is zero, A is unchanged. DET REAL(2) determinant of A or of TRANS(X)*X if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. DET(1) .LT. 10.0 or DET(1) .EQ. 0.0 . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal and the inverse is requested. It will not occur if the subroutines are called correctly and if SPOCO or SPOFA has set INFO .EQ. 0 .

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2B1B`
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

- Canonical provider: `lin/spodi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/spodi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/spodi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
