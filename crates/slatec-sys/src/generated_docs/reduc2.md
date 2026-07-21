# Purpose

This subroutine is a translation of the ALGOL procedure REDUC2, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine reduces the generalized SYMMETRIC eigenproblems ABx=(LAMBDA)x OR BAy=(LAMBDA)y, where B is POSITIVE DEFINITE, to the standard symmetric eigenproblem using the Cholesky factorization of B.

# Description

This canonical unsafe binding exposes original SLATEC routine `REDUC2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [REDUC2](https://www.netlib.org/slatec/lin/reduc2.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A and B, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrices A and B. If the Cholesky factor L of B is already available, N should be prefixed with a minus sign. N is an INTEGER variable.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real symmetric input matrices. Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. two-dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). DL contains, if N is negative, the diagonal elements of L. contains in its full lower triangle the full lower triangle of the symmetric matrix derived from the reduction to the standard form.

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real symmetric input matrices. Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. two-dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). DL contains, if N is negative, the diagonal elements of L. contains in its strict lower triangle the strict lower triangle of its Cholesky factor L.

## `DL`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a one-dimensional REAL array, dimensioned DL(N). contains the diagonal elements of L.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, 7*N+1 if B is not positive definite. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `DL`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::reduc2`
- Original SLATEC routine: `REDUC2`
- Native symbol: `reduc2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [REDUC2](https://www.netlib.org/slatec/lin/reduc2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
