# Purpose

CGEFA factors a complex matrix by Gaussian elimination. CGEFA is usually called by CGECO, but it can be called directly with a saving in time if RCOND is not needed. (Time for CGECO) = (1 + 9/N)*(Time for CGEFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGEFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGEFA](https://www.netlib.org/slatec/lin/cgefa.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the matrix to be factored. an upper triangular matrix and the multipliers which were used to obtain it. The factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `IPVT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) an integer vector of pivot indices.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 normal value. = K if U(K,K). EQ. 0. This is not an error condition for this subroutine, but it does indicate that CGESL or CGEDI will divide by zero if called. Use RCOND in CGECO for a reliable indication of singularity.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if U(K,K) .EQ. 0.0 . This is not an error condition for this subroutine, but it does indicate that CGESL or CGEDI will divide by zero if called. Use RCOND in CGECO for a reliable indication of singularity. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cgefa`
- Original SLATEC routine: `CGEFA`
- Native symbol: `cgefa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGEFA](https://www.netlib.org/slatec/lin/cgefa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
