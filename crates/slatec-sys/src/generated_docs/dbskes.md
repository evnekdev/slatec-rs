# Purpose

DBSKES(XNU,X,NIN,BKE) computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, XNU lies in (-1,1), and I = 0, 1, ... , NIN - 1, if NIN is positive and I = 0, -1, ... , NIN + 1, if NIN is negative. On return, the vector BKE(.) contains the results at X for order starting at XNU.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSKES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSKES](https://www.netlib.org/slatec/fnlib/dbskes.f).

# Arguments

## `XNU`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input fractional starting order. `XNU` must lie strictly between -1 and 1.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input evaluation point. `X` must be strictly positive; the returned sequence is evaluated at this point and native code does not modify it.

## `NIN`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input nonzero signed sequence length. `abs(NIN)` is the required result-array length; a positive value advances the order and a negative value steps it downward from `XNU`.

## `BKE`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Writable output array with at least `abs(NIN)` elements. On return `BKE\[I\]` holds `exp(X) * K_(XNU+I)(X)` for the requested order sequence; native code retains no pointer.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `BKE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbskes`
- Original SLATEC routine: `DBSKES`
- Native symbol: `dbskes_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSKES](https://www.netlib.org/slatec/fnlib/dbskes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
