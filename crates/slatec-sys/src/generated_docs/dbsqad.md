# Purpose

Abstract **** a double precision routine **** DBSQAD computes the integral on (X1,X2) of a K-th order B-spline using the B-representation (T,BCOEF,N,K). Orders K as high as 20 are permitted by applying a 2, 6, or 10 point Gauss formula on subintervals of (X1,X2) which are formed by included (distinct) knots. If orders K greater than 20 are needed, use DBFQAD with F(X) = 1. The maximum number of significant digits obtainable in DBSQAD is the smaller of 18 and the number of digits carried in double precision arithmetic.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSQAD](https://www.netlib.org/slatec/src/dbsqad.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

knot array of length N+K. LE. X. T(N+1) Output BQUAD,WORK are double precision.

## `BCOEF`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

B-spline coefficient array of length N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

length of coefficient array.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of B-spline, 1. LE. K. 20.

## `X1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

end points of quadrature interval in.

## `X2`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

end points of quadrature interval in.

## `BQUAD`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

integral of the B-spline over (X1,X2).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

work vector of length 3*K.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `BCOEF`: not a workspace argument
- `WORK`: work vector of length 3*K

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dbsqad`
- Original SLATEC routine: `DBSQAD`
- Native symbol: `dbsqad_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSQAD](https://www.netlib.org/slatec/src/dbsqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
