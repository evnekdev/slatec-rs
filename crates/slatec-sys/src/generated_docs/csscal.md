# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CSSCAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSSCAL](https://www.netlib.org/slatec/lin/csscal.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `SA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scale factor.

## `CX`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector with N elements scaled result (unchanged if N. LE. 0) Replace complex CX by (single precision SA) * (complex CX) For I = 0 to N-1, replace CX(IX+I*INCX) with SA * CX(IX+I*INCX), where IX = 1 if INCX. GE. 0, else IX = 1+(1-N)*INCX.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of CX.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `CX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::csscal`
- Original SLATEC routine: `CSSCAL`
- Native symbol: `csscal_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CSSCAL](https://www.netlib.org/slatec/lin/csscal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
