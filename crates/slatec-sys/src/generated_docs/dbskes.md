# Purpose

DBSKES(XNU,X,NIN,BKE) computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, XNU lies in (-1,1), and I = 0, 1, ... , NIN - 1, if NIN is positive and I = 0, -1, ... , NIN + 1, if NIN is negative. On return, the vector BKE(.) contains the results at X for order starting at XNU. XNU, X, and BKE are double precision. NIN is integer.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSKES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSKES](https://www.netlib.org/slatec/fnlib/dbskes.f).

# Arguments

## 1. `XNU`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, 1,1), and I = 0, 1, ... , NIN - 1, if NIN is positive is integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, is integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `NIN`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, is negative.  On return, the is negative.  On return, the vector BKE(.) contains the results at X for order starting at XNU. vector BKE(.) contains the results at X for order starting at XNU. is integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `BKE`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). computes a double precision sequence of exponentially scaled modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0, is integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XNU`: not a workspace argument
- `X`: not a workspace argument
- `NIN`: not a workspace argument
- `BKE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbskes`
- Original SLATEC routine: `DBSKES`
- Native symbol: `dbskes_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSKES](https://www.netlib.org/slatec/fnlib/dbskes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
