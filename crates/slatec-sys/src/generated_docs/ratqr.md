# Purpose

This subroutine is a translation of the ALGOL procedure RATQR, NUM. MATH. 11, 264-272(1968) by REINSCH and BAUER. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 257-265(1971). This subroutine finds the algebraically smallest or largest eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QR method with Newton corrections.

# Description

This canonical unsafe binding exposes original SLATEC routine `RATQR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RATQR](https://www.netlib.org/slatec/lin/ratqr.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `EPS1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a theoretical absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, or indeed smaller than its default value, it is reset at each iteration to the respective default value, namely, the product of the relative machine precision and the magnitude of the current eigenvalue iterate. The theoretical absolute error in the K-th eigenvalue is usually not greater than K times EPS1. EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is set to 0. 0e0 if the smallest eigenvalues have been found, and to 2.

## `E`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is set to 0. 0e0 if the smallest eigenvalues have been found, and to 2.

## `E2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E in its last N-1 positions. E2(1) is arbitrary. E2 is a one- dimensional REAL array, dimensioned E2(N).

## `M`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvalues to be found. M is an INTEGER variable. is greater than N, 5*N+K if successive iterates to the K-th eigenvalue are NOT monotone increasing, where K refers to the last such occurrence. Note that subroutine TRIDIB is generally faster and more accurate than RATQR if the eigenvalues are clustered. Questions and comments should be directed to B. S.

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the M algebraically smallest eigenvalues in ascending order, or the M largest eigenvalues in descending order. If an error exit is made because of an incorrect specification of IDEF, no eigenvalues are found. If the Newton iterates for a particular eigenvalue are not monotone, the best estimate obtained is returned and IERR is set. is a one-dimensional REAL array, dimensioned W(N). W need not be distinct from D.

## `IND`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. is an one-dimensional INTEGER array, dimensioned IND(N).

## `BD`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains refined bounds for the theoretical errors of the corresponding eigenvalues in W. These bounds are usually within the tolerance specified by EPS1. BD is a one- dimensional REAL array, dimensioned BD(N). BD need not be distinct from E2.

## `TYPE`

**Direction:** `input`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

Input logical selector. Set true to compute algebraically smallest eigenvalues and false to compute algebraically largest eigenvalues; it also determines the ordering of `W` and the sentinel stored in `E2(1)`.

## `IDEF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

should be set to 1 if the input matrix is known to be positive definite, to -1 if the input matrix is known to be negative definite, and to 0 otherwise. IDEF is an INTEGER variable.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 6*N+1 if IDEF is set to 1 and TYPE to. TRUE. when the matrix is NOT positive definite, or if IDEF is set to -1 and TYPE to. FALSE. when the matrix is NOT negative definite, no eigenvalues are computed, or.

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
- `BD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ratqr`
- Original SLATEC routine: `RATQR`
- Native symbol: `ratqr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [RATQR](https://www.netlib.org/slatec/lin/ratqr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
