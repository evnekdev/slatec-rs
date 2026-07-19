# SSICS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Incompl. Cholesky Decomposition Preconditioner SLAP Set Up. Routine to generate the Incomplete Cholesky decomposition, L*D*L-trans, of a symmetric positive definite matrix, A, which is stored in SLAP Column format. The unit lower triangular matrix L is stored by rows, and the inverse of the diagonal matrix D is stored.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM INTEGER NEL, IEL(NEL), JEL(NEL), IWARN REAL A(NELT), EL(NEL), D(N), R(N) CALL SSICS( N, NELT, IA, JA, A, ISYM, NEL, IEL, JEL, EL, D, R, $ IWARN ) *Arguments: N :IN Integer. Order of the Matrix. NELT :IN Integer. Number of elements in arrays IA, JA, and A. IA :INOUT Integer IA(NELT). JA :INOUT Integer JA(NELT). A :INOUT Real A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored. NEL :OUT Integer. Number of non-zeros in the lower triangle of A. Also corresponds to the length of the IEL, JEL, EL arrays. IEL :OUT Integer IEL(NEL). JEL :OUT Integer JEL(NEL). EL :OUT Real EL(NEL). IEL, JEL, EL contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. The Diagonal of ones *IS* stored. See "Description", below for more details about the SLAP Row fmt. D :OUT Real D(N) Upon return this array holds D(I) = 1./DIAG(A). R :WORK Real R(N). Temporary real workspace needed for the factorization. IWARN :OUT Integer. This is a warning variable and is zero if the IC factorization goes well. It is set to the row index corresponding to the last zero pivot found. See "Description", below. *Description =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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

- Canonical provider: `lin/ssics.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssics.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssics.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
