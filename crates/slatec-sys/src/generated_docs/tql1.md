# Purpose

This subroutine is a translation of the ALGOL procedure TQL1, NUM. MATH. 11, 293-306(1968) by Bowdler, Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 227-240(1971). This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the QL method.

# Description

This canonical unsafe binding exposes original SLATEC routine `TQL1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TQL1](https://www.netlib.org/slatec/lin/tql1.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an not applicable or not stated by selected source not a workspace argument

## 3. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned has been destroyed. contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned has been destroyed. not applicable or not stated by selected source not a workspace argument

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
- `E`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tql1`
- Original SLATEC routine: `TQL1`
- Native symbol: `tql1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [TQL1](https://www.netlib.org/slatec/lin/tql1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
