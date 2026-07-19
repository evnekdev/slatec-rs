# ISSBCG

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Preconditioned BiConjugate Gradient Stop Test. This routine calculates the stop test for the BiConjugate Gradient iteration scheme. It returns a non-zero if the error estimate (the type of which is determined by ITOL) is less than the user specified tolerance TOL.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX, ITER INTEGER IERR, IUNIT, IWORK(USER DEFINED) REAL B(N), X(N), A(N), TOL, ERR, R(N), Z(N), P(N) REAL RR(N), ZZ(N), PP(N), DZ(N) REAL RWORK(USER DEFINED), AK, BK, BNRM, SOLNRM EXTERNAL MSOLVE IF( ISSBCG(N, B, X, NELT, IA, JA, A, ISYM, MSOLVE, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, R, Z, P, RR, ZZ, PP, DZ, $ RWORK, IWORK, AK, BK, BNRM, SOLNRM) .NE. 0 ) $ THEN ITERATION DONE *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Right-hand side vector. X :INOUT Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. NELT :IN Integer. Number of Non-Zeros stored in A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Real A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description", in the SLAP routine SBCG for more details. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MSOLVE :EXT External. Name of a routine which solves a linear system MZ = R for Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays). The name of the MSOLVE routine must be declared external in the calling program. The

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `integer_or_index`
- Scalar kind: `integer`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/issbcg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/issbcg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/issbcg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
