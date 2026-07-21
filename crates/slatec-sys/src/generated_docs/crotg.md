# Purpose

Complex Givens transformation Construct the Givens transformation (C S) G = ( ), C**2 + ABS(S)**2 =1, (-S C) which zeros the second entry of the complex 2-vector (CA,CB)**T The quantity CA/ABS(CA)*NORM(CA,CB) overwrites CA in storage.

# Description

This canonical unsafe binding exposes original SLATEC routine `CROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CROTG](https://www.netlib.org/slatec/lin/crotg.f).

# Arguments

## `CA`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

(Complex) (Complex) CA/ABS(CA)*NORM(CA,CB).

## `CB`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

(Complex).

## `C`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(Real).

## `S`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

(Complex).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::crotg`
- Original SLATEC routine: `CROTG`
- Native symbol: `crotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CROTG](https://www.netlib.org/slatec/lin/crotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
