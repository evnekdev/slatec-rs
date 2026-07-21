# Purpose

This subroutine is a variant of IMTQL1 which is a translation of ALGOL procedure IMTQL1, NUM. MATH. 12, 377-383(1968) by Martin and Wilkinson, as modified in NUM. MATH. 15, 450(1970) by Dubrulle. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 241-248(1971). This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the implicit QL method and associates with them their corresponding submatrix indices.

# Description

This canonical unsafe binding exposes original SLATEC routine `IMTQLV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [IMTQLV](https://www.netlib.org/slatec/lin/imtqlv.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero.

## `E`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero.

## `E2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E in its last N-1 positions. E2(1) is arbitrary. E2 is a one- dimensional REAL array, dimensioned E2(N).

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct and ordered for indices 1, 2,. , IERR-1, but may not be the smallest eigenvalues. W is a one-dimensional REAL array, dimensioned.

## `IND`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. IND is a one-dimensional REAL array, dimensioned IND(N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues should be correct for indices 1, 2,. , IERR-1. These eigenvalues are ordered, but are not necessarily the smallest.

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a one-dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`IND` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `W`: not a workspace argument
- `IND`: not a workspace argument
- `RV1`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::imtqlv`
- Original SLATEC routine: `IMTQLV`
- Native symbol: `imtqlv_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [IMTQLV](https://www.netlib.org/slatec/lin/imtqlv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
