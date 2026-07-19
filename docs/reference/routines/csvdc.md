# CSVDC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Perform the singular value decomposition of a rectangular matrix.

## Description

CSVDC is a subroutine to reduce a complex NxP matrix X by unitary transformations U and V to diagonal form. The diagonal elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry X COMPLEX(LDX,P), where LDX .GE. N. X contains the matrix whose singular value decomposition is to be computed. X is destroyed by CSVDC. LDX INTEGER. LDX is the leading dimension of the array X. N INTEGER. N is the number of rows of the matrix X. P INTEGER. P is the number of columns of the matrix X. LDU INTEGER. LDU is the leading dimension of the array U (see below). LDV INTEGER. LDV is the leading dimension of the array V (see below). WORK COMPLEX(N). WORK is a scratch array. JOB INTEGER. JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A .EQ. 0 Do not compute the left singular vectors. A .EQ. 1 Return the N left singular vectors in U. A .GE. 2 Return the first MIN(N,P) left singular vectors in U. B .EQ. 0 Do not compute the right singular vectors. B .EQ. 1 Return the right singular vectors in V. On Return S COMPLEX(MM), where MM = MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude. E COMPLEX(P). E ordinarily contains zeros. However see the discussion of INFO for exceptions. U COMPLEX(LDU,K), where LDU .GE. N. If JOBA .EQ. 1 then K .EQ. N. If JOBA .GE. 2 then K .EQ. MIN(N,P). U contains the matrix of right singular vectors. U is not referenced if JOBA .EQ. 0. If N .LE. P or if JOBA .GT. 2, then U may be identified with X in the subroutine call. V COMPLEX(LDV,P), where LDV .GE. P. V contains the matrix of right singular vectors. V is not referenced if JOB .EQ. 0. If P .LE. N, then V may be identified with X in the subroutine call. INFO INTEGER. The singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),...,S(M) are correct (here M=MIN(N,P)). Thus if INFO.EQ. 0, all the singular values and their vectors are correct. In any event, the matrix B = CTRANS(U)*X*V is the bidiagonal matrix with the elements of S on its diagonal and the elements of E on its super-diagonal (CTRANS(U) is the conjugate-transpose of U). Thus the singular values of X and B are the same.

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
- GAMS classifications: `D6`
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

- Canonical provider: `lin/csvdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/csvdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/csvdc.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
