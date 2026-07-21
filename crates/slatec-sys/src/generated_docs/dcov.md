# Purpose

1. Purpose. DCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either DNLS1 or DNLS1E. DCOV and DNLS1 (and DNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, DCOV, DNLS1, and DNLS1E. 2. Subroutine and Type Statements. SUBROUTINE DCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO,

# Description

This canonical unsafe binding exposes original SLATEC routine `DCOV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCOV](https://www.netlib.org/slatec/src/dcov.f).

# Arguments

## `FCN`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed in DNLS1 or DNLS1E, then FCN must do the printing. See the explanation of NPRINT in DNLS1 or DNLS1E. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `IOPT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J).

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of functions.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of variables. N must not exceed M.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an array of length N. On input X must contain the value at which the covariance matrix is to be evaluated. This is usually the value for X returned from a successful run of DNLS1 (or DNLS1E). The value of X will not be changed.

## `FVEC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an output array of length M which contains the functions evaluated at X.

## `R`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDR, *).

is an output array. For IOPT=1 and 2, R is an M by N array. For IOPT=3, R is an N by N array. On output, if INFO=1, the upper N by N submatrix of R contains the covariance matrix evaluated at X.

## `LDR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable which specifies the leading dimension of the array R. For IOPT=1 and 2, must not be less than M. For IOPT=3, LDR must not be less than N.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN. Otherwise, INFO is set as follows. INFO = 0 Improper input parameters (M. LE.

## `WA1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3.

## `WA2`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3.

## `WA3`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION.

## `WA4`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION is a work array of length M.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 Improper input parameters (M.LE.0 or N.LE.0). |
| `INFO` | `1` | 1 Successful return. The covariance matrix has been calculated and stored in the upper N by N submatrix of R. |
| `INFO` | `2` | 2 The Jacobian matrix is singular for the input value of X. The covariance matrix cannot be calculated. The upper N by N submatrix of R contains the QR factorization of the Jacobian (probably not of interest to the user). |

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `R`: not a workspace argument
- `LDR`: not a workspace argument
- `WA1`: INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3.
- `WA2`: INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION work arrays of length N. and WA3.
- `WA3`: not a workspace argument
- `WA4`: INTEGER IOPT,M,N,LDR,INFO DOUBLE PRECISION X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. All TYPE REAL parameters are DOUBLE PRECISION is a work array of length M.

# ABI notes

- Canonical Rust path: `slatec_sys::least_squares::dcov`
- Original SLATEC routine: `DCOV`
- Native symbol: `dcov_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DCOV](https://www.netlib.org/slatec/src/dcov.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
