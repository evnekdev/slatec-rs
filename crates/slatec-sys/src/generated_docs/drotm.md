# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROTM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROTM](https://www.netlib.org/slatec/lin/drotm.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `DX`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements rotated vector (unchanged if N. LE. 0).

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DX.

## `DY`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements rotated vector (unchanged if N. LE. 0) Apply the modified Givens transformation, H, to the 2 by N matrix (DX**T) (DY**T) , where **T indicates transpose. The elements of DX are in DX(LX+I*INCX), I = 0 to N-1, where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DY.

## `DPARAM`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (5).

5-element D. P. vector. DPARAM(1) is DFLAG described below. Locations 2-5 of SPARAM contain elements of the transformation matrix H described below.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `DX`: not a workspace argument
- `DY`: not a workspace argument
- `DPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drotm`
- Original SLATEC routine: `DROTM`
- Native symbol: `drotm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROTM](https://www.netlib.org/slatec/lin/drotm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
