# R1MPYQ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNSQ and SNSQE

## Description

Given an M by N matrix A, this subroutine computes A*Q where Q is the product of 2*(N - 1) transformations GV(N-1)*...*GV(1)*GW(1)*...*GW(N-1) and GV(I), GW(I) are Givens rotations in the (I,N) plane which eliminate elements in the I-th and N-th planes, respectively. Q itself is not given, rather the information to recover the GV, GW rotations is supplied. The subroutine statement is SUBROUTINE R1MPYQ(M,N,A,LDA,V,W) where M is a positive integer input variable set to the number of rows of A. N is a positive integer input variable set to the number of columns of A. A is an M by N ARRAY. On input A must contain the matrix to be postmultiplied by the orthogonal matrix Q described above. On output A*Q has replaced A. LDA is a positive integer input variable not less than M which specifies the leading dimension of the array A. V is an input array of length N. V(I) must contain the information necessary to recover the Givens rotation GV(I) described above. W is an input array of length N. W(I) must contain the information necessary to recover the Givens rotation GW(I) described above.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SNSQ, SNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/r1mpyq.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/r1mpyq.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/r1mpyq.f) — `verified_cached`
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
