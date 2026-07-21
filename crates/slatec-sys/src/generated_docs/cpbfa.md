# Purpose

CPBFA factors a complex Hermitian positive definite matrix stored in band form. CPBFA is usually called by CPBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPBFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPBFA](https://www.netlib.org/slatec/lin/cpbfa.f).

# Arguments

## `ABD`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the matrix to be factored. The columns of the upper triangle are stored in the columns of ABD and the diagonals of the upper triangle are stored in the rows of ABD. See the comments below for details. an upper triangular matrix R , stored in band form, so that A = CTRANS(R)*R. A(I,J) 10 CONTINUE 20 CONTINUE.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array ABD. must be. GE. M + 1.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `M`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the number of diagonals above the main diagonal. 0. LE. M. LT. N.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 for normal return. = K if the leading minor of order K is not positive definite. Band Storage If A is a Hermitian positive definite band matrix, the following program segment will set up the input.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K if the leading minor of order K is not positive definite. Band Storage If A is a Hermitian positive definite band matrix, the following program segment will set up the input. |

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cpbfa`
- Original SLATEC routine: `CPBFA`
- Native symbol: `cpbfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [CPBFA](https://www.netlib.org/slatec/lin/cpbfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
