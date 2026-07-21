# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROTMG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROTMG](https://www.netlib.org/slatec/lin/drotmg.f).

# Arguments

## 1. `DD1`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar changed to represent the effect of the transformation not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DD2`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar changed to represent the effect of the transformation not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DX1`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar DX2  double precision scalar changed to represent the effect of the transformation DX2  unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector  (SQRT(DD1)*DX1,SQRT(DD2)* DY2)**T. double precision scalar DX2  double precision scalar changed to represent the effect of the transformation DX2  unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector  (SQRT(DD1)*DX1,SQRT(DD2)* DY2)**T. not applicable or not stated by selected source not a workspace argument

## 4. `DY1`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged. Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged. not applicable or not stated by selected source not a workspace argument

## 5. `DPARAM`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (5). vector. DPARAM(1)=DFLAG defined below. Locations 2-5 contain the rotation matrix. DFLAG=-1.D0     DFLAG=0.D0        DFLAG=1.D0     DFLAG=-2.D0 (DH11  DH12)    (1.D0  DH12)    (DH11  1.D0)    (1.D0  0.D0) H=(          )    (          )    (          )    (          ) (DH21  DH22),   (DH21  1.D0),   (-1.D0 DH22),   (0.D0  1.D0). Locations 2-5 of DPARAM contain DH11, DH21, DH12, and DH22, respectively.  (Values of 1.D0, -1.D0, or 0.D0 implied by the value of DPARAM(1) are not stored in DPARAM.) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `DD1`: not a workspace argument
- `DD2`: not a workspace argument
- `DX1`: not a workspace argument
- `DY1`: not a workspace argument
- `DPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drotmg`
- Original SLATEC routine: `DROTMG`
- Native symbol: `drotmg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROTMG](https://www.netlib.org/slatec/lin/drotmg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
