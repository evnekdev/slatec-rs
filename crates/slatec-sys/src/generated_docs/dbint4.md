# Purpose

Abstract **** a double precision routine **** DBINT4 computes the B representation (T,BCOEF,N,K) of a cubic spline (K=4) which interpolates data (X(I),Y(I)), I=1,NDATA. Parameters IBCL, IBCR, FBCL, FBCR allow the specification of the spline first or second derivative at both X(1) and X(NDATA). When this data is not specified by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) to zero (IBCL=IBCR=2,FBCL=FBCR=0.0). The spline is defined on T(4) .LE. X .LE. T(N+1) with (ordered) interior knots at X(I) values where N=NDATA+2. The knots T(1),T(2),T(3) lie to the left of T(4)=X(1) and the knots T(N+2), T(N+3), T(N+4) lie to the right of T(N+1)=X(NDATA) in increasing order. If no extrapolation outside (X(1),X(NDATA)) is anticipated, the knots T(1)=T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)=

# Description

This canonical unsafe binding exposes original SLATEC routine `DBINT4`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBINT4](https://www.netlib.org/slatec/src/dbint4.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

X vector of abscissae of length NDATA, distinct and in increasing order.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Y vector of ordinates of length NDATA.

## `NDATA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of data points, NDATA. GE. 2.

## `IBCL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

selection parameter for left boundary condition 1 constrain the first derivative at X(1) to FBCL = 2 constrain the second derivative at.

## `IBCR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

selection parameter for right boundary condition 1 constrain first derivative at X(NDATA) to FBCR 2 constrain second derivative at.

## `FBCL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

left boundary values governed by IBCL.

## `FBCR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

right boundary values governed by IBCR.

## `KNTOPT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

knot selection parameter 1 sets knot multiplicity at T(4) and T(N+1) to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets T(I)=W(I) and T(N+1+I)=W(3+I),I=1,3 where W(I),I=1,6 is supplied by the user.

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

X(NDATA) can be specified by KNTOPT=1. KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the first 7 knots symmetric about T(4)=X(1) and similarly for T(N+2), T(N+3), T(N+4) about T(N+1)=X(NDATA). KNTOPT=3 allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+3), T(N+4) to the right of X(NDATA) in the work array W(1) through W(6). In any case, the interpolation on. LE. X.

## `BCOEF`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

B spline coefficient array of length N.

## `N`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of coefficients, N=NDATA+2.

## `K`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of spline, K=4.

## `W`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (5, *).

work array of dimension at least 5*(NDATA+2) If KNTOPT=3, then W(1),W(2),W(3) are knot values to the left of X(1) and W(4),W(5),W(6) are knot values to the right of X(NDATA) in increasing order to be supplied by the user Output T,BCOEF are double precision.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `T`: X(NDATA) can be specified by KNTOPT=1. KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the first 7 knots symmetric about T(4)=X(1) and similarly for T(N+2), T(N+3), T(N+4) about T(N+1)=X(NDATA). KNTOPT=3 allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+3), T(N+4) to the right of X(NDATA) in the work array W(1) through W(6). In any case, the interpolation on. LE. X.
- `BCOEF`: not a workspace argument
- `W`: work array of dimension at least 5*(NDATA+2) If KNTOPT=3, then W(1),W(2),W(3) are knot values to the left of X(1) and W(4),W(5),W(6) are knot values to the right of X(NDATA) in increasing order to be supplied by the user Output T,BCOEF are double precision.

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbint4`
- Original SLATEC routine: `DBINT4`
- Native symbol: `dbint4_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2)`
- Exact Netlib source file: [DBINT4](https://www.netlib.org/slatec/src/dbint4.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
