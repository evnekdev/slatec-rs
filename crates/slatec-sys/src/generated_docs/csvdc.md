# Purpose

CSVDC is a subroutine to reduce a complex NxP matrix X by unitary transformations U and V to diagonal form. The diagonal elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSVDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSVDC](https://www.netlib.org/slatec/lin/csvdc.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDX, *).

COMPLEX(LDX,P), where LDX. GE. N. contains the matrix whose singular value decomposition is to be computed. X is destroyed by CSVDC.

## `LDX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array X.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of rows of the matrix X.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of columns of the matrix X.

## `S`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(MM), where MM = MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude.

## `E`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). E ordinarily contains zeros. However see the discussion of INFO for exceptions.

## `U`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDU, *).

COMPLEX(LDU,K), where LDU. GE. N. If JOBA. EQ. 1 then K.

## `LDU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array U (see below).

## `V`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDV, *).

COMPLEX(LDV,P), where LDV. GE. P. contains the matrix of right singular vectors. is not referenced if JOB. EQ.

## `LDV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array V (see below).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N). is a scratch array.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A. EQ. 0 Do not compute the left singular A. 1 Return the N left singular vectors in U.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. The singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),. ,S(M) are correct (here M=MIN(N,P)). Thus if INFO. EQ. 0, all the singular values and their vectors are correct.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`INFO` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `S`: not a workspace argument
- `E`: not a workspace argument
- `U`: not a workspace argument
- `LDU`: not a workspace argument
- `V`: not a workspace argument
- `LDV`: not a workspace argument
- `WORK`: COMPLEX(N). is a scratch array.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::csvdc`
- Original SLATEC routine: `CSVDC`
- Native symbol: `csvdc_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CSVDC](https://www.netlib.org/slatec/lin/csvdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
