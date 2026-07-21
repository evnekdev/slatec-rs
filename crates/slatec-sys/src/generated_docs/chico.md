# Purpose

CHICO factors a complex Hermitian matrix by elimination with symmetric pivoting and estimates the condition of the matrix. If RCOND is not needed, CHIFA is slightly faster. To solve A*X = B , follow CHICO by CHISL. To compute INVERSE(A)*C , follow CHICO by CHISL. To compute INVERSE(A) , follow CHICO by CHIDI. To compute DETERMINANT(A) , follow CHICO by CHIDI. To compute INERTIA(A), follow CHICO by CHIDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CHICO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHICO](https://www.netlib.org/slatec/lin/chico.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the Hermitian matrix to be factored. Only the diagonal and upper triangle are used. a block diagonal matrix and the multipliers which were used to obtain it. The factorization can be written A = U*D*CTRANS(U) where U is a product of permutation and unit upper triangular matrices , CTRANS(U) is the conjugate transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) an integer vector of pivot indices.

## `RCOND`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `KPVT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chico`
- Original SLATEC routine: `CHICO`
- Native symbol: `chico_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_f32,mut_complex32_array_rank1)`
- Exact Netlib source file: [CHICO](https://www.netlib.org/slatec/lin/chico.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
