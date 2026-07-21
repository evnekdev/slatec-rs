# Purpose

DLGAMS(X,DLGAM,SGNGAM) calculates the double precision natural logarithm of the absolute value of the Gamma function for double precision argument X and stores the result in double precision argument DLGAM.

# Description

This canonical unsafe binding exposes original SLATEC routine `DLGAMS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLGAMS](https://www.netlib.org/slatec/fnlib/dlgams.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision natural logarithm of the absolute value of the Gamma function for double precision argument X and stores the result in double precision argument DLGAM. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DLGAM`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision natural logarithm of the absolute value of the Gamma function for double precision argument X and stores the result in double precision argument DLGAM. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `SGNGAM`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision natural logarithm of the absolute value of the Gamma function for double precision argument X and stores the result in double precision argument DLGAM. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `DLGAM`: not a workspace argument
- `SGNGAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dlgams`
- Original SLATEC routine: `DLGAMS`
- Native symbol: `dlgams_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64)`
- Exact Netlib source file: [DLGAMS](https://www.netlib.org/slatec/fnlib/dlgams.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
