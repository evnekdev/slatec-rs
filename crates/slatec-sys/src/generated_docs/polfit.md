# Purpose

Given a collection of points X(I) and a set of values Y(I) which correspond to some function or measurement at each of the X(I), subroutine POLFIT computes the weighted least-squares polynomial fits of all degrees up to some degree either specified by the user or determined by the routine. The fits thus obtained are in orthogonal polynomial form. Subroutine PVALUE may then be called to evaluate the fitted polynomials and any of their derivatives at any point. The subroutine PCOEF may be used to

# Description

This canonical unsafe binding exposes original SLATEC routine `POLFIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POLFIT](https://www.netlib.org/slatec/src/polfit.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the number of data points.  The arrays X, Y and W must be dimensioned at least  N  (N .GE. 1). .GE. 1 1  for  EPS .GE. 0. 2  for  EPS .LT. 0. the number of data points.  The arrays X, Y and W must be dimensioned at least  N  (N .GE. 1). .GE. 1 1  for  EPS .GE. 0. 2  for  EPS .LT. 0. not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). C) for any specified point C. The parameters for  POLFIT  are array of values of the independent variable.  These values may appear in any order and need not all be distinct. C) using  PVALUE  and  PCOEF after just one call to  POLFIT . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). array of corresponding function values. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). array of positive values to be used as weights.  If is negative,  POLFIT  will set all the weights to 1.0, which means unweighted least squares error will be minimized.  To minimize relative error, the user should set the weights to:  W(I) = 1.0/Y(I)**2, I = 1,...,N . 1.0  or  W(I) .GT. 0., I=1,...,N . 3 -- cannot satisfy the RMS error requirement with a polynomial of degree no greater than  MAXDEG .  Best fit found is of degree  MAXDEG . 4 -- cannot satisfy the test for significance using current value of  MAXDEG .  Statistically, the best fit found is of order  NORD .  (In this case, not stated by selected source not applicable or not stated by selected source

## 5. `MAXDEG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. maximum degree to be allowed for polynomial fit. negative integer less than  N. Note -- MAXDEG  cannot be equal to  N-1  when a statistical test is to be used for degree selection, i.e., when input value of  EPS  is negative. 1  for  EPS .GE. 0. 2  for  EPS .LT. 0. 1, or MAXDEG).  Using a higher value of may result in passing the test. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `NDEG`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. degree of the highest degree fit computed. MAXDEG-2, .  Any or all of these fits can be evaluated or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. specifies the criterion to be used in determining the degree of fit to be computed. (1)  If  EPS  is input negative,  POLFIT  chooses the degree based on a statistical F test of significance.  One of three possible significance levels will be used:  .01, .05 or 1.0 , the routine will automatically select one of these levels based on the number of data points and the maximum degree to be considered.  If  EPS  is input as -.01, -.05, or -.10, a significance level of .01, .05, or .10, respectively, will be used. (2)  If  EPS  is set to 0.,  POLFIT  computes the polynomials of degrees 0 through  MAXDEG . (3)  If  EPS  is input positive,  EPS  is the RMS RMS error of the polynomial of degree  NDEG . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `R`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). vector of dimension at least NDEG containing values of the fit of degree  NDEG  at each of the  X(I) . Except when the statistical test is used, these values are more accurate than results from subroutine PVALUE  normally are. vector of dimension at least NDEG containing values of the fit of degree  NDEG  at each of the  X(I) . Except when the statistical test is used, these values are more accurate than results from subroutine PVALUE  normally are. not applicable or not stated by selected source not a workspace argument

## 9. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. error flag with the following possible values. 1 -- indicates normal execution, i.e., either (1)  the input value of  EPS  was negative, and the computed polynomial fit of degree  NDEG satisfies the specified F test, or (2)  the input value of  EPS  was 0., and the fits of all degrees up to  MAXDEG  are complete, or (3)  the input value of  EPS  was positive, and the polynomial of degree  NDEG  satisfies the RMS not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). work and output array having at least 3N+3MAXDEG+3 locations Note - POLFIT  calculates all fits of degrees up to and including not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

fitted polynomial.  POLFIT  will increase the degree of fit until this criterion is met or until the maximum degree is reached. 2 -- invalid input parameter.  At least one of the input parameters has an illegal value and must be corrected before  POLFIT  can proceed.  Valid input results when the following restrictions are observed

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `Y`: not a workspace argument
- `W`: array of positive values to be used as weights.  If is negative,  POLFIT  will set all the weights to 1.0, which means unweighted least squares error will be minimized.  To minimize relative error, the user should set the weights to:  W(I) = 1.0/Y(I)**2, I = 1,...,N . 1.0  or  W(I) .GT. 0., I=1,...,N . 3 -- cannot satisfy the RMS error requirement with a polynomial of degree no greater than  MAXDEG .  Best fit found is of degree  MAXDEG . 4 -- cannot satisfy the test for significance using current value of  MAXDEG .  Statistically, the best fit found is of order  NORD .  (In this case,
- `MAXDEG`: not a workspace argument
- `NDEG`: not a workspace argument
- `EPS`: not a workspace argument
- `R`: not a workspace argument
- `IERR`: not a workspace argument
- `A`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::polfit`
- Original SLATEC routine: `POLFIT`
- Native symbol: `polfit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [POLFIT](https://www.netlib.org/slatec/src/polfit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
