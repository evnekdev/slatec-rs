# Purpose

This subroutine is a translation of the ALGOL procedure TQLRAT. This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QL method.

# Description

This canonical unsafe binding exposes original SLATEC routine `TQLRAT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TQLRAT](https://www.netlib.org/slatec/lin/tqlrat.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct and ordered for indices 1, 2,. , IERR-1, but may not be the smallest eigenvalues. E2 has been destroyed.

## `E2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after 30 iterations. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E2`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tqlrat`
- Original SLATEC routine: `TQLRAT`
- Native symbol: `tqlrat_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [TQLRAT](https://www.netlib.org/slatec/lin/tqlrat.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
