# Purpose

Subroutine SPOFS solves a real positive definite symmetric NxN system of single precision linear equations using LINPACK subroutines SPOCO and SPOSL. That is, if A is an NxN real positive definite symmetric matrix and if X and B

# Description

This canonical unsafe binding exposes original SLATEC routine `SPOFS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPOFS](https://www.netlib.org/slatec/src/spofs.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). B. angular matrices R and R-TRANSPOSE.  These factors are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to solve only (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) on entry, the doubly subscripted array with dimension TRANSPOSE) * R . must be greater B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N. B. angular matrices R and R-TRANSPOSE.  These factors are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to solve only (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) on entry, the doubly subscripted array with dimension TRANSPOSE) * R . must be greater B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N. not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper INTEGER must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper INTEGER must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper INTEGER not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors, then SPOFS solves the equation must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper 1) INTEGER must be greater vectors, then SPOFS solves the equation must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper 1) INTEGER must be greater not applicable or not stated by selected source not a workspace argument

## 4. `V`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ITASK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1).  IND will not be changed by SPOFS in this case. Argument Description *** INTEGER 1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A. 3 is printed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1) 2) 3 is printed. INTEGER GT. 0  IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0  see error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than 1. 3  terminal   ITASK is less than 1. 4  Terminal   The matrix A is computationally singular or is not positive definite.  A solution has not been computed. 10 warning    The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) not stated by selected source not applicable or not stated by selected source

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
- `WORK`: REAL(N)

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::spofs`
- Original SLATEC routine: `SPOFS`
- Native symbol: `spofs_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SPOFS](https://www.netlib.org/slatec/src/spofs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
