# Purpose

This subroutine is a translation of the ALGOL procedure TQLRAT. This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QL method.

# Description

This canonical unsafe binding exposes original SLATEC routine `TQLRAT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TQLRAT](https://www.netlib.org/slatec/lin/tqlrat.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an not applicable or not stated by selected source not a workspace argument

## 3. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the subdiagonal elements of the dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). has been destroyed. contains the squares of the subdiagonal elements of the dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). has been destroyed. not applicable or not stated by selected source not a workspace argument

## 4. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, but may not be the smallest eigenvalues. is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after 30 iterations. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `D`: not a workspace argument
- `E2`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tqlrat`
- Original SLATEC routine: `TQLRAT`
- Native symbol: `tqlrat_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [TQLRAT](https://www.netlib.org/slatec/lin/tqlrat.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
