# Purpose

This subroutine is the second step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART, as modified in technical note NASA TN D-7305(1973) by WARD. This subroutine accepts a pair of REAL matrices, one of them in upper Hessenberg form and the other in upper triangular form. It reduces the Hessenberg matrix to quasi-triangular form using orthogonal transformations while maintaining the triangular form of the other matrix. It is usually preceded by QZHES and followed by QZVAL and, possibly, QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZIT](https://www.netlib.org/slatec/lin/qzit.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper Hessenberg matrix. A is a two- dimensional REAL array, dimensioned A(NM,N).

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper triangular matrix. B is a two- dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements have been altered. The location B(N,1) is used to store EPS1 times the norm of B for later use by QZVAL and QZVEC.

## `EPS1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a tolerance used to determine negligible elements. 0. 0 (or negative) may be input, in which case an element will be neglected only if it is less than roundoff error times the norm of its matrix. If the input EPS1 is positive, then an element will be considered negligible if it is less than EPS1 times the norm of its matrix. A positive value of EPS1 may result in faster execution, but less accurate results. EPS1 is a REAL variable.

## `MATZ`

**Direction:** `input`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

should be set to. TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to. FALSE. otherwise. MATZ is a LOGICAL variable.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

is not referenced. Z is a two-dimensional REAL array, dimensioned Z(NM,N). contains the product of the right hand transformations (for both steps) if MATZ has been set to. TRUE.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if neither A(J,J-1) nor A(J-1,J-2) has become zero after a total of 30*N iterations. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzit`
- Original SLATEC routine: `QZIT`
- Native symbol: `qzit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_fortran_logical_i32,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [QZIT](https://www.netlib.org/slatec/lin/qzit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
