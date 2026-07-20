# SGEIR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a general system of linear equations. Iterative refinement is used to obtain an error estimate.

## Description

Subroutine SGEIR solves a general NxN system of single precision linear equations using LINPACK subroutines SGEFA and SGESL. One pass of iterative refinement is used only to obtain an estimate of the accuracy. That is, if A is an NxN real matrix and if X and B are real N-vectors, then SGEIR solves the equation A*X=B. The matrix A is first factored into upper and lower triangular matrices U and L using partial pivoting. These factors and the pivoting information are used to calculate the solution, X. Then the residual vector is found and used to calculate an estimate of the relative error, IND. IND estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account any errors in the input data. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to solve only (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N, WORK, and IWORK must not have been altered by the user following factorization (ITASK=1). IND will not be changed by SGEIR in this case. Argument Description *** A REAL(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix. A is not altered by the routine. LDA INTEGER the leading dimension of the array A. LDA must be greater than or equal to N. (terminal error message IND=-1) N INTEGER the order of the matrix A. The first N elements of the array A are the elements of the first column of matrix A. N must be greater than or equal to 1. (terminal error message IND=-2) V REAL(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . ITASK INTEGER If ITASK=1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). If ITASK .LT. 1, then terminal error message IND=-3 is printed. IND INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. IND=75 means that the solution vector X is zero. LT. 0 see error message corresponding to IND below. WORK REAL(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). IWORK INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than one. IND=-3 terminal ITASK is less than one. IND=-4 terminal The matrix A is computationally singular. A solution has not been computed. IND=-10 warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

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
- GAMS classifications: `D2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `conflicting`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sgeir.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sgeir.f` (`live-main-source`)
  - `main-src-historical/src/sgeir.f.0` (`historical-variant`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sgeir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sgeir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Multiple or variant providers remain separate pending manual reconciliation.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgeir`
- Current legacy Rust paths: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
