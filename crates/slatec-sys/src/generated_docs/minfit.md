# Purpose

This subroutine is a translation of the ALGOL procedure MINFIT, NUM. MATH. 14, 403-420(1970) by Golub and Reinsch. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 134-151(1971). This subroutine determines, towards the solution of the linear system AX=B, the singular value decomposition A=USV of a real M by N rectangular matrix, forming U B rather than U. Householder bidiagonalization and a variant of the QR algorithm are used.

# Description

This canonical unsafe binding exposes original SLATEC routine `MINFIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [MINFIT](https://www.netlib.org/slatec/lin/minfit.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A and B, as declared in the calling program dimension statement. Note that NM must be at least as large as the maximum of M and N. NM is an INTEGER variable.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of rows of A and B. M is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of columns of A and the order of V. N is an INTEGER variable.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the rectangular coefficient matrix of the system. is a two-dimensional REAL array, dimensioned A(NM,N).

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the N (non-negative) singular values of A (the diagonal elements of S). They are unordered. If an error exit is made, the singular values should be correct for indices IERR+1, IERR+2,. , N. W is a one-dimensional REAL array, dimensioned W(N). B has been overwritten by U B.

## `IP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of columns of B. IP can be zero.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, IP).

contains the constant column matrix of the system if IP is not zero. Otherwise, B is not referenced. B is a two- dimensional REAL array, dimensioned B(NM,IP).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, K if the K-th singular value has not been determined after 30 iterations. The singular values should be correct for indices IERR+1, IERR+2,. , N.

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a one-dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `W`: not a workspace argument
- `B`: not a workspace argument
- `RV1`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::minfit`
- Original SLATEC routine: `MINFIT`
- Native symbol: `minfit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [MINFIT](https://www.netlib.org/slatec/lin/minfit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
