# Purpose

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram DFC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see

# Description

This canonical unsafe binding exposes original SLATEC routine `DEFC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEFC](https://www.netlib.org/slatec/src/defc.f).

# Arguments

## 1. `NDATA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Output.. All TYPE REAL variables are DOUBLE PRECISION not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `XDATA`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Readable array of `NDATA` abscissae for the discrete observations. The selected source permits unsorted input; native code does not retain the pointer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `YDATA`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Readable array of `NDATA` ordinates paired with `XDATA`. These values are the observed data used in the weighted B-spline fit and are not retained. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SDDATA`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and No sorting of XDATA(*) is required.  Any non-negative value of NDATA is allowed.  A negative value of NDATA is an will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `NORD`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. NORD+1). 1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data.  No sorting of BKPT(*) is required. Internal to DEFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values.  The contents of must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition 4 when we are using piecewise cubics.) Output.. All TYPE REAL variables are DOUBLE PRECISION The first time DBVALU( ) is called, INBV=1 must be specified.  This value of INBV is the overwritten by DBVALU( ).  The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to DEFC( ). DBVALU( ) expects the breakpoint array BKPT(*) to be sorted. NORD+1). 1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data.  No sorting of BKPT(*) is required. Internal to DEFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values.  The contents of must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition 4 when we are using piecewise cubics.) Output.. All TYPE REAL variables are DOUBLE PRECISION The first time DBVALU( ) is called, INBV=1 must be specified.  This value of INBV is the overwritten by DBVALU( ).  The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to DEFC( ). DBVALU( ) expects the breakpoint array BKPT(*) to be sorted. not applicable or not stated by selected source not a workspace argument

## 6. `NBKPT`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. spline of order NORD are in the array BKPT(*).  Normally the problem data interval will be included between NORD+1). .GE. 2*NORD. Other values are considered errors. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For example, NORD+3)*(NORD+1)+ Output.. All TYPE REAL variables are DOUBLE PRECISION Output.. All TYPE REAL variables are DOUBLE PRECISION NORD parameters are the B-spline coefficients. NORD+3)*(NORD+1) entries of W(*) and providing this data to DFC( ) with the "old problem" designation. The user should read the usage instructions for subprogram DFC( ) for further details. Working Array.. All TYPE REAL variables are DOUBLE PRECISION NORD+3)*(NORD+1) entries of NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `BKPT`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). NORD+1). NORD+1). 1,..., is not changed. NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `MDEIN`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1  The first time that DEFC( ) has been entered.  There are NDATA points to process. =2  This is another entry to DEFC().  The sub- 1 exactly once before for this problem.  There are NDATA new additional points to merge and process with any previous points. ant that the set of knots remain fixed at the same values for all entries to DEFC( ).) 1,2,2,... . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `MDEOUT`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. An output flag that indicates the status of the curve fit. =-1  A usage error of DEFC( ) occurred.  The offending condition is noted with the SLATEC 1, this array contains the unknowns obtained from the least 2, not enough data was processed to uniquely determine the B-spline coefficients. 1, all values of COEFF(*) are set to zero. If the user is not satisfied with the fitted curve returned by DEFC( ), the constrained least squares curve fitting subprogram DFC( ) may be required.  The work done within DEFC( ) to accumulate the data can be utilized by the user, if so desired.  This involves 1 or 2. Evaluating the Fitted Curve.. To evaluate derivative number IDER at XVAL, use the function subprogram DBVALU( ). 1 was obtained from DEFC( ), XVAL is in the data interval, and IDER is nonnegative and .LT. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `COEFF`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `LW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error.  This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy NORD+3)*(NORD+1)+ The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error.  This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy NORD+3)*(NORD+1)+ not applicable or not stated by selected source not a workspace argument

## 12. `W`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION This array is typed DOUBLE PRECISION. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls are acceptable as direct input to DFC( ) Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION This array is typed DOUBLE PRECISION. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls are acceptable as direct input to DFC( ) not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NDATA`: not a workspace argument
- `XDATA`: not a workspace argument
- `YDATA`: not a workspace argument
- `SDDATA`: not a workspace argument
- `NORD`: not a workspace argument
- `NBKPT`: not a workspace argument
- `BKPT`: not a workspace argument
- `MDEIN`: not a workspace argument
- `MDEOUT`: not a workspace argument
- `COEFF`: not a workspace argument
- `LW`: not a workspace argument
- `W`: Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION This array is typed DOUBLE PRECISION. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls are acceptable as direct input to DFC( )

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::defc`
- Original SLATEC routine: `DEFC`
- Native symbol: `defc_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DEFC](https://www.netlib.org/slatec/src/defc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
