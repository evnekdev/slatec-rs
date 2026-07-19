# RWUPDT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1 and SNLS1E

## Description

Given an N by N upper triangular matrix R, this subroutine computes the QR decomposition of the matrix formed when a row is added to R. If the row is specified by the vector W, then RWUPDT determines an orthogonal matrix Q such that when the N+1 by N matrix composed of R augmented by W is premultiplied by (Q TRANSPOSE), the resulting matrix is upper trapezoidal. The orthogonal matrix Q is the product of N transformations G(1)*G(2)* ... *G(N) where G(I) is a Givens rotation in the (I,N+1) plane which eliminates elements in the I-th plane. RWUPDT also computes the product (Q TRANSPOSE)*C where C is the (N+1)-vector (b,alpha). Q itself is not accumulated, rather the information to recover the G rotations is supplied. The subroutine statement is SUBROUTINE RWUPDT(N,R,LDR,W,B,ALPHA,COS,SIN) where N is a positive integer input variable set to the order of R. R is an N by N array. On input the upper triangular part of R must contain the matrix to be updated. On output R contains the updated triangular matrix. LDR is a positive integer input variable not less than N which specifies the leading dimension of the array R. W is an input array of length N which must contain the row vector to be added to R. B is an array of length N. On input B must contain the first N elements of the vector C. On output B contains the first N elements of the vector (Q TRANSPOSE)*C. ALPHA is a variable. On input ALPHA must contain the (N+1)-st element of the vector C. On output ALPHA contains the (N+1)-st element of the vector (Q TRANSPOSE)*C. COS is an output array of length N which contains the cosines of the transforming Givens rotations. SIN is an output array of length N which contains the sines of the transforming Givens rotations.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SNLS1, SNLS1E`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/rwupdt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rwupdt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rwupdt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
