# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SSCAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSCAL](https://www.netlib.org/slatec/lin/sscal.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of elements in input vector(s).

## `SA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

single precision scale factor.

## `SX`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

single precision vector with N elements single precision result (unchanged if N. LE. 0) Replace single precision SX by single precision SA*SX. For I = 0 to N-1, replace SX(IX+I*INCX) with SA * SX(IX+I*INCX), where IX = 1 if INCX. GE. 0, else IX = 1+(1-N)*INCX.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

storage spacing between elements of SX.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `SX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::sscal`
- Original SLATEC routine: `SSCAL`
- Native symbol: `sscal_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSCAL](https://www.netlib.org/slatec/lin/sscal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
