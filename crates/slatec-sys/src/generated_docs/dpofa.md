# Purpose

DPOFA factors a double precision symmetric positive definite matrix. DPOFA is usually called by DPOCO, but it can be called directly with a saving in time if RCOND is not needed. (time for DPOCO) = (1 + 18/N)*(time for DPOFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOFA](https://www.netlib.org/slatec/lin/dpofa.f).

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

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dpofa`
- Original SLATEC routine: `DPOFA`
- Native symbol: `dpofa_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DPOFA](https://www.netlib.org/slatec/lin/dpofa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
