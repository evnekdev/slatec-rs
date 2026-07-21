# Purpose

This subroutine is the first step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL GENERAL matrices and reduces one of them to upper Hessenberg form and the other to upper triangular form using orthogonal transformations. It is usually followed by QZIT, QZVAL and, possibly, QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZHES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZHES](https://www.netlib.org/slatec/lin/qzhes.f).

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

## `MATZ`

**Direction:** `input`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

should be set to. TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to. FALSE. otherwise. MATZ is a LOGICAL variable.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the product of the right hand transformations if MATZ has been set to. TRUE. Otherwise, Z is not referenced. is a two-dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzhes`
- Original SLATEC routine: `QZHES`
- Native symbol: `qzhes_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_fortran_logical_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZHES](https://www.netlib.org/slatec/lin/qzhes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
