# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `ICOPY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ICOPY](https://www.netlib.org/slatec/lin/icopy.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `IX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

integer vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of IX.

## `IY`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

integer vector with N elements copy of vector IX (unchanged if N. LE. 0) Copy integer IX to integer IY. For I = 0 to N-1, copy IX(LX+I*INCX) to IY(LY+I*INCY), where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of IY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `IX`: not a workspace argument
- `IY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::icopy`
- Original SLATEC routine: `ICOPY`
- Native symbol: `icopy_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [ICOPY](https://www.netlib.org/slatec/lin/icopy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
