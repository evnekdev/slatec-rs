# TRBAK1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED1.

## Description

This subroutine is a translation of the ALGOL procedure TRBAK1, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a REAL SYMMETRIC matrix by back transforming those of the corresponding symmetric tridiagonal matrix determined by TRED1. On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. A contains information about the orthogonal transformations used in the reduction by TRED1 in its strict lower triangle. A is a two-dimensional REAL array, dimensioned A(NM,N). E contains the subdiagonal elements of the tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. These elements provide the remaining information about the orthogonal transformations. E is a one-dimensional REAL array, dimensioned E(N). M is the number of columns of Z to be back transformed. M is an INTEGER variable. Z contains the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array, dimensioned Z(NM,M). On Output Z contains the transformed eigenvectors in its first M columns. Note that TRBAK1 preserves vector Euclidean norms. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C4`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/trbak1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/trbak1.f) — `verified_cached`
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
