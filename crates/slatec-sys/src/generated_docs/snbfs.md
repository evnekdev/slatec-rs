# Purpose

Subroutine SNBFS solves a general nonsymmetric banded NxN system of single precision real linear equations using SLATEC subroutines SNBCO and SNBSL. These are adaptations of the LINPACK subroutines SBGCO and SGBSL, which require a different format for storing the matrix elements. If A is an NxN real matrix and if X and B are real N-vectors, then SNBFS solves the equation A*X=B. A band matrix is a matrix whose nonzero elements are all fairly near the main diagonal, specifically A(I,J) = 0 if I-J is greater than ML or J-I is greater than MU . The integers ML and MU are called the lower and upper band widths and M = ML+MU+1 is the total band width. SNBFS uses less time and storage than the corresponding program for general matrices (SGEFS) if 2*ML+MU .LT. N . The matrix A is first factored into upper and lower tri- angular matrices U and L using partial pivoting. These factors and the pivoting information are used to find the solution vector X. An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N and IWORK must not have been altered by the user follow- ing factorization (ITASK=1). IND will not be changed by SNBFS in this case. Band Storage If A is a band matrix, the following program segment will set up the input.

# Description

This canonical unsafe binding exposes original SLATEC routine `SNBFS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SNBFS](https://www.netlib.org/slatec/src/snbfs.f).

# Arguments

## `ABE`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

A(I,J) 10 CONTINUE 20 CONTINUE This uses columns 1 through ML+MU+1 of ABE. Furthermore, ML additional columns are needed in starting with column ML+MU+2 for elements generated during the triangularization. The total number of columns needed in ABE is 2*ML+MU+1. Example: If the original matrix is 11 12 13 0 0 0 21 22 23 24 0 0 0 32 33 34 35 0 0 0 43 44 45 46 0 0 0 54 55 56 0 0 0 0 65 66 then N = 6, ML = 1, MU = 2, LDA. GE. 5 and ABE should contain 11 12 13 + , * = not used 21 22 23 24 + , + = used for pivoting 32 33 34 35 + 43 44 45 46 + 54 55 56 * + 65 66 * * + Argument Description *** REAL(LDA,NC) on entry, contains the matrix in band storage as described above.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of array ABE. LDA must be great- er than or equal to N. (terminal error message IND=-1).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A. N must be greater than or equal to 1. (terminal error message IND=-2).

## `ML`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(band width below the diagonal) INTEGER the number of diagonals below the main diagonal. must not be less than zero nor greater than or equal to N. (terminal error message IND=-5).

## `MU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(band width above the diagonal) DO 20 I = 1, N J1 = MAX(1, I-ML) J2 = MIN(N, I+MU) DO 10 J = J1, J2 K = J - I + ML + 1 INTEGER the number of diagonals above the main diagonal. must not be less than zero nor greater than or equal to N. (terminal error message IND=-6).

## `V`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X.

## `ITASK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER If ITASK=1, the matrix A is factored and then the linear equation is solved. If ITASK. GT. 1, the equation is solved using the existing factored matrix A and IWORK. LT. 1, then terminal error message IND=-3 is printed.

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0 See error message corresponding to IND below.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) a singly subscripted array of dimension at least N.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal the matrix A is computationally singular. A solution has not been computed. INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal the matrix A is computationally singular. A solution has not been computed. IND=-5 terminal ML is less than zero or is greater than or equal to N . IND=-6 terminal MU is less than zero or is greater than IND=-10 warning the solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `-1` | terminal N is greater than LDA. |
| `IND` | `-2` | terminal N is less than 1. |
| `IND` | `-3` | terminal ITASK is less than 1. |
| `IND` | `-4` | terminal the matrix A is computationally singular. A solution has not been computed. |
| `IND` | `-5` | terminal ML is less than zero or is greater than or equal to N . |
| `IND` | `-6` | terminal MU is less than zero or is greater than or equal to N . |
| `IND` | `-10` | warning the solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which |
| `IND` | `1` | (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort. |

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `V`: not a workspace argument
- `WORK`: REAL(N) a singly subscripted array of dimension at least N.
- `IWORK`: INTEGER(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal the matrix A is computationally singular. A solution has not been computed. IND=-5 terminal ML is less than zero or is greater than or equal to N . IND=-6 terminal MU is less than zero or is greater than IND=-10 warning the solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::snbfs`
- Original SLATEC routine: `SNBFS`
- Native symbol: `snbfs_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [SNBFS](https://www.netlib.org/slatec/src/snbfs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
