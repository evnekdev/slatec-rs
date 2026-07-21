# Purpose

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, and zero only when both factors are zero, this subroutine reduces it to a SYMMETRIC TRIDIAGONAL matrix using and accumulating diagonal similarity transformations.

# Description

This canonical unsafe binding exposes original SLATEC routine `FIGI2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FIGI2](https://www.netlib.org/slatec/lin/figi2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, T and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

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

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the diagonal transformation matrix produced in the symmetrization. Z is a two-dimensional REAL array, dimensioned Z(NM,N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative, 2*N+I if T(I,1)*T(I-1,3) is zero with one factor non-zero. In these cases, there does not exist a symmetrizing similarity transformation which is essential for the validity of the later eigenvector computation. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi2`
- Original SLATEC routine: `FIGI2`
- Native symbol: `figi2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [FIGI2](https://www.netlib.org/slatec/lin/figi2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
