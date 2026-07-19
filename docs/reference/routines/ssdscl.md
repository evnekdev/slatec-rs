# SSDSCL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Diagonal Scaling of system Ax = b. This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling.

## Description

This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling. The new system is: -1/2 -1/2 1/2 -1/2 D AD (D x) = D b when scaling is selected with the JOB parameter. When unscaling is selected this process is reversed. The true solution is also scaled or unscaled if ITOL is set appropriately, see below. *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, JOB, ITOL REAL A(NELT), X(N), B(N), DINV(N) CALL SSDSCL( N, NELT, IA, JA, A, ISYM, X, B, DINV, JOB, ITOL ) *Arguments: N :IN Integer Order of the Matrix. NELT :IN Integer. Number of elements in arrays IA, JA, and A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Real A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. X :INOUT Real X(N). Initial guess that will be later used in the iterative solution. of the scaled system. B :INOUT Real B(N). Right hand side vector. DINV :INOUT Real DINV(N). Upon return this array holds 1./DIAG(A). This is an input if JOB = 0. JOB :IN Integer. Flag indicating whether to scale or not. JOB non-zero means do scaling. JOB = 0 means do unscaling. ITOL :IN Integer. Flag indicating what type of error estimation to do in the iterative method. When ITOL = 11 the exact solution from common block SSLBLK will be used. When the system is scaled then the true solution must also be scaled. If ITOL is not 11 then this vector is not referenced. *Common Blocks: SOLN :INOUT Real SOLN(N). COMMON BLOCK /SSLBLK/ The true solution, SOLN, is scaled (or unscaled) if ITOL is set to 11, see above. *Description =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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

- Canonical provider: `lin/ssdscl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssdscl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
