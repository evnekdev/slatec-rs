# Purpose

DCV( ) is a companion function subprogram for DFC( ). The documentation for DFC( ) has complete usage instructions. DCV( ) is used to evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram, DFC( ). The variance function defines the square of the probable error of the fitted curve at any point, XVAL. One can use the square root of this variance function to determine a probable error band around the fitted curve. DCV( ) is used after a call to DFC( ). MODE, an input variable to DFC( ), is used to indicate if the variance function is desired. In order to use DCV( ), MODE must equal 2 or 4 on input to DFC( ). MODE is also used as an output flag from DFC( ). Check to make sure that MODE = 0 after calling DFC( ), indicating a successful constrained curve fit. The array SDDATA, as input to DFC( ), must also be defined with the standard deviation or uncertainty of the Y values to use DCV( ). To evaluate the variance function after calling DFC( ) as stated above, use DCV( ) as shown here VAR=DCV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by

# Description

This canonical unsafe binding exposes original SLATEC routine `DCV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCV](https://www.netlib.org/slatec/src/dcv.f).

# Arguments

## 1. `XVAL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. N,1)) N,1)) spline basis function values at The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for DCV( ) are used in DFC( ) except the variable XVAL.  Do not change the values of these variables between the call to DFC( ) and the use of DCV( ). The following is a brief description of the variables The point where the variance is desired, a double precision variable. N,1)) N,1)) spline basis function values at The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints.  This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for DCV( ) are used in DFC( ) except the variable XVAL.  Do not change the values of these variables between the call to DFC( ) and the use of DCV( ). The following is a brief description of the variables The point where the variance is desired, a double precision variable. not applicable or not stated by selected source not a workspace argument

## 2. `NDATA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. N,1)) The number of discrete (X,Y) pairs for which DFC( ) calculated a piece-wise polynomial curve. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `NCONST`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. spline in DFC( ). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NORD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. spline used in DFC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For wise cubics.) NORD+1).  The additional end 1 and I=NBKPT-NORD+2,...,NBKPT, are required by DFC( ) to compute the functions used to fit the data. spline used in DFC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For wise cubics.) NORD+1).  The additional end 1 and I=NBKPT-NORD+2,...,NBKPT, are required by DFC( ) to compute the functions used to fit the data. not applicable or not stated by selected source not a workspace argument

## 5. `NBKPT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. NORD. The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. NORD+1).  The additional end NORD. The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. NORD+1).  The additional end not applicable or not stated by selected source not a workspace argument

## 6. `BKPT`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). The double precision array of knots.  Normally the problem data interval will be included between the limits NORD+1).  The additional end NORD+1).  The additional end 1 and I=NBKPT-NORD+2,...,NBKPT, are required by DFC( ) to compute the functions used to fit the data. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `W`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision work array as used in DFC( ).  See DFC( ) for the required length of W(*).  The contents of W(*) must not be modified by the user if the variance function is desired. Double precision work array as used in DFC( ).  See DFC( ) for the required length of W(*).  The contents of W(*) must not be modified by the user if the variance function is desired. not applicable or not stated by selected source

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`. It has no separate Rust `Result` status channel.

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
- `W`: Double precision work array as used in DFC( ).  See DFC( ) for the required length of W(*).  The contents of W(*) must not be modified by the user if the variance function is desired.

# ABI notes

- Canonical Rust path: `slatec_sys::statistics::dcv`
- Original SLATEC routine: `DCV`
- Native symbol: `dcv_`
- ABI fingerprint: `function:f64(mut_f64,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DCV](https://www.netlib.org/slatec/src/dcv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
