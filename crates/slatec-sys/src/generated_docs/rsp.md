# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a REAL SYMMETRIC PACKED matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `RSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RSP](https://www.netlib.org/slatec/lin/rsp.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `NV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER variable set equal to the dimension of the array A as specified in the calling program. NV must not be less than N*(N+1)/2.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the lower triangle, stored row-wise, of the real symmetric packed matrix. A is a one-dimensional REAL array, dimensioned A(NV).

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the eigenvalues in ascending order. W is a one-dimensional REAL array, dimensioned W(N).

## `MATZ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the eigenvectors if MATZ is not zero. The eigen- vectors are orthonormal. Z is a two-dimensional REAL array, dimensioned Z(NM,N).

## `FV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

## `FV2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, 20*N if NV is less than N*(N+1)/2, J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices 1, 2,. , IERR-1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `W`: not a workspace argument
- `Z`: not a workspace argument
- `FV1`: not a workspace argument
- `FV2`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rsp`
- Original SLATEC routine: `RSP`
- Native symbol: `rsp_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [RSP](https://www.netlib.org/slatec/lin/rsp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
