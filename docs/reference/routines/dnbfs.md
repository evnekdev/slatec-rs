# DNBFS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a general nonsymmetric banded system of linear equations.

## Description

Subroutine DNBFS solves a general nonsymmetric banded NxN system of double precision real linear equations using SLATEC subroutines DNBCO and DNBSL. These are adaptations of the LINPACK subroutines DGBCO and DGBSL which require a different format for storing the matrix elements. If A is an NxN double precision matrix and if X and B are double precision N-vectors, then DNBFS solves the equation A*X=B. A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . The integers ML and MU are called the lower and upper band widths and M = ML+MU+1 is the total band width. DNBFS uses less time and storage than the corresponding program for general matrices (DGEFS) if 2*ML+MU .LT. N . The matrix A is first factored into upper and lower triangular matrices U and L using partial pivoting. These factors and the pivoting information are used to find the solution vector X. An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N and IWORK must not have been altered by the user following factorization (ITASK=1). IND will not be changed by DNBFS in this case. Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 ABE(I,K) = A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE . Furthermore, ML additional columns are needed in ABE starting with column ML+MU+2 for elements generated during the triangularization. The total number of columns needed in ABE is 2*ML+MU+1 . Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA .GE. 5 and ABE should contain * 11 12 13 + , * = not used 21 22 23 24 + , + = used for pivoting 32 33 34 35 + 43 44 45 46 + 54 55 56 * + 65 66 * * + Argument Description *** ABE DOUBLE PRECISION(LDA,NC) on entry, contains the matrix in band storage as described above. NC must not be less than 2*ML+MU+1 . The user is cautioned to specify NC with care since it is not an argument and cannot be checked by DNBFS. The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. LDA INTEGER the leading dimension of array ABE. LDA must be greater than or equal to N. (terminal error message IND=-1) N INTEGER the order of the matrix A. N must be greater than or equal to 1 . (terminal error message IND=-2) ML INTEGER the number of diagonals below the main diagonal. ML must not be less than zero nor greater than or equal to N . (terminal error message IND=-5) MU INTEGER the number of diagonals above the main diagonal. MU must not be less than zero nor greater than or equal to N . (terminal error message IND=-6) V DOUBLE PRECISION(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . ITASK INTEGER If ITASK=1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A and IWORK. If ITASK .LT. 1, then terminal error message IND=-3 is printed. IND INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0 See error message corresponding to IND below. WORK DOUBLE PRECISION(N) a singly subscripted array of dimension at least N. IWORK INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal The matrix A is computationally singular. A solution has not been computed. IND=-5 terminal ML is less than zero or is greater than or equal to N . IND=-6 terminal MU is less than zero or is greater than or equal to N . IND=-10 warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

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
- GAMS classifications: `D2A2`
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

- Canonical provider: `main-src/src/dnbfs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnbfs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnbfs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnbfs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dnbfs`
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
