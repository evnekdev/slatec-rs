# Purpose

CSIFA factors a complex symmetric matrix by elimination with symmetric pivoting. To solve A*X = B , follow CSIFA by CSISL. To compute INVERSE(A)*C , follow CSIFA by CSISL. To compute DETERMINANT(A) , follow CSIFA by CSIDI. To compute INVERSE(A) , follow CSIFA by CSIDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSIFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSIFA](https://www.netlib.org/slatec/lin/csifa.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA,N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. a block diagonal matrix and the multipliers which were used to obtain it. The factorization can be written A = U*D*TRANS(U) where U is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) an integer vector of pivot indices.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CSISL or CSIDI may divide by zero if called.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CSISL or CSIDI may divide by zero if called. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `KPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::csifa`
- Original SLATEC routine: `CSIFA`
- Native symbol: `csifa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CSIFA](https://www.netlib.org/slatec/lin/csifa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
