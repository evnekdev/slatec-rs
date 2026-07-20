# DULSIA

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 2; dimensions (MDA, *) | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. | DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 2; dimensions (MDB, *) | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. | DULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NB` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | DULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RE` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | ****************************************************************** * * * Note - Use of RE and AE are what make this * * code significantly different from * * other linear least squares solvers. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AE` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | ****************************************************************** * * * Note - Use of RE and AE are what make this * * code significantly different from * * other linear least squares solvers. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KEY` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * However, the inexperienced user is * * advised to set RE=0.,AE=0.,KEY=0. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | KEY=0 RE scalar AE scalar KEY=1 RE vector AE scalar KEY=2 RE scalar AE vector KEY=3 RE vector AE vector MODE The integer MODE indicates how the routine is to react if rank deficiency is detected. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NP` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | If MODE = 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol The inexperienced user is advised to set MODE=0 NP The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. | If MODE = 0 return immediately, no solution 1 compute truncated solution 2 compute minimal length least squares sol The inexperienced user is advised to set MODE=0 NP The first NP rows of A will not be interchanged with other rows even though the pivot strategy would suggest otherwise. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KRANK` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, KRANK, LW, IWORK, LIW, and the first 2*M locations of WORK as output by the original call to DULSIA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KSURE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | KRANK,KSURE The numerical rank of A, based upon the relative and absolute bounds on uncertainty, is bounded above by KRANK and below by KSURE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | RNORM() Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. | RNORM() Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. | LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. Leading dimension: not established Workspace: LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. | LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. Leading dimension: not established Workspace: LW Actual dimension of WORK IWORK() Integer work array dimensioned at least N+M. | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | LIW Actual dimension of IWORK. | LIW Actual dimension of IWORK. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | INFO Is a flag which provides for the efficient solution of subsequent problems involving the same A but different B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::dulsia`. Native symbol: `dulsia_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dulsia`
- Compatibility aliases: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
