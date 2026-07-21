# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVD is the BSPLVD routine of the reference. DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. ILEFT is input such that

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPVD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPVD](https://www.netlib.org/slatec/src/dbspvd.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

LE. X. LT. T(ILEFT+1). A call to INTRV(T,N+1,X, ILO,ILEFT,MFLAG) will produce the proper ILEFT. The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

LE. ILEFT. N. The nonzero part of the I-th basis function lies in (T(I),T(I+K)), I=1,N). If X=T(ILEFT+1) then VNIKX contains left limiting values (left derivatives) at T(ILEFT+1). In particular, ILEFT = N produces left limiting values at the right end point order of the B-spline, K.

## `NDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of derivatives = NDERIV-1, 1. LE. NDERIV. K.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

T(N+1). To obtain left limiting values at T(I), I=K+1,N+1, set X= next lower distinct knot, call INTRV to get ILEFT, set X=T(I), and then call DBSPVD. argument of basis functions,.

## `ILEFT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

largest integer such that.

## `LDVNIK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

leading dimension of matrix VNIKX Output VNIKX,WORK are double precision.

## `VNIKX`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDVNIK, *).

matrix of dimension at least (K,NDERIV) contain- ing the nonzero basis functions at X and their derivatives columnwise.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

a work vector of length (K+1)*(K+2)/2.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDVNIK`: not a workspace argument
- `VNIKX`: not a workspace argument
- `WORK`: a work vector of length (K+1)*(K+2)/2

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbspvd`
- Original SLATEC routine: `DBSPVD`
- Native symbol: `dbspvd_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSPVD](https://www.netlib.org/slatec/src/dbspvd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
