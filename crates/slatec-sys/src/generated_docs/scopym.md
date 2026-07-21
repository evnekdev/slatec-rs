# Purpose

Description of Parameters The * Flags Output Variables

# Description

This canonical unsafe binding exposes original SLATEC routine `SCOPYM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCOPYM](https://www.netlib.org/slatec/lin/scopym.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of elements in vector(s).

## `SX`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Storage spacing between elements of SX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `SY`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

negative copy of SX.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Storage spacing between elements of SY Note that SY = -SX *** Copy negative of real SX to real SY. For I=0 to N-1, copy -SX(LX+I*INCX) to SY(LY+I*INCY), where LX=1 if.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SX`: not a workspace argument
- `SY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::scopym`
- Original SLATEC routine: `SCOPYM`
- Native symbol: `scopym_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SCOPYM](https://www.netlib.org/slatec/lin/scopym.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
