# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DSCAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSCAL](https://www.netlib.org/slatec/lin/dscal.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `DA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scale factor.

## `DX`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

double precision vector with N elements double precision result (unchanged if N. LE. 0) Replace double precision DX by double precision DA*DX. For I = 0 to N-1, replace DX(IX+I*INCX) with DA * DX(IX+I*INCX), where IX = 1 if INCX. GE. 0, else IX = 1+(1-N)*INCX.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of DX.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `DX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dscal`
- Original SLATEC routine: `DSCAL`
- Native symbol: `dscal_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSCAL](https://www.netlib.org/slatec/lin/dscal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
