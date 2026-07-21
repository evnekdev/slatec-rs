# Purpose

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram DFC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978).

# Description

This canonical unsafe binding exposes original SLATEC routine `DEFC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEFC](https://www.netlib.org/slatec/src/defc.f).

# Arguments

## `NDATA`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and.

## `XDATA`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable array of `NDATA` abscissae for the discrete observations. The selected source permits unsorted input; native code does not retain the pointer.

## `YDATA`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable array of `NDATA` ordinates paired with `XDATA`. These values are the observed data used in the weighted B-spline fit and are not retained.

## `SDDATA`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and No sorting of XDATA(*) is required. Any non-negative value of NDATA is allowed. A negative value of NDATA is an error. A zero value for any entry of SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry.

## `NORD`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required.

## `NBKPT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required.

## `BKPT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required.

## `MDEIN`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1 The first time that DEFC( ) has been entered. There are NDATA points to process. =2 This is another entry to DEFC(). The sub- program DEFC( ) has been entered with MDEIN=1 exactly once before for this problem. There are NDATA new additional points to merge and process with any previous points.

## `MDEOUT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An output flag that indicates the status of the curve fit. =-1 A usage error of DEFC( ) occurred. The offending condition is noted with the SLATEC.

## `COEFF`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Writable B-spline coefficient array. When `MDEOUT=1`, it receives the `NBKPT-NORD` fitted coefficients; when the fit is incomplete or invalid, the selected source sets its entries to zero.

## `LW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error. This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy. GE.

## `W`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Persistent work array whose allocated length is supplied by `LW`. Its contents must not be modified by the caller between `MDEIN=1,2,2,...` calls; its leading `(NBKPT-NORD+3)*(NORD+1)` entries can be passed to `DFC` as an old problem when `MDEOUT` is 1 or 2.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `XDATA`: not a workspace argument
- `YDATA`: not a workspace argument
- `SDDATA`: not a workspace argument
- `BKPT`: not a workspace argument
- `COEFF`: not a workspace argument
- `W`: Persistent work array whose allocated length is supplied by `LW`. Its contents must not be modified by the caller between `MDEIN=1,2,2,...` calls; its leading `(NBKPT-NORD+3)*(NORD+1)` entries can be passed to `DFC` as an old problem when `MDEOUT` is 1 or 2.

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::defc`
- Original SLATEC routine: `DEFC`
- Native symbol: `defc_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DEFC](https://www.netlib.org/slatec/src/defc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
