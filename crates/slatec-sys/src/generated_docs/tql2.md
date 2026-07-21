# Purpose

This subroutine is a translation of the ALGOL procedure TQL2, NUM. MATH. 11, 293-306(1968) by Bowdler, Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 227-240(1971). This subroutine finds the eigenvalues and eigenvectors of a SYMMETRIC TRIDIAGONAL matrix by the QL method. The eigenvectors of a FULL SYMMETRIC matrix can also be found if TRED2 has been used to reduce this full matrix to tridiagonal form.

# Description

This canonical unsafe binding exposes original SLATEC routine `TQL2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TQL2](https://www.netlib.org/slatec/lin/tql2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct but unordered for indices 1, 2,. , IERR-1. E has been destroyed.

## `E`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned.

## `Z`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the transformation matrix produced in the reduction by TRED2, if performed. If the eigenvectors of the tridiagonal matrix are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,N). contains orthonormal eigenvectors of the symmetric tridiagonal (or full) matrix. If an error exit is made, contains the eigenvectors associated with the stored eigenvalues.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after 30 iterations. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tql2`
- Original SLATEC routine: `TQL2`
- Native symbol: `tql2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [TQL2](https://www.netlib.org/slatec/lin/tql2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
