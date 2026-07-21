# Purpose

This subroutine finds those eigenvectors of a REAL SYMMETRIC BAND matrix corresponding to specified eigenvalues, using inverse iteration. The subroutine may also be used to solve systems of linear equations with a symmetric or non-symmetric band coefficient matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `BANDV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BANDV](https://www.netlib.org/slatec/lin/bandv.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `MBW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of columns of the array A used to store the band matrix. If the matrix is symmetric, MBW is its (half) band width, denoted MB and defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, it must however have the same number of adjacent diagonals above the main diagonal as below, and in this case, MBW=2*MB-1. MBW is an INTEGER variable. MB must not be greater than N.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the lower triangle of the symmetric band input matrix stored as an N by MB array. Its lowest subdiagonal is stored in the last N+1-MB positions of the first column, its next subdiagonal in the last N+2-MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of column MB. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, A is N by 2*MB-1 instead with lower triangle as above and with its first superdiagonal stored in the first N-1 positions of column MB+1, its second superdiagonal in the first N-2 positions of column MB+2, further superdiagonals similarly, and finally its highest superdiagonal in the first N+1-MB positions of the last column. Contents of storage locations not part of the matrix are arbitrary. A is a two-dimensional REAL array, dimensioned A(NM,MBW). unaltered.

## `E21`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

specifies the ordering of the eigenvalues and contains 0. 0E0 if the eigenvalues are in ascending order, or 2. 0E0 if the eigenvalues are in descending order. If the subroutine is being used to solve systems of linear equations, E21 should be set to 1. 0E0 if the coefficient matrix is symmetric and to -1. 0E0 if not.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of specified eigenvalues or the number of systems of linear equations. M is an INTEGER variable.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear equations (A-W(J)*I)*X(J)=B(J), where I is the identity matrix, W(J) should be set accordingly, for J=1,2,. ,M. is a one-dimensional REAL array, dimensioned W(M). unaltered.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the constant matrix columns (B(J),J=1,2,. ,M), if the subroutine is used to solve systems of linear equations. is a two-dimensional REAL array, dimensioned Z(NM,M). contains the associated set of orthogonal eigenvectors. Any vector which fails to converge is set to zero. If the subroutine is used to solve systems of linear equations, contains the solution matrix columns (X(J),J=1,2,.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, -J if the eigenvector corresponding to the J-th eigenvalue fails to converge, or if the J-th system of linear equations is nearly singular.

## `NV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable.

## `RV`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays. If the subroutine is being used to solve systems of linear equations, the determinant (up to sign) of A-W(M)*I is available, upon return, as the product of the first N elements of RV. one-dimensional REAL arrays. Note that RV is dimensioned RV(NV), where NV must be at least N*(2*MB-1).

## `RV6`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

temporary storage arrays. If the subroutine is being used to solve systems of linear equations, the determinant (up to sign) of A-W(M)*I is available, upon return, as the product of the first N elements of RV. one-dimensional REAL arrays. Note that RV is dimensioned RV(NV), where NV must be at least N*(2*MB-1). is dimensioned RV6(N). Questions and comments should be directed to B.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `W`: not a workspace argument
- `Z`: not a workspace argument
- `RV`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandv`
- Original SLATEC routine: `BANDV`
- Native symbol: `bandv_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BANDV](https://www.netlib.org/slatec/lin/bandv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
