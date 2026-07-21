# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DSWAP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSWAP](https://www.netlib.org/slatec/lin/dswap.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `DX`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements input vector DY (unchanged if N. LE. 0).

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DX.

## `DY`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements input vector DX (unchanged if N. LE. 0) Interchange double precision DX and double precision DY. For I = 0 to N-1, interchange DX(LX+I*INCX) and DY(LY+I*INCY), where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `DX`: not a workspace argument
- `DY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dswap`
- Original SLATEC routine: `DSWAP`
- Native symbol: `dswap_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSWAP](https://www.netlib.org/slatec/lin/dswap.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
