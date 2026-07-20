# SCHEX

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Update the Cholesky factorization A=TRANS(R)*R of A positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E, where E is a permutation matrix.

## Description

SCHEX updates the Cholesky factorization A = TRANS(R)*R of a positive definite matrix A of order P under diagonal permutations of the form TRANS(E)*A*E where E is a permutation matrix. Specifically, given an upper triangular matrix R and a permutation matrix E (which is specified by K, L, and JOB), SCHEX determines an orthogonal matrix U such that U*R*E = RR, where RR is upper triangular. At the users option, the transformation U will be multiplied into the array Z. If A = TRANS(X)*X, so that R is the triangular part of the QR factorization of X, then RR is the triangular part of the QR factorization of X*E, i.e., X with its columns permuted. For a less terse description of what SCHEX does and how it may be applied, see the LINPACK guide. The matrix Q is determined as the product U(L-K)*...*U(1) of plane rotations of the form ( C(I) S(I) ) ( ) , ( -S(I) C(I) ) where C(I) is real. The rows these rotations operate on are described below. There are two types of permutations, which are determined by the value of JOB. 1. Right circular shift (JOB = 1). The columns are rearranged in the following order. 1,...,K-1,L,K,K+1,...,L-1,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (L-I,L-I+1)-plane. 2. Left circular shift (JOB = 2). The columns are rearranged in the following order 1,...,K-1,K+1,K+2,...,L,K,L+1,...,P. U is the product of L-K rotations U(I), where U(I) acts in the (K+I-1,K+I)-plane. On Entry R REAL(LDR,P), where LDR .GE. P. R contains the upper triangular factor that is to be updated. Elements of R below the diagonal are not referenced. LDR INTEGER. LDR is the leading dimension of the array R. P INTEGER. P is the order of the matrix R. K INTEGER. K is the first column to be permuted. L INTEGER. L is the last column to be permuted. L must be strictly greater than K. Z REAL(LDZ,NZ), where LDZ.GE.P. Z is an array of NZ P-vectors into which the transformation U is multiplied. Z is not referenced if NZ = 0. LDZ INTEGER. LDZ is the leading dimension of the array Z. NZ INTEGER. NZ is the number of columns of the matrix Z. JOB INTEGER. JOB determines the type of permutation. JOB = 1 right circular shift. JOB = 2 left circular shift. On Return R contains the updated factor. Z contains the updated matrix Z. C REAL(P). C contains the cosines of the transforming rotations. S REAL(P). S contains the sines of the transforming rotations.

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
- GAMS classifications: `D7B`
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

- Canonical provider: `lin/schex.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/schex.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/schex.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
