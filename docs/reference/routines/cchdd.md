# CCHDD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Downdate an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition.

## Description

CCHDD downdates an augmented Cholesky decomposition or the triangular factor of an augmented QR decomposition. Specifically, given an upper triangular matrix R of order P, a row vector X, a column vector Z, and a scalar Y, CCHDD determines a unitary matrix U and a scalar ZETA such that (R Z ) (RR ZZ) U * ( ) = ( ) , (0 ZETA) ( X Y) where RR is upper triangular. If R and Z have been obtained from the factorization of a least squares problem, then RR and ZZ are the factors corresponding to the problem with the observation (X,Y) removed. In this case, if RHO is the norm of the residual vector, then the norm of the residual vector of the downdated problem is SQRT(RHO**2 - ZETA**2). CCHDD will simultaneously downdate several triplets (Z,Y,RHO) along with R. For a less terse description of what CCHDD does and how it may be applied, see the LINPACK Guide. The matrix U is determined as the product U(1)*...*U(P) where U(I) is a rotation in the (P+1,I)-plane of the form ( C(I) -CONJG(S(I)) ) ( ) . ( S(I) C(I) ) the rotations are chosen so that C(I) is real. The user is warned that a given downdating problem may be impossible to accomplish or may produce inaccurate results. For example, this can happen if X is near a vector whose removal will reduce the rank of R. Beware. On Entry R COMPLEX(LDR,P), where LDR .GE. P. R contains the upper triangular matrix that is to be downdated. The part of R below the diagonal is not referenced. LDR INTEGER. LDR is the leading dimension of the array R. p INTEGER. P is the order of the matrix R. X COMPLEX(P). X contains the row vector that is to be removed from R. X is not altered by CCHDD. Z COMPLEX(LDZ,NZ), where LDZ .GE. P. Z is an array of NZ P-vectors which are to be downdated along with R. LDZ INTEGER. LDZ is the leading dimension of the array Z. NZ INTEGER. NZ is the number of vectors to be downdated NZ may be zero, in which case Z, Y, and RHO are not referenced. Y COMPLEX(NZ). Y contains the scalars for the downdating of the vectors Z. Y is not altered by CCHDD. RHO REAL(NZ). RHO contains the norms of the residual vectors that are to be downdated. On Return R Z contain the downdated quantities. RHO C REAL(P). C contains the cosines of the transforming rotations. S COMPLEX(P). S contains the sines of the transforming rotations. INFO INTEGER. INFO is set as follows. INFO = 0 if the entire downdating was successful. INFO =-1 if R could not be downdated. in this case, all quantities are left unaltered. INFO = 1 if some RHO could not be downdated. The offending RHO's are set to -1.

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

- Canonical provider: `lin/cchdd.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cchdd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cchdd.f) — `verified_cached`
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
