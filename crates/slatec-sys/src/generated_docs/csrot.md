# Purpose

CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY. Argument Description

# Description

This canonical unsafe binding exposes original SLATEC routine `CSROT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSROT](https://www.netlib.org/slatec/lin/csrot.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(integer) number of elements in each vector.

## `CX`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

(complex array) beginning of one vector.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(integer) memory spacing of successive elements of vector CX.

## `CY`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

(complex array) beginning of the other vector.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(integer) memory spacing of successive elements of vector CY.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(real) cosine term of the rotation.

## `S`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(real) sine term of the rotation.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `CX`: not a workspace argument
- `CY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::csrot`
- Original SLATEC routine: `CSROT`
- Native symbol: `csrot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CSROT](https://www.netlib.org/slatec/lin/csrot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
