# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPEV is the BSPLEV routine of the reference. DBSPEV calculates the value of the spline and its derivatives at X from the B-representation (T,A,N,K) and returns them in SVALUE(I),I=1,NDERIV, T(K) .LE. X .LE. T(N+1). AD(I) can be the B-spline coefficients A(I), I=1,N) if NDERIV=1. Otherwise

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPEV](https://www.netlib.org/slatec/src/dbspev.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

knot vector of length N+K.

## `AD`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

must be computed before hand by a call to DBSPDR (T,A,N,K, If X=T(I),I=K,N), right limiting values are obtained. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBSPEV calls DINTRV, DBSPVN vector of length (2*N-NDERIV+1)*NDERIV/2 containing the difference table from DBSPDR.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of B-spline coefficients sum of knot multiplicities-K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the B-spline, K. GE. 1.

## `NDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

If X=T(I),I=K,N), right limiting values are obtained. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBSPEV calls DINTRV, DBSPVN number of derivatives, 1. LE. NDERIV. K.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

argument, T(K). LE. X. T(N+1).

## `INEV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

an initialization parameter which must be set to 1 the first time DBSPEV is called. Output SVALUE,WORK are double precision INEV contains information for efficient process- ing after the initial call and INEV must not be changed by the user. Distinct splines require distinct INEV parameters.

## `SVALUE`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

vector of length NDERIV containing the spline value in SVALUE(1) and the NDERIV-1 derivatives in the remaining components.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

work vector of length 3*K.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `AD`: not a workspace argument
- `SVALUE`: not a workspace argument
- `WORK`: work vector of length 3*K

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbspev`
- Original SLATEC routine: `DBSPEV`
- Native symbol: `dbspev_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSPEV](https://www.netlib.org/slatec/src/dbspev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
