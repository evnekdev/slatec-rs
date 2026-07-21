# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SSWAP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSWAP](https://www.netlib.org/slatec/lin/sswap.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `SX`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

single precision vector with N elements input vector SY (unchanged if N. LE. 0).

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of SX.

## `SY`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

single precision vector with N elements input vector SX (unchanged if N. LE. 0) Interchange single precision SX and single precision SY. For I = 0 to N-1, interchange SX(LX+I*INCX) and SY(LY+I*INCY), where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of SY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SX`: not a workspace argument
- `SY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::sswap`
- Original SLATEC routine: `SSWAP`
- Native symbol: `sswap_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSWAP](https://www.netlib.org/slatec/lin/sswap.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
