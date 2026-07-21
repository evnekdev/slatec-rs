# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CAXPY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CAXPY](https://www.netlib.org/slatec/lin/caxpy.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `CA`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

complex scalar multiplier.

## `CX`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector with N elements.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of CX.

## `CY`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector with N elements complex result (unchanged if N. LE. 0) Overwrite complex CY with complex CA*CX + CY. For I = 0 to N-1, replace CY(LY+I*INCY) with CA*CX(LX+I*INCX) + where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of CY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `CX`: not a workspace argument
- `CY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::caxpy`
- Original SLATEC routine: `CAXPY`
- Native symbol: `caxpy_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CAXPY](https://www.netlib.org/slatec/lin/caxpy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
