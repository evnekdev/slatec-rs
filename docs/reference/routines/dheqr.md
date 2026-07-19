# DHEQR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine performs a QR decomposition of an upper Hessenberg matrix A using Givens rotations. There are two options available: 1) Performing a fresh decomposition 2) updating the QR factors by adding a row and a column to the matrix A. *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. On output, the upper triangular matrix R. The factorization can be written Q*A = R, where Q is a product of Givens rotations and R is upper triangular. LDA :IN Integer The leading dimension of the array A. N :IN Integer A is an (N+1) by N Hessenberg matrix. Q :OUT Double Precision Q(2*N) The factors c and s of each Givens rotation used in decomposing A. INFO :OUT Integer = 0 normal value. = K if A(K,K) .eq. 0.0 . This is not an error condition for this subroutine, but it does indicate that DHELS will divide by zero if called. IJOB :IN Integer = 1 means that a fresh decomposition of the matrix A is desired. .ge. 2 means that the current decomposition of A will be updated by the addition of a row and a column.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `lin/dheqr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dheqr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
