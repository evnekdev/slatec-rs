# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROTG](https://www.netlib.org/slatec/lin/drotg.f).

# Arguments

## 1. `DA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar double precision result R not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DB`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar double precision result Z not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DC`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision result Z**2)  and  DS=Z DC**2) Normally, the subprogram DROT(N,DX,INCX,DY,INCY,DC,DS) will next be called to apply the transformation to a 2 by N matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DS`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision result Construct the Givens transformation ( DC  DS ) G = (        ) ,    DC**2 + DS**2 = 1 , DC ) which zeros the second entry of the 2-vector  (DA,DB)**T . The quantity R = (+/-)SQRT(DA**2 + DB**2) overwrites DA in storage.  The value of DB is overwritten by a value Z which allows DC and DS to be recovered by the following algorithm. If Z=1  set  DC=0.0  and  DS=1.0 DC**2) Normally, the subprogram DROT(N,DX,INCX,DY,INCY,DC,DS) will next be called to apply the transformation to a 2 by N matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `DA`: not a workspace argument
- `DB`: not a workspace argument
- `DC`: not a workspace argument
- `DS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drotg`
- Original SLATEC routine: `DROTG`
- Native symbol: `drotg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROTG](https://www.netlib.org/slatec/lin/drotg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
