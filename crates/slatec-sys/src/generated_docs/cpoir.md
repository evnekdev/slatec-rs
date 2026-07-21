# Purpose

Subroutine CPOIR solves a complex positive definite Hermitian NxN system of single precision linear equations using LINPACK subroutines CPOFA and CPOSL. One pass of iterative refine- ment is used only to obtain an estimate of the accuracy. That is, if A is an NxN complex positive definite Hermitian matrix and if X and B are complex N-vectors, then CPOIR solves the A*X=B. Care should be taken not to use CPOIR with a non-Hermitian matrix. The matrix A is first factored into upper and lower triangular matrices R and R-TRANSPOSE. These factors are used to calculate the solution, X. Then the residual vector is found and used to calculate an estimate of the relative error, IND. IND estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account any errors in the input data. If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, N, and WORK must not have been altered by the user following factorization (ITASK=1). IND will not be changed by CPOIR in this case. Argument Description ***

# Description

This canonical unsafe binding exposes original SLATEC routine `CPOIR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPOIR](https://www.netlib.org/slatec/src/cpoir.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix. Only the upper triangle, including the diagonal, of the coefficient matrix need be entered. A is not altered by the routine.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A. LDA must be great- er than or equal to N. (terminal error message IND=-1).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A. N must be greater than or equal to one. (terminal error message IND=-2).

## `V`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X.

## `ITASK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER if ITASK = 1, the matrix A is factored and then the linear equation is solved. if ITASK. GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). LT. 1, then terminal terminal error IND=-3 is printed.

## `IND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. IND=75 means that the solution vector X is zero. LT. 0 see error message corresponding to IND below.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (N, *).

COMPLEX(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than one. IND=-3 terminal ITASK is less than one. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. COMPLEX(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than one. IND=-3 terminal ITASK is less than one. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 warning The solution has no apparent significance. the solution may be inaccurate or the matrix a may be poorly scaled. NOTE- the above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `75` | means that the solution vector X is zero. LT. 0 see error message corresponding to IND below. |
| `IND` | `-1` | terminal N is greater than LDA. |
| `IND` | `-2` | terminal N is less than one. |
| `IND` | `-3` | terminal ITASK is less than one. |
| `IND` | `-4` | terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. |
| `IND` | `-10` | warning The solution has no apparent significance. the solution may be inaccurate or the matrix a may be poorly scaled. NOTE- the above terminal(*fatal*) error messages are designed to be handled by XERMSG in which |
| `IND` | `1` | (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `V`: not a workspace argument
- `WORK`: COMPLEX(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than one. IND=-3 terminal ITASK is less than one. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 warning The solution has no apparent significance. the solution may be inaccurate or the matrix a may be poorly scaled. NOTE- the above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpoir`
- Original SLATEC routine: `CPOIR`
- Native symbol: `cpoir_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank2)`
- Exact Netlib source file: [CPOIR](https://www.netlib.org/slatec/src/cpoir.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
