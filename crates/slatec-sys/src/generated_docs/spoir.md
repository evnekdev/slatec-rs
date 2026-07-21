# Purpose

Subroutine SPOIR solves a real positive definite symmetric NxN system of single precision linear equations using LINPACK subroutines SPOFA and SPOSL. One pass of iterative refine- ment is used only to obtain an estimate of the accuracy. That is, if A is an NxN real positive definite symmetric matrix

# Description

This canonical unsafe binding exposes original SLATEC routine `SPOIR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPOIR](https://www.netlib.org/slatec/src/spoir.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). B. The matrix A is first factored into upper and lower triangular matrices R and R-TRANSPOSE.  These factors are used to calculate the solution, X. Then the residual vector is found and used to calculate an estimate of the relative error, IND. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered.  A is not altered by the routine. must be greater than B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N*(N+1). solution has not been computed. may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. B. The matrix A is first factored into upper and lower triangular matrices R and R-TRANSPOSE.  These factors are used to calculate the solution, X. Then the residual vector is found and used to calculate an estimate of the relative error, IND. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered.  A is not altered by the routine. must be greater than B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N*(N+1). solution has not been computed. may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. must not have been altered by the user INTEGER must not have been altered by the user INTEGER must not have been altered by the user INTEGER not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors, then SPOIR solves the equation must not have been altered by the user 1) INTEGER must be greater than vectors, then SPOIR solves the equation must not have been altered by the user 1) INTEGER must be greater than not applicable or not stated by selected source not a workspace argument

## 4. `V`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ITASK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1).  IND will not be changed by SPOIR in this case. Argument Description *** INTEGER 1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). 3 is printed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account any errors in the input data. 1) 2) 3 is printed. INTEGER GT. 0  IND is a rough estimate of the number of digits 75 means that the solution vector X is zero. LT. 0  See error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than one. 3  terminal   ITASK is less than one. 4  Terminal   The matrix A is computationally singular or is not positive definite. 10 warning    The solution has no apparent significance. The solution may be inaccurate or the matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (N, *). must not have been altered by the user REAL(N*(N+1)) must not have been altered by the user REAL(N*(N+1)) not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `V`: not a workspace argument
- `ITASK`: not a workspace argument
- `IND`: not a workspace argument
- `WORK`: must not have been altered by the user REAL(N*(N+1))

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::spoir`
- Original SLATEC routine: `SPOIR`
- Native symbol: `spoir_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [SPOIR](https://www.netlib.org/slatec/src/spoir.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
