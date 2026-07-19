# SSLUI2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

SLAP Backsolve for LDU Factorization. Routine to solve a system of the form L*D*U X = B, where L is a unit lower triangular matrix, D is a diagonal matrix, and U is a unit upper triangular matrix.

## Description

*Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSLUI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Right hand side. X :OUT Real X(N). Solution of L*D*U x = b. IL :IN Integer IL(NL). JL :IN Integer JL(NL). L :IN Real L(NL). IL, JL, L contain the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array.) DINV :IN Real DINV(N). Inverse of the diagonal matrix D. IU :IN Integer IU(NU). JU :IN Integer JU(NU). U :IN Real U(NU). IU, JU, U contain the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array.) *Description: This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SIR and SBCG iteration routines for the drivers SSILUR and SSLUBC. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSLUI. **** THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** **** SLAP MSOLVE CALLING CONVENTION **** IL, JL, L should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. IU, JU, U should contain the unit upper factor of the incomplete decomposition of the A matrix stored in SLAP Column format This ILU factorization can be computed by the SSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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
- GAMS classifications: `D2E`
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

- Canonical provider: `lin/sslui2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sslui2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
