# CPOIR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a positive definite Hermitian system of linear equations. Iterative refinement is used to obtain an error estimate.

## Description

Subroutine CPOIR solves a complex positive definite Hermitian NxN system of single precision linear equations using LINPACK subroutines CPOFA and CPOSL. One pass of iterative refinement is used only to obtain an estimate of the accuracy. That is, if A is an NxN complex positive definite Hermitian matrix and if X and B are complex N-vectors, then CPOIR solves the equation A*X=B. Care should be taken not to use CPOIR with a non-Hermitian matrix. The matrix A is first factored into upper and lower triangular matrices R and R-TRANSPOSE. These factors are used to calculate the solution, X. Then the residual vector is found and used to calculate an estimate of the relative error, IND. IND estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account any errors in the input data. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N, and WORK must not have been altered by the user following factorization (ITASK=1). IND will not be changed by CPOIR in this case. Argument Description *** A COMPLEX(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix. Only the upper triangle, including the diagonal, of the coefficient matrix need be entered. A is not altered by the routine. LDA INTEGER the leading dimension of the array A. LDA must be greater than or equal to N. (terminal error message IND=-1) N INTEGER the order of the matrix A. N must be greater than or equal to one. (terminal error message IND=-2) V COMPLEX(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . ITASK INTEGER if ITASK = 1, the matrix A is factored and then the linear equation is solved. if ITASK .GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). if ITASK .LT. 1, then terminal terminal error IND=-3 is printed. IND INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. IND=75 means that the solution vector X is zero. LT. 0 see error message corresponding to IND below. WORK COMPLEX(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than one. IND=-3 terminal ITASK is less than one. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 warning The solution has no apparent significance. the solution may be inaccurate or the matrix a may be poorly scaled. NOTE- the above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1B`
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

- Canonical provider: `main-src/src/cpoir.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cpoir.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cpoir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cpoir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
