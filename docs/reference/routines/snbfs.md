# SNBFS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a general nonsymmetric banded system of linear equations.

## Description

Subroutine SNBFS solves a general nonsymmetric banded NxN system of single precision real linear equations using SLATEC subroutines SNBCO and SNBSL. These are adaptations of the LINPACK subroutines SBGCO and SGBSL, which require a different format for storing the matrix elements. If A is an NxN real matrix and if X and B are real N-vectors, then SNBFS solves the equation A*X=B. A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . The integers ML and MU are called the lower and upper band widths and M = ML+MU+1 is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N . The matrix A is first factored into upper and lower triangular matrices U and L using partial pivoting. These factors and the pivoting information are used to find the solution vector X. An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N and IWORK must not have been altered by the user following factorization (ITASK=1). IND will not be changed by SNBFS in this case. Band Storage If A is a band matrix, the following program segment will set up the input. ML = (band width below the diagonal) MU = (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 ABE(I,K) = A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE . Furthermore, ML additional columns are needed in ABE starting with column ML+MU+2 for elements generated during the triangularization. The total number of columns needed in ABE is 2*ML+MU+1 . Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA .GE. 5 and ABE should contain * 11 12 13 + , * = not used 21 22 23 24 + , + = used for pivoting 32 33 34 35 + 43 44 45 46 + 54 55 56 * + 65 66 * * + Argument Description *** ABE REAL(LDA,NC) on entry, contains the matrix in band storage as described above. NC must not be less than 2*ML+MU+1 . The user is cautioned to specify NC with care since it is not an argument and cannot be checked by SNBFS. The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE . on return, contains an upper triangular matrix U and the multipliers necessary to construct a matrix L so that A=L*U. LDA INTEGER the leading dimension of array ABE. LDA must be greater than or equal to N. (terminal error message IND=-1) N INTEGER the order of the matrix A. N must be greater than or equal to 1 . (terminal error message IND=-2) ML INTEGER the number of diagonals below the main diagonal. ML must not be less than zero nor greater than or equal to N . (terminal error message IND=-5) MU INTEGER the number of diagonals above the main diagonal. MU must not be less than zero nor greater than or equal to N . (terminal error message IND=-6) V REAL(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . ITASK INTEGER If ITASK=1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A and IWORK. If ITASK .LT. 1, then terminal error message IND=-3 is printed. IND INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0 See error message corresponding to IND below. WORK REAL(N) a singly subscripted array of dimension at least N. IWORK INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal the matrix A is computationally singular. A solution has not been computed. IND=-5 terminal ML is less than zero or is greater than or equal to N . IND=-6 terminal MU is less than zero or is greater than or equal to N . IND=-10 warning the solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

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

- Canonical provider: `main-src/src/snbfs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/snbfs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/snbfs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/snbfs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ABE` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDA, *) | ML = (band width below the diagonal) MU = (band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 ABE(I,K) = A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | In this case, the contents of A, LDA, N and IWORK must not have been altered by the user following factorization (ITASK=1). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If A is an NxN real matrix and if X and B are real N-vectors, then SNBFS solves the equation A*X=B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ML` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . | A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MU` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . | A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | (terminal error message IND=-6) V REAL(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. | (terminal error message IND=-6) V REAL(N) on entry, the singly subscripted array(vector) of dimension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ITASK` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IND will not be changed by SNBFS in this case. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK REAL(N) a singly subscripted array of dimension at least N. | WORK REAL(N) a singly subscripted array of dimension at least N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | In this case, the contents of A, LDA, N and IWORK must not have been altered by the user following factorization (ITASK=1). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::banded::snbfs`. Native symbol: `snbfs_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::snbfs`
- Compatibility aliases: `slatec_sys::linear_algebra::dense::snbfs`
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
