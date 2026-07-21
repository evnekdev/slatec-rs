# Purpose

Description of Parameters The * Flags Output Variables

# Description

This canonical unsafe binding exposes original SLATEC routine `DCOPYM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCOPYM](https://www.netlib.org/slatec/lin/dcopym.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of elements in vector(s).

## `DX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Storage spacing between elements of DX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `DY`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

negative copy of DX.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Storage spacing between elements of DY Note that DY = -DX *** Copy negative of d. p. DX to d. DY. For I=0 to N-1, copy -DX(LX+I*INCX) to DY(LY+I*INCY), where LX=1 if.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `DX`: not a workspace argument
- `DY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dcopym`
- Original SLATEC routine: `DCOPYM`
- Native symbol: `dcopym_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DCOPYM](https://www.netlib.org/slatec/lin/dcopym.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
