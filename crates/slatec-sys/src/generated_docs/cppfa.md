# Purpose

CPPFA factors a complex Hermitian positive definite matrix stored in packed form. CPPFA is usually called by CPPCO, but it can be called directly with a saving in time if RCOND is not needed. (Time for CPPCO) = (1 + 18/N)*(Time for CPPFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPPFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPPFA](https://www.netlib.org/slatec/lin/cppfa.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix A. The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2. See comments below for details. an upper triangular matrix R , stored in packed form, so that A = CTRANS(R)*R. A(I,J) 10 CONTINUE 20 CONTINUE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 for normal return. = K If the leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K If the leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. |
| `INFO` | `1` | 1, N 1, J K = K + 1 |

# Workspace and array requirements

- `AP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cppfa`
- Original SLATEC routine: `CPPFA`
- Native symbol: `cppfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CPPFA](https://www.netlib.org/slatec/lin/cppfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
