# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. Right limiting values (right derivatives) are returned except at the right end point X=T(N+1) where left limiting values are computed. The spline is defined on T(K) .LE. X .LE. T(N+1). DBVALU returns a fatal error message when X is outside of this interval. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV

# Description

This canonical unsafe binding exposes original SLATEC routine `DBVALU`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBVALU](https://www.netlib.org/slatec/src/dbvalu.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

knot vector of length N+K.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

B-spline coefficient vector of length N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of B-spline coefficients sum of knot multiplicities-K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the B-spline, K. GE. 1.

## `IDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the derivative, 0. LE. IDERIV. K-1 0 returns the B-spline value.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

argument, T(K). LE. X. T(N+1).

## `INBV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

an initialization parameter which must be set to 1 the first time DBVALU is called. Output WORK,DBVALU are double precision INBV contains information for efficient process- ing after the initial call and INBV must not be changed by the user. Distinct splines require distinct INBV parameters.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

work vector of length 3*K. DBVALU - value of the IDERIV-th derivative at X.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `WORK`: work vector of length 3*K. DBVALU - value of the IDERIV-th derivative at X

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbvalu`
- Original SLATEC routine: `DBVALU`
- Native symbol: `dbvalu_`
- ABI fingerprint: `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBVALU](https://www.netlib.org/slatec/src/dbvalu.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
