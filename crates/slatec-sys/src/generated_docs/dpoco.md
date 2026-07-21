# Purpose

DPOCO factors a double precision symmetric positive definite matrix and estimates the condition of the matrix. If RCOND is not needed, DPOFA is slightly faster. To solve A*X = B , follow DPOCO by DPOSL. To compute INVERSE(A)*C , follow DPOCO by DPOSL. To compute DETERMINANT(A) , follow DPOCO by DPODI. To compute INVERSE(A) , follow DPOCO by DPODI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOCO](https://www.netlib.org/slatec/lin/dpoco.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. an upper triangular matrix R so that A = TRANS(R)*R where TRANS(R) is the transpose. The strict lower triangle is unaltered. If INFO. NE.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `RCOND`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION an estimate of the reciprocal condition of A. For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z). If INFO. NE. 0 , Z is unchanged.

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

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dpoco`
- Original SLATEC routine: `DPOCO`
- Native symbol: `dpoco_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPOCO](https://www.netlib.org/slatec/lin/dpoco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
