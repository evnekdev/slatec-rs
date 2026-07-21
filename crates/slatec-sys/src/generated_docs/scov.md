# Purpose

1. Purpose. SCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either SNLS1 or SNLS1E. SCOV and SNLS1 (and SNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, SCOV, SNLS1, and SNLS1E. 2. Subroutine and Type Statements. SUBROUTINE SCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO REAL X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters.

# Description

This canonical unsafe binding exposes original SLATEC routine `SCOV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCOV](https://www.netlib.org/slatec/src/scov.f).

# Arguments

## 1. `FCN`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. supplied subroutine which calculates the functions.  If the user wants to supply the Jacobian must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) is set as follows. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. supplied subroutine which calculates the functions.  If the user wants to supply the Jacobian must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) is set as follows. not applicable or not stated by selected source not a workspace argument

## 2. `IOPT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions.  See the explanation must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 1. 2. 3. 2.  FVEC contains the function values at X and must not be altered.  FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN 3.  FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN is an input variable which specifies how the Jacobian will 2 or 3, then the user must supply the Jacobian, as well as the function values, through the 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) 1, the code will approximate the Jacobian by forward differencing. 1 and 2, R is an M by N array. 3, R is an N by N array.  On output, if INFO=1, 1 and 2, 3, LDR must not be less than N. 2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions.  See the explanation must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) 1. 2. 3. 2.  FVEC contains the function values at X and must not be altered.  FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN 3.  FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN is an input variable which specifies how the Jacobian will 2 or 3, then the user must supply the Jacobian, as well as the function values, through the 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) 1, the code will approximate the Jacobian by forward differencing. 1 and 2, R is an M by N array. 3, R is an N by N array.  On output, if INFO=1, 1 and 2, 3, LDR must not be less than N. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a positive integer input variable set to the number of functions. contains the functions evaluated at X. 3, LDR must not be less than N. is a positive integer input variable set to the number of functions. contains the functions evaluated at X. 3, LDR must not be less than N. not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 2. 3. is a positive integer input variable set to the number of variables.  N must not exceed M. contain the value at which the covariance matrix is to be evaluated.  This is usually the value for X returned from a successful run of SNLS1 (or SNLS1E).  The value of X will not be changed. contains the covariance contains the covariance matrix evaluated at X. matrix evaluated at X. contains the QR contains the QR factorization of the Jacobian (probably not of factorization of the Jacobian (probably not of interest to the user). interest to the user). 2. 3. is a positive integer input variable set to the number of variables.  N must not exceed M. contain the value at which the covariance matrix is to be evaluated.  This is usually the value for X returned from a successful run of SNLS1 (or SNLS1E).  The value of X will not be changed. contains the covariance contains the covariance matrix evaluated at X. matrix evaluated at X. contains the QR contains the QR factorization of the Jacobian (probably not of factorization of the Jacobian (probably not of interest to the user). interest to the user). not applicable or not stated by selected source not a workspace argument

## 5. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the value contain the value at which the covariance matrix is to be evaluated.  This is at which the covariance matrix is to be evaluated.  This is usually the value for X returned from a successful run of usually the value for X returned from a successful run of SNLS1 (or SNLS1E).  The value of X will not be changed. SNLS1 (or SNLS1E).  The value of X will not be changed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `FVEC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the functions evaluated at X. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `R`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDR, *). 1 and 2, R is an M by N array. contains the covariance matrix evaluated at X. 1 and 2, contains the QR factorization of the Jacobian (probably not of interest to the user). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `LDR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a positive integer input variable which specifies 3, LDR must not be less than N. is a positive integer input variable which specifies 3, LDR must not be less than N. is a positive integer input variable which specifies 3, LDR must not be less than N. not a workspace argument

## 9. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See is set as follows. 0 Improper input parameters (M.LE.0 or N.LE.0). 1 Successful return.  The covariance matrix has been calculated and stored in the upper N by N submatrix of R. 2 The Jacobian matrix is singular for the input value of X.  The covariance matrix cannot be calculated. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `WA1`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a work array of length N. is a work array of length N. not applicable or not stated by selected source

## 11. `WA2`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a work array of length N. is a work array of length N. not applicable or not stated by selected source

## 12. `WA3`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a work array of length N. is a work array of length N. not applicable or not stated by selected source

## 13. `WA4`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a work array of length M. is a work array of length M. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `FCN`: not a workspace argument
- `IOPT`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `R`: not a workspace argument
- `LDR`: not a workspace argument
- `INFO`: not a workspace argument
- `WA1`: is a work array of length N.
- `WA2`: is a work array of length N.
- `WA3`: is a work array of length N.
- `WA4`: is a work array of length M.

# ABI notes

- Canonical Rust path: `slatec_sys::least_squares::scov`
- Original SLATEC routine: `SCOV`
- Native symbol: `scov_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SCOV](https://www.netlib.org/slatec/src/scov.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
