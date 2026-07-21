# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROTMG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROTMG](https://www.netlib.org/slatec/lin/srotmg.f).

# Arguments

## `SD1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar changed to represent the effect of the transformation.

## `SD2`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar changed to represent the effect of the transformation.

## `SX1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar SY2 single precision scalar changed to represent the effect of the transformation SY2 unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector (SQRT(SD1)*SX1,SQRT(SD2)* SY2)**T. With SPARAM(1)=SFLAG, H has one of the following forms: SFLAG=-1. E0 SFLAG=0. E0 SFLAG=1. E0 SFLAG=-2. E0 (SH11 SH12) (1.

## `SY1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged.

## `SPARAM`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (5).

S. P. 5-vector. SPARAM(1)=SFLAG defined below. Locations 2-5 contain the rotation matrix.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srotmg`
- Original SLATEC routine: `SROTMG`
- Native symbol: `srotmg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROTMG](https://www.netlib.org/slatec/lin/srotmg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
