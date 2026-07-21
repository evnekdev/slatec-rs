# Purpose

This subroutine is a translation of the ALGOL procedure MINFIT, NUM. MATH. 14, 403-420(1970) by Golub and Reinsch. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 134-151(1971). This subroutine determines, towards the solution of the linear T system AX=B, the singular value decomposition A=USV of a real T

# Description

This canonical unsafe binding exposes original SLATEC routine `MINFIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [MINFIT](https://www.netlib.org/slatec/lin/minfit.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and B, as declared in the calling program dimension statement.  Note that NM must be at least is an INTEGER variable. dimensional array parameters, A and B, as declared in the calling program dimension statement.  Note that NM must be at least is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. by N rectangular matrix, forming U B rather than U.  Householder bidiagonalization and a variant of the QR algorithm are used. is an INTEGER variable. is the number of rows of A and B.  M is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable. is the number of columns of A and the order of V.  N is an INTEGER variable. negative) singular values of A (the diagonal elements of S).  They are unordered.  If an dimensional REAL array, dimensioned W(N). T is an INTEGER variable. is the number of columns of A and the order of V.  N is an INTEGER variable. negative) singular values of A (the diagonal elements of S).  They are unordered.  If an dimensional REAL array, dimensioned W(N). T not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the rectangular coefficient matrix of the system. dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,IP). has been overwritten by the matrix V (orthogonal) of the decomposition in its first N rows and columns.  If an dimensional REAL array, dimensioned W(N). T dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the rectangular coefficient matrix of the system. dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,IP). has been overwritten by the matrix V (orthogonal) of the decomposition in its first N rows and columns.  If an dimensional REAL array, dimensioned W(N). T dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 5. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). negative) singular values of A (the diagonal elements of S).  They are unordered.  If an dimensional REAL array, dimensioned W(N). T negative) singular values of A (the diagonal elements of S).  They are unordered.  If an dimensional REAL array, dimensioned W(N). T not applicable or not stated by selected source

## 6. `IP`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of columns of B.  IP can be zero. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, IP). contains the constant column matrix of the system if IP is dimensional REAL array, dimensioned B(NM,IP). dimensional REAL array, dimensioned B(NM,IP). is made, is made, T T the rows of U B corresponding to indices of correct singular the rows of U B corresponding to indices of correct singular values should be correct. values should be correct. system Routines - EISPACK Guide, Springer-Verlag, 1976. contains the constant column matrix of the system if IP is dimensional REAL array, dimensioned B(NM,IP). dimensional REAL array, dimensioned B(NM,IP). is made, is made, T T the rows of U B corresponding to indices of correct singular the rows of U B corresponding to indices of correct singular values should be correct. values should be correct. system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional dimensional REAL array, dimensioned W(N). REAL array, dimensioned W(N). T T is an INTEGER flag set to Zero       for normal return, K          if the K-th singular value has not been determined after 30 iterations. The singular values should be correct for indices IERR+1, IERR+2, ..., N. dimensional dimensional REAL array, dimensioned W(N). REAL array, dimensioned W(N). T T is an INTEGER flag set to Zero       for normal return, K          if the K-th singular value has not been determined after 30 iterations. The singular values should be correct for indices IERR+1, IERR+2, ..., N. not applicable or not stated by selected source not a workspace argument

## 9. `RV1`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

indices of correct singular values should be correct.

# Workspace and array requirements

- `NM`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `W`: negative) singular values of A (the diagonal elements of S).  They are unordered.  If an dimensional REAL array, dimensioned W(N). T
- `IP`: not a workspace argument
- `B`: not a workspace argument
- `IERR`: not a workspace argument
- `RV1`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::minfit`
- Original SLATEC routine: `MINFIT`
- Native symbol: `minfit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [MINFIT](https://www.netlib.org/slatec/lin/minfit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
