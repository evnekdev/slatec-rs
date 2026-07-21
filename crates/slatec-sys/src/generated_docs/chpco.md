# Purpose

CHPCO factors a complex Hermitian matrix stored in packed form by elimination with symmetric pivoting and estimates the condition of the matrix. if RCOND is not needed, CHPFA is slightly faster. To solve A*X = B , follow CHPCO by CHPSL. To compute INVERSE(A)*C , follow CHPCO by CHPSL. To compute INVERSE(A) , follow CHPCO by CHPDI. To compute DETERMINANT(A) , follow CHPCO by CHPDI. To compute INERTIA(A), follow CHPCO by CHPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CHPCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHPCO](https://www.netlib.org/slatec/lin/chpco.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix A. The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2. See comments below for details. a block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written A = U*D*CTRANS(U) where U is a product of permutation and unit upper triangular matrices , CTRANS(U) is the conjugate transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10 CONTINUE 20 CONTINUE.

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

COMPLEX(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z). Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `KPVT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::chpco`
- Original SLATEC routine: `CHPCO`
- Native symbol: `chpco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_f32,mut_complex32_array_rank1)`
- Exact Netlib source file: [CHPCO](https://www.netlib.org/slatec/lin/chpco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
