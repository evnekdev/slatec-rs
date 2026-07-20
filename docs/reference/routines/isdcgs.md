# ISDCGS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Preconditioned BiConjugate Gradient Squared Stop Test. This routine calculates the stop test for the BiConjugate Gradient Squared iteration scheme. It returns a non-zero if the error estimate (the type of which is determined by ITOL) is less than the user specified tolerance TOL.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX, ITER INTEGER IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(N), TOL, ERR, R(N), R0(N), P(N) DOUBLE PRECISION Q(N), U(N), V1(N), V2(N) DOUBLE PRECISION RWORK(USER DEFINED), AK, BK, BNRM, SOLNRM EXTERNAL MATVEC, MSOLVE IF( ISDCGS(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, ITOL, $ TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, R0, P, Q, U, V1, $ V2, RWORK, IWORK, AK, BK, BNRM, SOLNRM) .NE. 0 ) $ THEN ITERATION DONE *Arguments: N :IN Integer Order of the Matrix. B :IN Double Precision B(N). Right-hand side vector. X :INOUT Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. NELT :IN Integer. Number of Non-Zeros stored in A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Double Precision A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description" in SLAP routine DCGS for more details. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT External. Name of a routine which performs the matrix vector multiply operation Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence of MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return, X is an input vector. NELT, IA, JA, A, and ISYM define the SLAP matrix data structure. MSOLVE :EXT External. Name of a routine which solves a linear system MZ = R for Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays). The name of the MSOLVE routine must be declared external in the calling program. The

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

- Canonical provider: `lin/isdcgs.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/isdcgs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/isdcgs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
