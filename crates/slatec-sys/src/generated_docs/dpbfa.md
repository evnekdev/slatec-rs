# Purpose

DPBFA factors a double precision symmetric positive definite matrix stored in band form. DPBFA is usually called by DPBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPBFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPBFA](https://www.netlib.org/slatec/lin/dpbfa.f).

# Arguments

## `ABD`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, N) the matrix to be factored. The columns of the upper triangle are stored in the columns of ABD and the diagonals of the upper triangle are stored in the rows of ABD. See the comments below for details. an upper triangular matrix R , stored in band form, so that A = TRANS(R)*R. A(I,J) 10 CONTINUE 20 CONTINUE.

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

INTEGER = 0 for normal return. = K if the leading minor of order K is not positive definite. Band Storage If A is a symmetric positive definite band matrix, the following program segment will set up the input.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K if the leading minor of order K is not positive definite. Band Storage If A is a symmetric positive definite band matrix, the following program segment will set up the input. |

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dpbfa`
- Original SLATEC routine: `DPBFA`
- Native symbol: `dpbfa_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DPBFA](https://www.netlib.org/slatec/lin/dpbfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
