# Purpose

SPPCO factors a real symmetric positive definite matrix stored in packed form and estimates the condition of the matrix. If RCOND is not needed, SPPFA is slightly faster. To solve A*X = B , follow SPPCO by SPPSL. To compute INVERSE(A)*C , follow SPPCO by SPPSL. To compute DETERMINANT(A) , follow SPPCO by SPPDI. To compute INVERSE(A) , follow SPPCO by SPPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPPCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPPCO](https://www.netlib.org/slatec/lin/sppco.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL (N*(N+1)/2) the packed form of a symmetric matrix A. The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2. See comments below for details. an upper triangular matrix R , stored in packed form, so that A = TRANS(R)*R. If INFO. NE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `RCOND`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) a work vector whose contents are usually unimportant. If A is singular to working precision, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z). If INFO. NE. 0 , Z is unchanged.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. |
| `INFO` | `1` | 1, N 1, J K = K + 1 |

# Workspace and array requirements

- `AP`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sppco`
- Original SLATEC routine: `SPPCO`
- Native symbol: `sppco_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SPPCO](https://www.netlib.org/slatec/lin/sppco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
