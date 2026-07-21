# Purpose

1. Purpose. The purpose of DNSQ is to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD and HYBRDJ. 2. Subroutine and Type Statements. SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV,

# Description

This canonical unsafe binding exposes original SLATEC routine `DNSQ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DNSQ](https://www.netlib.org/slatec/src/dnsq.f).

# Arguments

## `FCN`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) CALCULATE THE FUNCTIONS AT X AND RETURN THIS VECTOR IN FVEC. END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of DNSQ. In this case set IFLAG to a negative integer. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `JAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the Jacobian. If IOPT=1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG DOUBLE PRECISION X(N),FVEC(N),FJAC(LDFJAC,N) Calculate the Jacobian at X and return this matrix in FJAC. FVEC contains the function values at X and should not be altered. RETURN END The value of IFLAG should not be changed by JAC unless the user wants to terminate execution of DNSQ. In this case set IFLAG to a negative integer.

## `IOPT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an input variable which specifies how the Jacobian will be calculated. If IOPT=1, then the user must supply the Jacobian through the subroutine JAC. If IOPT=2, then the code will approximate the Jacobian by forward-differencing. 1, then EPSFCN can be ignored (treat it as a dummy.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of functions and variables.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an array of length N. On input X must contain an initial estimate of the solution vector. On output X contains the final estimate of the solution vector.

## `FVEC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an output array of length N which contains the functions evaluated at the output X.

## `FJAC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDFJAC, *).

is an output N by N array which contains the orthogonal matrix Q produced by the QR factorization of the final approximate Jacobian.

## `LDFJAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC.

## `XTOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

is a nonnegative input variable. Termination occurs when the relative error between two consecutive iterates is at most Therefore, XTOL measures the relative error desired in the approximate solution. Section 4 contains more details about XTOL. This parameter is used in a test which makes a comparison between the approximation X and a solution XSOL. DNSQ terminates when the test is satisfied. If the convergence parameter is less than the machine precision (as defined by the function D1MACH(4)), then DNSQ only attempts to satisfy the test defined by the machine precision.

## `MAXFEV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable. Termination occurs when the number of calls to FCN is at least MAXFEV by the end of an iteration. INFO = 3 XTOL is too small. No further improvement in the approximate solution X is possible. INFO = 4 Iteration is not making good progress, as measured by the improvement from the last five Jacobian evaluations. INFO = 5 Iteration is not making good progress, as measured by the improvement from the last ten iterations.

## `ML`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is a nonnegative integer input variable which specifies the number of subdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded or IOPT=1, set ML to at least N - 1.

## `MU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is a nonnegative integer input variable which specifies the number of superdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded or IOPT=1, set MU to at least N - 1.

## `EPSFCN`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an input variable used in determining a suitable step for the forward-difference approximation. This approximation assumes that the relative errors in the functions are of the order of EPSFCN. If EPSFCN is less than the machine precision, it is assumed that the relative errors in the functions are of the order of the machine precision. If.

## `DIAG`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an array of length N. If MODE = 1 (see below), DIAG is internally set. If MODE = 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables.

## `MODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an integer input variable. If MODE = 1, the variables will be scaled internally. If MODE = 2, the scaling is specified by the input DIAG. Other values of MODE are equivalent to MODE = 1.

## `FACTOR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is a positive input variable used in determining the initial step bound. This bound is set to the product of FACTOR and the Euclidean norm of DIAG*X if nonzero, or else to FACTOR itself. In most cases FACTOR should lie in the interval (. 1,100. ). 100.

## `NPRINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. appropriate print statements must be added to FCN(see example). If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows. INFO = 0 Improper input parameters. INFO = 1 Relative error between two consecutive iterates is at most XTOL.

## `NFEV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an integer output variable set to the number of calls to.

## `NJEV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an integer output variable set to the number of calls to.

## `R`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an output array of length LR which contains the upper triangular matrix produced by the QR factorization of the final approximate Jacobian, stored rowwise.

## `LR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is a positive integer input variable not less than (N*(N+1))/2.

## `QTF`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR is an output array of length N which contains the vector (Q transpose)*FVEC.

## `WA1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.

## `WA2`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.

## `WA3`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.

## `WA4`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 Improper input parameters. |
| `INFO` | `1` | 1 Relative error between two consecutive iterates is at most XTOL. |
| `INFO` | `2` | 2 Number of calls to FCN has reached or exceeded |
| `INFO` | `3` | 3 XTOL is too small. No further improvement in the approximate solution X is possible. |
| `INFO` | `4` | 4 Iteration is not making good progress, as measured by the improvement from the last five Jacobian evaluations. |
| `INFO` | `5` | 5 Iteration is not making good progress, as measured by the improvement from the last ten iterations. Sections 4 and 5 contain more details about INFO. components of D*X may have large relative errors, but the fast rate of convergence of DNSQ usually avoids this possibility. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. 5. Unsuccessful Completion. Unsuccessful termination of DNSQ can be due to improper input parameters, arithmetic interrupts, an excessive number of function evaluations, or lack of good progress. Improper Input Parameters. INFO is set to 0 if IOPT .LT .1, |
| `INFO` | `>0` | 2, or N .LE. 0, or LDFJAC .LT. N, or |

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `FJAC`: not a workspace argument
- `LDFJAC`: not a workspace argument
- `DIAG`: not a workspace argument
- `R`: not a workspace argument
- `QTF`: not a workspace argument
- `WA1`: INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.
- `WA2`: INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.
- `WA3`: INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.
- `WA4`: INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR EXTERNAL FCN,JAC work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter.

# ABI notes

- Canonical Rust path: `slatec_sys::nonlinear::dnsq`
- Original SLATEC routine: `DNSQ`
- Native symbol: `dnsq_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DNSQ](https://www.netlib.org/slatec/src/dnsq.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
