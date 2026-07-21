# Purpose

CHPFA factors a complex Hermitian matrix stored in packed form by elimination with symmetric pivoting. To solve A*X = B , follow CHPFA by CHPSL. To compute INVERSE(A)*C , follow CHPFA by CHPSL. To compute DETERMINANT(A) , follow CHPFA by CHPDI. To compute INERTIA(A) , follow CHPFA by CHPDI. To compute INVERSE(A) , follow CHPFA by CHPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CHPFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHPFA](https://www.netlib.org/slatec/lin/chpfa.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix A. The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2. See comments below for details. A block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written A = U*D*CTRANS(U) where U is a product of permutation and unit upper triangular matrices , CTRANS(U) is the conjugate transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10 CONTINUE 20 CONTINUE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) an integer vector of pivot indices.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CHPSL or CHPDI may divide by zero if called. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CHPSL or CHPDI may divide by zero if called. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. |
| `INFO` | `1` | 1, N 1, J K = K + 1 |

# Workspace and array requirements

- `AP`: not a workspace argument
- `KPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::chpfa`
- Original SLATEC routine: `CHPFA`
- Native symbol: `chpfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CHPFA](https://www.netlib.org/slatec/lin/chpfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
