# Purpose

This subroutine is a translation of the ALGOL procedure BISECT, NUM. MATH. 9, 386-393(1967) by Barth, Martin, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 249-256(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix between specified boundary indices, using bisection.

# Description

This canonical unsafe binding exposes original SLATEC routine `TRIDIB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TRIDIB](https://www.netlib.org/slatec/lin/tridib.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix. N is an INTEGER variable.

## `EPS1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is an absolute error tolerance for the computed eigen- values. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.

## `E`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.

## `E2`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

contains the squares of the corresponding elements of E. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). is also set to zero.

## `LB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

an interval containing exactly the desired eigenvalues. LB and UB are REAL variables. W contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order.

## `UB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

an interval containing exactly the desired eigenvalues. LB and UB are REAL variables. W contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order.

## `M11`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

specifies the lower boundary index for the set of desired eigenvalues. M11 is an INTEGER variable.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

specifies the number of eigenvalues desired. The upper boundary index M22 is then obtained as M22=M11+M-1. is an INTEGER variable.

## `W`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a one-dimensional REAL array, dimensioned W(M).

## `IND`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. is an one-dimensional INTEGER array, dimensioned IND(M).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 3*N+1 if multiple eigenvalues at index M11 make unique selection of LB impossible, 3*N+2 if multiple eigenvalues at index M22 make unique selection of UB impossible.

## `RV4`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in the bisection process. RV4 and RV5 are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

## `RV5`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in the bisection process. RV4 and RV5 are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

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
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tridib`
- Original SLATEC routine: `TRIDIB`
- Native symbol: `tridib_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TRIDIB](https://www.netlib.org/slatec/lin/tridib.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
