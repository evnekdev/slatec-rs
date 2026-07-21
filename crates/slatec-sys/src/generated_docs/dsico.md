# Purpose

DSICO factors a double precision symmetric matrix by elimination with symmetric pivoting and estimates the condition of the matrix. If RCOND is not needed, DSIFA is slightly faster. To solve A*X = B , follow DSICO by DSISL. To compute INVERSE(A)*C , follow DSICO by DSISL. To compute INVERSE(A) , follow DSICO by DSIDI. To compute DETERMINANT(A) , follow DSICO by DSIDI. To compute INERTIA(A), follow DSICO by DSIDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DSICO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSICO](https://www.netlib.org/slatec/lin/dsico.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. a block diagonal matrix and the multipliers which were used to obtain it. The factorization can be written A = U*D*TRANS(U) where U is a product of permutation and unit upper triangular matrices, TRANS(U) is the transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks.

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

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `KPVT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsico`
- Original SLATEC routine: `DSICO`
- Native symbol: `dsico_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DSICO](https://www.netlib.org/slatec/lin/dsico.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
