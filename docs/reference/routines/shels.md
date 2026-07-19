# SHELS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for SGMRES.

## Description

This routine is extracted from the LINPACK routine SGESL with changes due to the fact that A is an upper Hessenberg matrix. SHELS solves the least squares problem: MIN(B-A*X,B-A*X) using the factors computed by SHEQR. *Usage: INTEGER LDA, N REAL A(LDA,N), Q(2*N), B(N+1) CALL SHELS(A, LDA, N, Q, B) *Arguments: A :IN Real A(LDA,N) The output from SHEQR which contains the upper triangular factor R in the QR decomposition of A. LDA :IN Integer The leading dimension of the array A. N :IN Integer A is originally an (N+1) by N matrix. Q :IN Real Q(2*N) The coefficients of the N Givens rotations used in the QR factorization of A. B :INOUT Real B(N+1) On input, B is the right hand side vector. On output, B is the solution vector X.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A4`
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

- Canonical provider: `lin/shels.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/shels.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/shels.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
