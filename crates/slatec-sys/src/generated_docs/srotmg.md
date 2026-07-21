# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROTMG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROTMG](https://www.netlib.org/slatec/lin/srotmg.f).

# Arguments

## 1. `SD1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision scalar changed to represent the effect of the transformation not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SD2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision scalar changed to represent the effect of the transformation not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `SX1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision scalar SY2  single precision scalar changed to represent the effect of the transformation SY2  unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector  (SQRT(SD1)*SX1,SQRT(SD2)* SY2)**T. single precision scalar SY2  single precision scalar changed to represent the effect of the transformation SY2  unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector  (SQRT(SD1)*SX1,SQRT(SD2)* SY2)**T. not applicable or not stated by selected source not a workspace argument

## 4. `SY1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged. Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged. not applicable or not stated by selected source not a workspace argument

## 5. `SPARAM`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (5). vector. SPARAM(1)=SFLAG defined below. Locations 2-5 contain the rotation matrix. SFLAG=-1.E0     SFLAG=0.E0        SFLAG=1.E0     SFLAG=-2.E0 (SH11  SH12)    (1.E0  SH12)    (SH11  1.E0)    (1.E0  0.E0) H=(          )    (          )    (          )    (          ) (SH21  SH22),   (SH21  1.E0),   (-1.E0 SH22),   (0.E0  1.E0). Locations 2-5 of SPARAM contain SH11, SH21, SH12, and SH22, respectively.  (Values of 1.E0, -1.E0, or 0.E0 implied by the value of SPARAM(1) are not stored in SPARAM.) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `SD1`: not a workspace argument
- `SD2`: not a workspace argument
- `SX1`: not a workspace argument
- `SY1`: not a workspace argument
- `SPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srotmg`
- Original SLATEC routine: `SROTMG`
- Native symbol: `srotmg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROTMG](https://www.netlib.org/slatec/lin/srotmg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
