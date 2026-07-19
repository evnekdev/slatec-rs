# DBNFAC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBINT4 and DBINTK

## Description

DBNFAC is the BANFAC routine from * A Practical Guide to Splines * by C. de Boor DBNFAC is a double precision routine Returns in W the LU-factorization (without pivoting) of the banded matrix A of order NROW with (NBANDL + 1 + NBANDU) bands or diagonals in the work array W . ***** I N P U T ****** W is double precision W.....Work array of size (NROWW,NROW) containing the interesting part of a banded matrix A , with the diagonals or bands of A stored in the rows of W , while columns of A correspond to columns of W . This is the storage mode used in LINPACK and results in efficient innermost loops. Explicitly, A has NBANDL bands below the diagonal + 1 (main) diagonal + NBANDU bands above the diagonal and thus, with MIDDLE = NBANDU + 1, A(I+J,J) is in W(I+MIDDLE,J) for I=-NBANDU,...,NBANDL J=1,...,NROW . For example, the interesting entries of A (1,2)-banded matrix of order 9 would appear in the first 1+1+2 = 4 rows of W as follows. 13 24 35 46 57 68 79 12 23 34 45 56 67 78 89 11 22 33 44 55 66 77 88 99 21 32 43 54 65 76 87 98 All other entries of W not identified in this way with an entry of A are never referenced . NROWW.....Row dimension of the work array W . must be .GE. NBANDL + 1 + NBANDU . NBANDL.....Number of bands of A below the main diagonal NBANDU.....Number of bands of A above the main diagonal . ***** O U T P U T ****** W is double precision IFLAG.....Integer indicating success( = 1) or failure ( = 2) . If IFLAG = 1, then W.....contains the LU-factorization of A into a unit lower triangular matrix L and an upper triangular matrix U (both banded) and stored in customary fashion over the corresponding entries of A . This makes it possible to solve any particular linear system A*X = B for X by a CALL DBNSLV ( W, NROWW, NROW, NBANDL, NBANDU, B ) with the solution X contained in B on return . If IFLAG = 2, then one of NROW-1, NBANDL,NBANDU failed to be nonnegative, or else one of the potential pivots was found to be zero indicating that A does not have an LU-factorization. This implies that A is singular in case it is totally positive .

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBINT4, DBINTK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbnfac.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbnfac.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbnfac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
