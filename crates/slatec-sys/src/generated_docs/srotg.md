# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROTG](https://www.netlib.org/slatec/lin/srotg.f).

# Arguments

## 1. `SA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision scalar single precision result R not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SB`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision scalar single precision result Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `SC`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision result If Z=1  set  SC=0.0  and  SS=1.0 Z**2)  and  SS=Z SC**2) Normally, the subprogram SROT(N,SX,INCX,SY,INCY,SC,SS) will next be called to apply the transformation to a 2 by N matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SS`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. single precision result Construct the Givens transformation ( SC  SS ) G = (        ) ,    SC**2 + SS**2 = 1 , SC ) which zeros the second entry of the 2-vector  (SA,SB)**T. The quantity R = (+/-)SQRT(SA**2 + SB**2) overwrites SA in storage.  The value of SB is overwritten by a value Z which If Z=1  set  SC=0.0  and  SS=1.0 SC**2) Normally, the subprogram SROT(N,SX,INCX,SY,INCY,SC,SS) will next be called to apply the transformation to a 2 by N matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `SA`: not a workspace argument
- `SB`: not a workspace argument
- `SC`: not a workspace argument
- `SS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srotg`
- Original SLATEC routine: `SROTG`
- Native symbol: `srotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROTG](https://www.netlib.org/slatec/lin/srotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
