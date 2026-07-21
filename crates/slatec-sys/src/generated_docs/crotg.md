# Purpose

Complex Givens transformation Construct the Givens transformation

# Description

This canonical unsafe binding exposes original SLATEC routine `CROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CROTG](https://www.netlib.org/slatec/lin/crotg.f).

# Arguments

## 1. `CA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. (Complex) (Complex)      CA/ABS(CA)*NORM(CA,CB) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CB`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. (Complex) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `C`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. S) G  =  (      ),  C**2 + ABS(S)**2 =1, (Real) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `S`

output `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. C) which zeros the second entry of the complex 2-vector (CA,CB)**T The quantity CA/ABS(CA)*NORM(CA,CB) overwrites CA in storage. (Complex) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `CA`: not a workspace argument
- `CB`: not a workspace argument
- `C`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::crotg`
- Original SLATEC routine: `CROTG`
- Native symbol: `crotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CROTG](https://www.netlib.org/slatec/lin/crotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
