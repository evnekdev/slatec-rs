# Purpose

This subroutine is the third step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form and the other in upper triangular form. It reduces the quasi-triangular matrix further, so that any remaining 2-by-2 blocks correspond to pairs of complex eigenvalues, and returns quantities whose ratios give the generalized eigenvalues. It is usually preceded by QZHES and QZIT and may be followed by QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZVAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZVAL](https://www.netlib.org/slatec/lin/qzval.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper quasi-triangular matrix. A is a two- dimensional REAL array, dimensioned A(NM,N).

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper triangular matrix. In addition, location B(N,1) contains the tolerance quantity (EPSB) computed and saved in QZIT. B is a two-dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements have been altered. B(N,1) is unaltered.

## `ALFR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations. Non-zero values of ALFI occur in pairs, the first member positive and the second negative. one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N).

## `ALFI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations. Non-zero values of ALFI occur in pairs, the first member positive and the second negative. one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N).

## `BETA`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the corresponding B, normalized to be real and non-negative. The generalized eigenvalues are then the ratios ((ALFR+I*ALFI)/BETA). is a one-dimensional REAL array, dimensioned BETA(N).

## `MATZ`

**Direction:** `input`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

should be set to. TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to. FALSE. otherwise. MATZ is a LOGICAL variable.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the product of the right hand transformations (for all three steps) if MATZ has been set to. TRUE. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

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

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzval`
- Original SLATEC routine: `QZVAL`
- Native symbol: `qzval_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZVAL](https://www.netlib.org/slatec/lin/qzval.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
