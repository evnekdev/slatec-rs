# SCPPLT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Printer Plot of SLAP Column Format Matrix. Routine to print out a SLAP Column format matrix in a "printer plot" graphical representation.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT REAL A(NELT) CALL SCPPLT( N, NELT, IA, JA, A, ISYM, IUNIT ) *Arguments: N :IN Integer Order of the Matrix. If N.gt.MAXORD, only the leading MAXORD x MAXORD submatrix will be printed. (Currently MAXORD = 225.) NELT :IN Integer. Number of non-zeros stored in A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Real A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored. IUNIT :IN Integer. Fortran logical I/O device unit number to write the matrix to. This unit must be connected in a system dependent fashion to a file or the console or you will get a nasty message from the Fortran I/O libraries. *Description: This routine prints out a SLAP Column format matrix to the Fortran logical I/O unit number IUNIT. The numbers them selves are not printed out, but rather a one character representation of the numbers. Elements of the matrix that are not represented in the (IA,JA,A) arrays are denoted by ' ' character (a blank). Elements of A that are *ZERO* (and hence should really not be stored) are denoted by a '0' character. Elements of A that are *POSITIVE* are denoted by 'D' if they are Diagonal elements and '#' if they are off Diagonal elements. Elements of A that are *NEGATIVE* are denoted by 'N' if they are Diagonal elements and '*' if they are off Diagonal elements. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `unknown`
- GAMS classifications: `N1`
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

- Canonical provider: `lin/scpplt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/scpplt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
