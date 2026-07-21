# Purpose

Compute the dot product of 2 complex vectors, CX and CY, e.g. CX DOT CY, or, CXconjugate DOT CY. The real and imaginary parts of CX and CY are converted to double precision, the dot product accumulation is done in double precision and the output is given as 2 double precision numbers, corresponding to the real and imaginary part of the result.

# Description

This canonical unsafe binding exposes original SLATEC routine `DCDOT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCDOT](https://www.netlib.org/slatec/lin/dcdot.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of complex components of CX and CY.

## `FM`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

=+1. 0 compute CX DOT CY. =-1. 0 compute CXconjugate DOT CY.

## `CX`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Complex arrays of length N.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Integer) Spacing of elements of CX to use.

## `CY`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Complex arrays of length N.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Integer) Spacing of elements of CY to use.

## `DCR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

(Double Precision) Real part of dot product.

## `DCI`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

(Double Precision) Imaginary part of dot product.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `CX`: not a workspace argument
- `CY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dcdot`
- Original SLATEC routine: `DCDOT`
- Native symbol: `dcdot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DCDOT](https://www.netlib.org/slatec/lin/dcdot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
