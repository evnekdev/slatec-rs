# Purpose

SPPFA factors a real symmetric positive definite matrix stored in packed form. SPPFA is usually called by SPPCO, but it can be called directly with a saving in time if RCOND is not needed. (Time for SPPCO) = (1 + 18/N)*(Time for SPPFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPPFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPPFA](https://www.netlib.org/slatec/lin/sppfa.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL (N*(N+1)/2) the packed form of a symmetric matrix A. The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2. See comments below for details. an upper triangular matrix R , stored in packed form, so that A = TRANS(R)*R. A(I,J) 10 CONTINUE 20 CONTINUE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 for normal return. = K if the leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K if the leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. |
| `INFO` | `1` | 1, N 1, J K = K + 1 |

# Workspace and array requirements

- `AP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sppfa`
- Original SLATEC routine: `SPPFA`
- Native symbol: `sppfa_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SPPFA](https://www.netlib.org/slatec/lin/sppfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
