# Purpose

SSVDC is a subroutine to reduce a real NxP matrix X by orthogonal transformations U and V to diagonal form. The elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SSVDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSVDC](https://www.netlib.org/slatec/lin/ssvdc.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDX, *).

REAL(LDX,P), where LDX. GE. N. contains the matrix whose singular value decomposition is to be computed. X is destroyed by SSVDC.

## `LDX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array X.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the number of rows of the matrix X.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the number of columns of the matrix X.

## `S`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(MM), where MM=MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude.

## `E`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(P). E ordinarily contains zeros. However, see the discussion of INFO for exceptions.

## `U`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDU, *).

REAL(LDU,K), where LDU. GE. N. If JOBA. EQ. 1, then K.

## `LDU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array U. (See below).

## `V`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDV, *).

REAL(LDV,P), where LDV. GE. P. contains the matrix of right singular vectors. is not referenced if JOB. EQ.

## `LDV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array V. (See below).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) is a scratch array.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A. EQ. 0 Do not compute the left singular A. 1 Return the N left singular vectors in U. A.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. the singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),. ,S(M) are correct (here M=MIN(N,P)). Thus if. EQ. 0, all the singular values and their vectors are correct.

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
- `WORK`: REAL(N) is a scratch array.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssvdc`
- Original SLATEC routine: `SSVDC`
- Native symbol: `ssvdc_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SSVDC](https://www.netlib.org/slatec/lin/ssvdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
