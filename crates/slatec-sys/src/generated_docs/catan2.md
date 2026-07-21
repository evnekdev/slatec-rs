# Purpose

CATAN2(CSN,CCS) calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI). The result is in units of radians and the real part is between -PI and +PI.

# Description

This canonical unsafe binding exposes original SLATEC routine `CATAN2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CATAN2](https://www.netlib.org/slatec/fnlib/catan2.f).

# Arguments

## 1. `CSN`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI).  The result is in units of radians and the real part is between -PI and +PI. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CCS`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI).  The result is in units of radians and the real part is between -PI and +PI. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32,mut_complex32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `CSN`: not a workspace argument
- `CCS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::catan2`
- Original SLATEC routine: `CATAN2`
- Native symbol: `catan2_`
- ABI fingerprint: `function:complex32(mut_complex32,mut_complex32)`
- Exact Netlib source file: [CATAN2](https://www.netlib.org/slatec/fnlib/catan2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
