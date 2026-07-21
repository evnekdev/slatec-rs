# Purpose

CNBFA factors a complex band matrix by elimination. CNBFA is usually called by CNBCO, but it can be called directly with a saving in time if RCOND is not needed. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CNBFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CNBFA](https://www.netlib.org/slatec/src/cnbfa.f).

# Arguments

## `ABE`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, NC) contains the matrix in band storage. The rows of the original matrix are stored in the rows of ABE and the diagonals of the original matrix are stored in columns 1 through ML+MU+1 of ABE. NC must be. GE. 2*ML+MU+1. See the comments below for details.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array ABE. must be. GE. N.

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

INTEGER =0 normal value =K if U(K,K). EQ. 0. This is not an error condition for this subroutine, but it does indicate that CNBSL will divide by zero if called. Use RCOND in CNBCO for a reliable indication of singularity. Band Storage If A is a band matrix, the following program segment will set up the input.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | normal value =K if U(K,K) .EQ. 0.0 . This is not an error condition for this subroutine, but it does indicate that CNBSL will divide by zero if called. Use RCOND in CNBCO for a reliable indication of singularity. Band Storage If A is a band matrix, the following program segment will set up the input. |

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cnbfa`
- Original SLATEC routine: `CNBFA`
- Native symbol: `cnbfa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CNBFA](https://www.netlib.org/slatec/src/cnbfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
