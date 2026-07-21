# Purpose

CGBFA factors a complex band matrix by elimination. CGBFA is usually called by CGBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGBFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGBFA](https://www.netlib.org/slatec/lin/cgbfa.f).

# Arguments

## `ABD`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) contains the matrix in band storage. The columns of the matrix are stored in the columns of ABD and the diagonals of the matrix are stored in rows ML+1 through 2*ML+MU+1 of ABD. See the comments below for details. an upper triangular matrix in band storage and the multipliers which were used to obtain it. The factorization can be written A = L*U where L is a product of permutation and unit lower triangular matrices and U is upper triangular. A(I,J) 10 CONTINUE 20 CONTINUE This uses rows ML+1 through 2*ML+MU+1 of ABD.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array ABD. must be. GE. 2*ML + MU + 1.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the original matrix.

## `ML`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER number of diagonals below the main diagonal. 0. LE. ML. LT. N.

## `MU`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER number of diagonals above the main diagonal. 0. LE. MU. LT. N.

## `IPVT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) an integer vector of pivot indices.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 normal value. = K if U(K,K). EQ. 0. This is not an error condition for this subroutine, but it does indicate that CGBSL will divide by zero if called. Use RCOND in CGBCO for a reliable indication of singularity.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if U(K,K) .EQ. 0.0 . This is not an error condition for this subroutine, but it does indicate that CGBSL will divide by zero if called. Use RCOND in CGBCO for a reliable indication of singularity. Band Storage If A is a band matrix, the following program segment will set up the input. |

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cgbfa`
- Original SLATEC routine: `CGBFA`
- Native symbol: `cgbfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGBFA](https://www.netlib.org/slatec/lin/cgbfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
