# Purpose

Given a collection of points X(I) and a set of values Y(I) which correspond to some function or measurement at each of the X(I), subroutine DPOLFT computes the weighted least-squares polynomial fits of all degrees up to some degree either specified by the user or determined by the routine. The fits thus obtained are in orthogonal polynomial form. Subroutine DP1VLU may then be called to evaluate the fitted polynomials and any of their derivatives at any point. The subroutine DPCOEF may be used to express the polynomial fits as powers of (X-C) for any specified point C. The parameters for DPOLFT are

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOLFT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOLFT](https://www.netlib.org/slatec/src/dpolft.f).

# Arguments

## `N`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the number of data points. The arrays X, Y and W must be dimensioned at least N (N. GE. 1). 0. LE.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

array of values of the independent variable. These values may appear in any order and need not all be distinct.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

array of corresponding function values.

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

array of positive values to be used as weights. If is negative, DPOLFT will set all the weights to 1. 0, which means unweighted least squares error will be minimized. To minimize relative error, the user should set the weights to: W(I) = 1. 0/Y(I)**2, I = 1,. ,N.

## `MAXDEG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

maximum degree to be allowed for polynomial fit. may be any non-negative integer less than N. Note -- MAXDEG cannot be equal to N-1 when a statistical test is to be used for degree selection, i. e. , when input value of EPS is negative. may result in passing the test.

## `NDEG`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

degree of the highest degree fit computed.

## `EPS`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

specifies the criterion to be used in determining the degree of fit to be computed. (1) If EPS is input negative, DPOLFT chooses the degree based on a statistical F test of significance. One of three possible significance levels will be used:. 01,. 05 or. 10.

## `R`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

vector of dimension at least NDEG containing values of the fit of degree NDEG at each of the X(I). Except when the statistical test is used, these values are more accurate than results from subroutine DP1VLU normally are.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

error flag with the following possible values. 1 -- indicates normal execution, i. e. , either (1) the input value of EPS was negative, and the computed polynomial fit of degree NDEG satisfies the specified F test, or (2) the input value of EPS was 0. , and the fits of all degrees up to MAXDEG are complete, or (3) the input value of EPS was positive, and the polynomial of degree NDEG satisfies the RMS error requirement. 2 -- invalid input parameter.

## `A`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

work and output array having at least 3N+3MAXDEG+3 locations Note - DPOLFT calculates all fits of degrees up to and including NDEG. Any or all of these fits can be evaluated or expressed as powers of (X-C) using DP1VLU and DPCOEF after just one call to DPOLFT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `W`: not a workspace argument
- `R`: not a workspace argument
- `A`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dpolft`
- Original SLATEC routine: `DPOLFT`
- Native symbol: `dpolft_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPOLFT](https://www.netlib.org/slatec/src/dpolft.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
