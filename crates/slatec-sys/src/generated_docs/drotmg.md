# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROTMG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROTMG](https://www.netlib.org/slatec/lin/drotmg.f).

# Arguments

## `DD1`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar changed to represent the effect of the transformation.

## `DD2`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar changed to represent the effect of the transformation.

## `DX1`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

double precision scalar DX2 double precision scalar changed to represent the effect of the transformation DX2 unchanged Construct the modified Givens transformation matrix H which zeros the second component of the 2-vector (SQRT(DD1)*DX1,SQRT(DD2)* DY2)**T. With DPARAM(1)=DFLAG, H has one of the following forms: DFLAG=-1. D0 DFLAG=0. D0 DFLAG=1. D0 DFLAG=-2. D0 (DH11 DH12) (1.

## `DY1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Readable second component of the input two-vector used to construct the modified Givens transformation. The routine uses it to form the rotation but leaves this scalar unchanged.

## `DPARAM`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (5).

D. P. 5-vector. DPARAM(1)=DFLAG defined below. Locations 2-5 contain the rotation matrix.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `DPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drotmg`
- Original SLATEC routine: `DROTMG`
- Native symbol: `drotmg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROTMG](https://www.netlib.org/slatec/lin/drotmg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
