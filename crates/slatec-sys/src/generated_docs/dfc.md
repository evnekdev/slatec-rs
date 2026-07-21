# Purpose

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. Equality and inequality constraints can be imposed on the fitted curve. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further documentation and discussion of constrained curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978).

# Description

This canonical unsafe binding exposes original SLATEC routine `DFC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DFC](https://www.netlib.org/slatec/src/dfc.f).

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

## `NCONST`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of conditions that constrain the B-spline is NCONST. A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array NEQCON+NINCON. When the subprogram DFC( ) uses DLSEI( ) the length of the working array W(*) must be at least LW=NB+(L+NCONST)*L+ 2*(NEQCON+L)+(NINCON+L)+(NINCON+2)*(L+6) The length of the array IW(*) must be at least IW1=NINCON+2*L in any case.

## `XCONST`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

The number of conditions that constrain the B-spline is NCONST. A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array.

## `YCONST`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

The number of conditions that constrain the B-spline is NCONST. A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array Y, and.

## `NDERIV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

The number of conditions that constrain the B-spline is NCONST. A constraint is specified by an (X,Y) pair in the arrays XCONST(*) and YCONST(*), and by the type of constraint and derivative value encoded in the array No sorting of XCONST(*) is required. The value of NDERIV(*) is determined as follows. Suppose the I-th constraint applies to the J-th derivative of the B-spline. (Any non-negative value of J < NORD is permitted. In particular the value J=0 refers to the B-spline itself.

## `MODE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An input flag that directs the least squares solution method used by DFC( ). The variance function, referred to below, defines the square of the probable error of the fitted curve at any point, XVAL. This feature of DFC( ) allows one to use the square root of this variance function to determine a probable error band around the fitted curve. =1 a new problem. No variance function. =2 a new problem.

## `COEFF`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Writable B-spline coefficient array. When `MDEOUT=1`, it receives the `NBKPT-NORD` fitted coefficients; when the fit is incomplete or invalid, the selected source sets its entries to zero.

## `W`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Persistent work array whose allocated length is supplied by `LW`. Its contents must not be modified by the caller between `MDEIN=1,2,2,...` calls; its leading `(NBKPT-NORD+3)*(NORD+1)` entries can be passed to `DFC` as an old problem when `MDEOUT` is 1 or 2.

## `IW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

The amounts of working storage actually allocated for the working arrays W(*) and These quantities are compared with the actual amounts of storage needed in DFC( ). Insufficient storage allocated for either an error. This feature was included in DFC( ) because misreading the storage formulas for W(*) and IW(*) might very well lead to subtle and hard-to-find programming bugs. The length of W(*) must be at least NB=(NBKPT-NORD+3)*(NORD+1)+ 2*MAX(NDATA,NBKPT)+NBKPT+NORD**2 Whenever possible the code uses banded matrix processors DBNDAC( ) and DBNDSL( ). These are utilized if there are no constraints, no variance function is required, and there is sufficient data to uniquely determine the B-spline coefficients. If the band processors cannot be used to determine the solution, then the constrained least squares code DLSEI is used.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `XDATA`: not a workspace argument
- `YDATA`: not a workspace argument
- `SDDATA`: not a workspace argument
- `BKPT`: not a workspace argument
- `XCONST`: not a workspace argument
- `YCONST`: not a workspace argument
- `NDERIV`: not a workspace argument
- `COEFF`: not a workspace argument
- `W`: Persistent work array whose allocated length is supplied by `LW`. Its contents must not be modified by the caller between `MDEIN=1,2,2,...` calls; its leading `(NBKPT-NORD+3)*(NORD+1)` entries can be passed to `DFC` as an old problem when `MDEOUT` is 1 or 2.
- `IW`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::dfc`
- Original SLATEC routine: `DFC`
- Native symbol: `dfc_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DFC](https://www.netlib.org/slatec/src/dfc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
