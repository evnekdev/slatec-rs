# Purpose

CPOFA factors a complex Hermitian positive definite matrix. CPOFA is usually called by CPOCO, but it can be called directly with a saving in time if RCOND is not needed. (Time for CPOCO) = (1 + 18/N)*(Time for CPOFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPOFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPOFA](https://www.netlib.org/slatec/lin/cpofa.f).

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

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpofa`
- Original SLATEC routine: `CPOFA`
- Native symbol: `cpofa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [CPOFA](https://www.netlib.org/slatec/lin/cpofa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
