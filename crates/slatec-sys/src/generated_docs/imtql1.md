# Purpose

This subroutine is a translation of the ALGOL procedure IMTQL1, NUM. MATH. 12, 377-383(1968) by Martin and Wilkinson, as modified in NUM. MATH. 15, 450(1970) by Dubrulle. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 241-248(1971). This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the implicit QL method.

# Description

This canonical unsafe binding exposes original SLATEC routine `IMTQL1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [IMTQL1](https://www.netlib.org/slatec/lin/imtql1.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct and ordered for indices 1, 2,. , IERR-1, but may not be the smallest eigenvalues. E has been destroyed.

## `E`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues should be correct for indices 1, 2,. , IERR-1. These eigenvalues are ordered, but are not necessarily the smallest. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::imtql1`
- Original SLATEC routine: `IMTQL1`
- Native symbol: `imtql1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [IMTQL1](https://www.netlib.org/slatec/lin/imtql1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
