# DGLSS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the input matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. If M.GE.N, the least squares solution is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factorization). If the matrix A is determined to be rank deficient, that is the rank of A is less than MIN(M,N), then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. M is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). B(,) Right hand side(s), with MDB the actual first MDB,NB dimension of B in the calling program. NB is the number of M by 1 right hand sides. Must have MDB.GE.MAX(M,N). If NB = 0, B is never accessed. RNORM() Vector of length at least NB. On input the contents of RNORM are unused. WORK() A real work array dimensioned 5*MIN(M,N). LW Actual dimension of WORK. IWORK() Integer work array dimensioned at least N+M. LIW Actual dimension of IWORK. INFO A flag which provides for the efficient solution of subsequent problems involving the same A but different B. If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, INFO, LW, IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to DGLSS. Output..All TYPE REAL variables are DOUBLE PRECISION A(,) Contains the triangular part of the reduced matrix and the transformation information. It together with the first 2*MIN(M,N) elements of WORK (see below) completely specify the factorization of A. B(,) Contains the N by NB solution matrix X. RNORM() Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. WORK() The first 2*MIN(M,N) locations of WORK contain value necessary to reproduce the factorization of A. IWORK() The first M+N locations contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. INFO Flag to indicate status of computation on completion -1 Parameter error(s) 0 - Full rank N.GT.0 - Reduced rank rank=MIN(M,N)-INFO

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dglss.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dglss.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dglss.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dglss.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
