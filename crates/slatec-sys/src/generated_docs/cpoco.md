# Purpose

CPOCO factors a complex Hermitian positive definite matrix and estimates the condition of the matrix. If RCOND is not needed, CPOFA is slightly faster. To solve A*X = B , follow CPOCO by CPOSL. To compute INVERSE(A)*C , follow CPOCO by CPOSL. To compute DETERMINANT(A) , follow CPOCO by CPODI. To compute INVERSE(A) , follow CPOCO by CPODI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPOCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPOCO](https://www.netlib.org/slatec/lin/cpoco.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the Hermitian matrix to be factored. Only the diagonal and upper triangle are used. an upper triangular matrix R so that A = CTRANS(R)*R where CTRANS(R) is the conjugate transpose. The strict lower triangle is unaltered. If INFO. NE.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `RCOND`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z). If INFO. NE. 0 , Z is unchanged.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpoco`
- Original SLATEC routine: `CPOCO`
- Native symbol: `cpoco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_f32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CPOCO](https://www.netlib.org/slatec/lin/cpoco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
