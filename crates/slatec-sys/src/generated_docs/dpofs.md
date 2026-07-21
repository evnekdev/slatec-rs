# Purpose

Subroutine DPOFS solves a positive definite symmetric NxN system of double precision linear equations using LINPACK subroutines DPOCO and DPOSL. That is, if A is an NxN double precision positive definite symmetric matrix and if X and B are double precision N-vectors, then DPOFS solves the equation A*X=B. The matrix A is first factored into upper and lower tri- angular matrices R and R-TRANPOSE. These factors are used to find the solution vector X. An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option only to solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, and N must not have been altered by the user following factorization (ITASK=1). IND will not be changed by DPOFS in this case. Argument Description ***

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOFS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOFS](https://www.netlib.org/slatec/src/dpofs.f).

# Arguments

## `A`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA,N) on entry, the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix. Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, A contains in its upper triangle an upper triangular matrix R such that A = (R-TRANPOSE) * R.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A. LDA must be great- er than or equal to N. (terminal error message IND=-1).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A. N must be greater than or equal to 1. (terminal error message IND=-2).

## `V`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X.

## `ITASK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER If ITASK = 1, the matrix A is factored and then the linear equation is solved. If ITASK. GT. 1, the equation is solved using the existing factored matrix A. LT. 1, then terminal error message IND=-3 is printed.

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0 See error message corresponding to IND below.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 Terminal N is greater than LDA. IND=-2 Terminal N is less than 1. IND=-3 Terminal ITASK is less than 1. IND=-4 Terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. DOUBLE PRECISION(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 Terminal N is greater than LDA. IND=-2 Terminal N is less than 1. IND=-3 Terminal ITASK is less than 1. IND=-4 Terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 Warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above Terminal(*fatal*) Error Messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `-1` | Terminal N is greater than LDA. |
| `IND` | `-2` | Terminal N is less than 1. |
| `IND` | `-3` | Terminal ITASK is less than 1. |
| `IND` | `-4` | Terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. |
| `IND` | `-10` | Warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above Terminal(*fatal*) Error Messages are designed to be handled by XERMSG in which |
| `IND` | `1` | (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `V`: not a workspace argument
- `WORK`: DOUBLE PRECISION(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 Terminal N is greater than LDA. IND=-2 Terminal N is less than 1. IND=-3 Terminal ITASK is less than 1. IND=-4 Terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 Warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note- The above Terminal(*fatal*) Error Messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dpofs`
- Original SLATEC routine: `DPOFS`
- Native symbol: `dpofs_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPOFS](https://www.netlib.org/slatec/src/dpofs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
