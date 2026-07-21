# Purpose

Subroutine CGEFS solves A general NxN system of complex linear equations using LINPACK subroutines CGECO and CGESL. That is, if A is an NxN complex matrix

# Description

This canonical unsafe binding exposes original SLATEC routine `CGEFS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGEFS](https://www.netlib.org/slatec/src/cgefs.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). B. angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, COMPLEX(LDA,N) on entry, the doubly subscripted array with dimension L*U. must be greater than or equal to 1. B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N. singly subscripted array of dimension at least N. solution has not been computed. may be poorly scaled. NOTE-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. B. angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, COMPLEX(LDA,N) on entry, the doubly subscripted array with dimension L*U. must be greater than or equal to 1. B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N. singly subscripted array of dimension at least N. solution has not been computed. may be poorly scaled. NOTE-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. contains the coefficient matrix. on return, an upper triangular matrix U and the multipliers necessary to construct a matrix L INTEGER not stated by selected source contains the coefficient matrix. on return, an upper triangular matrix U and the multipliers necessary to construct a matrix L INTEGER not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. vectors, then CGEFS solves the equation contains the coefficient matrix. on return, an upper triangular matrix U and the multipliers necessary to construct a matrix L 1) INTEGER the order of the matrix A.  The first N elements of the array A are the elements of the first column of must be greater than or equal to 1. vectors, then CGEFS solves the equation contains the coefficient matrix. on return, an upper triangular matrix U and the multipliers necessary to construct a matrix L 1) INTEGER the order of the matrix A.  The first N elements of the array A are the elements of the first column of must be greater than or equal to 1. not applicable or not stated by selected source not a workspace argument

## 4. `V`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ITASK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1).  IND will not be changed by CGEFS in this case. Argument Description *** INTEGER 1, the matrix A is factored and then the linear equation is solved. if ITASK .GT. 1, the equation is solved using the existing factored matrix A and IWORK. 3 is printed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1) 2) 3 is printed. INTEGER GT.0  IND is a rough estimate of the number of digits of accuracy in the solution, X. LT.0  see error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than 1. 3  terminal   ITASK is less than 1. 4  terminal   The matrix A is computationally singular. 10 warning    The solution has no apparent significance. The solution may be inaccurate or the matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `WORK`

workspace `workspace` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) not stated by selected source not applicable or not stated by selected source

## 8. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) not stated by selected source not applicable or not stated by selected source

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
- `WORK`: COMPLEX(N)
- `IWORK`: INTEGER(N)

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cgefs`
- Original SLATEC routine: `CGEFS`
- Native symbol: `cgefs_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32_array_rank1)`
- Exact Netlib source file: [CGEFS](https://www.netlib.org/slatec/src/cgefs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
