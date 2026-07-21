# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) for the REAL GENERAL GENERALIZED eigenproblem Ax = (LAMBDA)Bx.

# Description

This canonical unsafe binding exposes original SLATEC routine `RGG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RGG](https://www.netlib.org/slatec/lin/rgg.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real general matrix. A is a two-dimensional REAL array, dimensioned A(NM,N).

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real general matrix. B is a two-dimensional REAL array, dimensioned B(NM,N).

## `ALFR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the numerators of the eigenvalues. one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N).

## `ALFI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the numerators of the eigenvalues. one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N).

## `BETA`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the denominators of the eigenvalues, which are thus given by the ratios (ALFR+I*ALFI)/BETA. Complex conjugate pairs of eigenvalues appear consecutively with the eigenvalue having the positive imaginary part first. is a one-dimensional REAL array, dimensioned BETA(N).

## `MATZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the real and imaginary parts of the eigenvectors if MATZ is not zero. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector. The conjugate of this vector is the eigenvector for the conjugate eigenvalue. is a two-dimensional REAL array, dimensioned Z(NM,N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `ALFR`: not a workspace argument
- `ALFI`: not a workspace argument
- `BETA`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rgg`
- Original SLATEC routine: `RGG`
- Native symbol: `rgg_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [RGG](https://www.netlib.org/slatec/lin/rgg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
