# Purpose

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, this subroutine reduces it to a symmetric tridiagonal matrix with the same eigenvalues. If, further, a zero product only occurs when both factors are zero, the reduced matrix is similar to the original matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `FIGI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FIGI](https://www.netlib.org/slatec/lin/figi.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix T. N is an INTEGER variable. must be less than or equal to NM.

## `T`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, 3).

contains the nonsymmetric matrix. Its subdiagonal is stored in the last N-1 positions of the first column, its diagonal in the N positions of the second column, and its superdiagonal in the first N-1 positions of the third column. T(1,1) and T(N,3) are arbitrary. is a two-dimensional REAL array, dimensioned T(NM,3). is unaltered.

## `D`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the tridiagonal symmetric matrix. D is a one-dimensional REAL array, dimensioned D(N).

## `E`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the tridiagonal symmetric matrix in its last N-1 positions. E(1) is not set. is a one-dimensional REAL array, dimensioned E(N).

## `E2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E. E2 may coincide with E if the squares are not needed. is a one-dimensional REAL array, dimensioned E2(N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative and a symmetric matrix cannot be produced with FIGI, -(3*N+I) if T(I,1)*T(I-1,3) is zero with one factor non-zero. In this case, the eigenvectors of the symmetric matrix are not simply related to those of T and should not be sought. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi`
- Original SLATEC routine: `FIGI`
- Native symbol: `figi_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [FIGI](https://www.netlib.org/slatec/lin/figi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
