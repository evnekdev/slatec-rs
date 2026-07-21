# Purpose

MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE DOUBLE PRECISION VERSION

# Description

This canonical unsafe binding exposes original SLATEC routine `DQMOMO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQMOMO](https://www.netlib.org/slatec/src/dqmomo.f).

# Arguments

## `ALFA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameter in the weight function W(X), ALFA. GT. (-1).

## `BETA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Parameter in the weight function W(X), BETA. GT. (-1).

## `RI`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (25).

Vector of dimension 25 is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1,. , 25.

## `RJ`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (25).

Vector of dimension 25 is the integral over (-1,1) of (1-X)**BETA*T(K-1,X), K = 1,. , 25.

## `RG`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (25).

Vector of dimension 25 is the integral over (-1,1) of (1+X)**ALFA*LOG((1+X)/2)*T(K-1,X), K = 1,. , 25.

## `RH`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (25).

Vector of dimension 25 is the integral over (-1,1) of (1-X)**BETA*LOG((1-X)/2)*T(K-1,X), K = 1,. , 25.

## `INTEGR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input parameter indicating the modified Moments to be computed 1 compute RI, RJ = 2 compute RI, RJ, RG = 3 compute RI, RJ, RH = 4 compute RI, RJ, RG, RH.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `RI`: not a workspace argument
- `RJ`: not a workspace argument
- `RG`: not a workspace argument
- `RH`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqmomo`
- Original SLATEC routine: `DQMOMO`
- Native symbol: `dqmomo_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DQMOMO](https://www.netlib.org/slatec/src/dqmomo.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
