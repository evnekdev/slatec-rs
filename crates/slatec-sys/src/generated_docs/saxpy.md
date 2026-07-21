# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SAXPY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SAXPY](https://www.netlib.org/slatec/lin/saxpy.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `SA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scalar multiplier.

## `SX`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

single precision vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of SX.

## `SY`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

single precision vector with N elements single precision result (unchanged if N. LE. 0) Overwrite single precision SY with single precision SA*SX +SY. For I = 0 to N-1, replace SY(LY+I*INCY) with SA*SX(LX+I*INCX) + where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of SY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SX`: not a workspace argument
- `SY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::saxpy`
- Original SLATEC routine: `SAXPY`
- Native symbol: `saxpy_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SAXPY](https://www.netlib.org/slatec/lin/saxpy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
