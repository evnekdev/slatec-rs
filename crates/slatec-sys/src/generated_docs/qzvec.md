# Purpose

This subroutine is the optional fourth step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form (in which each 2-by-2 block corresponds to a pair of complex eigenvalues) and the other in upper triangular form. It computes the eigenvectors of the triangular problem and transforms the results back to the original coordinate system. It is usually preceded by QZHES, QZIT, and QZVAL.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZVEC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZVEC](https://www.netlib.org/slatec/lin/qzvec.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper quasi-triangular matrix. A is a two- dimensional REAL array, dimensioned A(NM,N). is unaltered. Its subdiagonal elements provide information about the storage of the complex eigenvectors. B has been destroyed.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains a real upper triangular matrix. In addition, location B(N,1) contains the tolerance quantity (EPSB) computed and saved in QZIT. B is a two-dimensional REAL array, dimensioned B(NM,N).

## `ALFR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays with components whose ratios ((ALFR+I*ALFI)/BETA) are the generalized eigenvalues. They are usually obtained from QZVAL. They are dimensioned ALFR(N), ALFI(N), and BETA(N). unaltered.

## `ALFI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays with components whose ratios ((ALFR+I*ALFI)/BETA) are the generalized eigenvalues. They are usually obtained from QZVAL. They are dimensioned ALFR(N), ALFI(N), and BETA(N). unaltered.

## `BETA`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays with components whose ratios ((ALFR+I*ALFI)/BETA) are the generalized eigenvalues. They are usually obtained from QZVAL. They are dimensioned ALFR(N), ALFI(N), and BETA(N). unaltered.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the transformation matrix produced in the reductions by QZHES, QZIT, and QZVAL, if performed. If the eigenvectors of the triangular problem are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,N). contains the real and imaginary parts of the eigenvectors. If ALFI(J). EQ.

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

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzvec`
- Original SLATEC routine: `QZVEC`
- Native symbol: `qzvec_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZVEC](https://www.netlib.org/slatec/lin/qzvec.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
