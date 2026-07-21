# Purpose

CV( ) is a companion function subprogram for FC( ). The documentation for FC( ) has complete usage instructions. CV( ) is used to evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram, FC( ). The variance function defines the square of the probable error of the fitted curve at any point, XVAL. One can use the square root of this variance function to determine a probable error band around the fitted curve. CV( ) is used after a call to FC( ). MODE, an input variable to FC( ), is used to indicate if the variance function is desired. In order to use CV( ), MODE must equal 2 or 4 on input to FC( ). MODE is also used as an output flag from FC( ). Check to make sure that MODE = 0 after calling FC( ), indicating a successful constrained curve fit. The array SDDATA, as input to FC( ), must also be defined with the standard deviation or uncertainty of the Y values to use CV( ). To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by

# Description

This canonical unsafe binding exposes original SLATEC routine `CV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CV](https://www.netlib.org/slatec/src/cv.f).

# Arguments

## 1. `XVAL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. N,1) N,1) spline basis function values at The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for CV( ) are used in FC( ) except the variable XVAL.  Do not change the values of these variables between the call to FC( ) and the use of CV( ). The following is a brief description of the variables The point where the variance is desired. N,1) N,1) spline basis function values at The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for CV( ) are used in FC( ) except the variable XVAL.  Do not change the values of these variables between the call to FC( ) and the use of CV( ). The following is a brief description of the variables The point where the variance is desired. not applicable or not stated by selected source not a workspace argument

## 2. `NDATA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. N,1) The number of discrete (X,Y) pairs for which FC( ) calculated a piece-wise polynomial curve. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `NCONST`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. spline in FC( ). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NORD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. spline used in FC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For wise cubics.) 1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. spline used in FC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For wise cubics.) 1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. not applicable or not stated by selected source not a workspace argument

## 5. `NBKPT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. NORD. The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. NORD+1).  The additional end knots NORD. The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. NORD+1).  The additional end knots not applicable or not stated by selected source not a workspace argument

## 6. `BKPT`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NBKPT). The real array of knots.  Normally the problem data interval will be included between the limits BKPT(NORD) NORD+1).  The additional end knots 1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real work array as used in FC( ).  See FC( ) for the must not must not be modified by the user if the variance function is be modified by the user if the variance function is desired. desired. Real work array as used in FC( ).  See FC( ) for the must not must not be modified by the user if the variance function is be modified by the user if the variance function is desired. desired. not applicable or not stated by selected source

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XVAL`: not a workspace argument
- `NDATA`: not a workspace argument
- `NCONST`: not a workspace argument
- `NORD`: not a workspace argument
- `NBKPT`: not a workspace argument
- `BKPT`: not a workspace argument
- `W`: Real work array as used in FC( ).  See FC( ) for the must not must not be modified by the user if the variance function is be modified by the user if the variance function is desired. desired.

# ABI notes

- Canonical Rust path: `slatec_sys::statistics::cv`
- Original SLATEC routine: `CV`
- Native symbol: `cv_`
- ABI fingerprint: `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CV](https://www.netlib.org/slatec/src/cv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
