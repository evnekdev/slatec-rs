# DULSIA

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve an underdetermined linear system of equations by performing an LQ factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. User input bounds on the uncertainty in the elements of A are used to detect numerical rank deficiency. The algorithm employs a row and column pivot strategy to minimize the growth of uncertainty and round-off errors. DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ****************************************************************** Input.. All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. M is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). Must have MDA.GE.M and M.LE.N. B(,) Right hand side(s), with MDB the actual first MDB,NB dimension of B in the calling program. NB is the number of M by 1 right hand sides. Since the solution is returned in B, must have MDB.GE.N. If NB = 0, B is never accessed. ****************************************************************** * * * Note - Use of RE and AE are what make this * * code significantly different from * * other linear least squares solvers. * * However, the inexperienced user is * * advised to set RE=0.,AE=0.,KEY=0. * * * ****************************************************************** RE(),AE(),KEY RE() RE() is a vector of length N such that RE(I) is the maximum relative uncertainty in row I of the matrix A. The values of RE() must be between 0 and 1. A minimum of 10*machine precision will be enforced. AE() AE() is a vector of length N such that AE(I) is the maximum absolute uncertainty in row I of the matrix A. The values of AE() must be greater than or equal to 0. KEY For ease of use, RE and AE may be input as either vectors or scalars. If a scalar is input, the algorithm will use that value for each column of A. The parameter KEY indicates whether scalars or vectors are being input. KEY=0 RE scalar AE scalar KEY=1 RE vector AE scalar KEY=2 RE scalar AE vector KEY=3 RE vector AE vector MODE The integer MODE indicates how the routine is to react if rank deficiency is detected. If MODE = 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol The inexperienced user is advised to set MODE=0 NP The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. The inexperienced user is advised to set NP=0. WORK() A real work array dimensioned 5*M. However, if RE or AE have been specified as vectors, dimension WORK 4*M. If both RE and AE have been specified as vectors, dimension WORK 3*M. LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. LIW Actual dimension of IWORK. INFO Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, KRANK, LW, IWORK, LIW, and the first 2*M locations of WORK as output by the original call to DULSIA. MODE must be equal to the value of MODE in the original call. If MODE.LT.2, only the first N locations of WORK are accessed. AE, RE, KEY, and NP are not accessed. Output..All TYPE REAL variables are DOUBLE PRECISION A(,) Contains the lower triangular part of the reduced matrix and the transformation information. It togeth with the first M elements of WORK (see below) completely specify the LQ factorization of A. B(,) Contains the N by NB solution matrix for X. KRANK,KSURE The numerical rank of A, based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. The algorithm returns a solution based on KRANK. KSURE provides an indication of the precision of the rank. RNORM() Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. If the matrix A is of full rank, then RNORM=0.0. WORK() The first M locations of WORK contain values necessary to reproduce the Householder transformation. IWORK() The first N locations contain the order in which the columns of A were used. The next M locations contain the order in which the rows of A were used. INFO Flag to indicate status of computation on completion -1 Parameter error(s) 0 - Rank deficient, no solution 1 - Rank deficient, truncated solution 2 - Rank deficient, minimal length least squares sol 3 - Numerical rank 0, zero solution 4 - Rank .LT. NP 5 - Full rank

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

- Canonical provider: `main-src/src/dulsia.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dulsia.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dulsia.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dulsia.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
