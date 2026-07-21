# Purpose

DLGAMS(X,DLGAM,SGNGAM) calculates the double precision natural logarithm of the absolute value of the Gamma function for double precision argument X and stores the result in double precision argument DLGAM.

# Description

This canonical unsafe binding exposes original SLATEC routine `DLGAMS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLGAMS](https://www.netlib.org/slatec/fnlib/dlgams.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input double-precision argument at which the logarithm of the absolute gamma value is evaluated.

## `DLGAM`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Writable output for the natural logarithm of the absolute value of the gamma function at `X`.

## `SGNGAM`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Writable output sign of the gamma function at `X`; the selected implementation returns either 1 or -1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dlgams`
- Original SLATEC routine: `DLGAMS`
- Native symbol: `dlgams_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64)`
- Exact Netlib source file: [DLGAMS](https://www.netlib.org/slatec/fnlib/dlgams.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
