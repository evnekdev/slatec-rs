# CCHUD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Update an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition.

## Description

CCHUD updates an augmented Cholesky decomposition of the triangular part of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHUD determines a unitary matrix U and a scalar ZETA such that (R Z) (RR ZZ ) U * ( ) = ( ) , (X Y) ( 0 ZETA) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) appended. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the updated problem is SQRT(RHO**2 + ZETA**2). CCHUD will simultaneously update several triplets (Z,Y,RHO). For a less terse description of what CCHUD does and how it may be applied see the LINPACK Guide. The matrix U is determined as the product U(P)*...*U(1), where U(I) is a rotation in the (I,P+1) plane of the form ( (CI) S(I) ) ( ) . ( -CONJG(S(I)) (CI) ) The rotations are chosen so that C(I) is real. On Entry R COMPLEX(LDR,P), where LDR .GE. P. R contains the upper triangular matrix that is to be updated. The part of R below the diagonal is not referenced. LDR INTEGER. LDR is the leading dimension of the array R. P INTEGER. P is the order of the matrix R. X COMPLEX(P). X contains the row to be added to R. X is not altered by CCHUD. Z COMPLEX(LDZ,NZ), where LDZ .GE. P. Z is an array containing NZ P-vectors to be updated with R. LDZ INTEGER. LDZ is the leading dimension of the array Z. NZ INTEGER. NZ is the number of vectors to be updated NZ may be zero, in which case Z, Y, and RHO are not referenced. Y COMPLEX(NZ). Y contains the scalars for updating the vectors Z. Y is not altered by CCHUD. RHO REAL(NZ). RHO contains the norms of the residual vectors that are to be updated. If RHO(J) is negative, it is left unaltered. On Return RC RHO contain the updated quantities. Z C REAL(P). C contains the cosines of the transforming rotations. S COMPLEX(P). S contains the sines of the transforming rotations.

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

- Canonical provider: `lin/cchud.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cchud.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cchud.f) — `verified_cached`
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
