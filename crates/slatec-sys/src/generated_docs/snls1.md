# Purpose

1. Purpose. The purpose of SNLS1 is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. The user must provide a subrou- tine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER, LMDIF, and LMSTR. 2. Subroutine and Type Statements. SUBROUTINE SNLS1(FCN,IOPT,M,N,X,FVEC,FJAC,LDFJAC,FTOL,XTOL,

# Description

This canonical unsafe binding exposes original SLATEC routine `SNLS1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SNLS1](https://www.netlib.org/slatec/src/snls1.f).

# Arguments

## `FCN`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed (NPRINT positive), then must do the printing. See the explanation of NPRINT below. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

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

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector.

## `FVEC`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an output array of length M which contains the functions evaluated at the output X. should not be altered. If NPRINT is not positive, no special calls to FCN with IFLAG = 0 are made.

## `FJAC`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDFJAC, *).

is an output array. For IOPT=1 and 2, FJAC is an M by N array. For IOPT=3, FJAC is an N by N array. The upper N by N submatrix of FJAC contains an upper triangular matrix R with diagonal elements of nonincreasing magnitude such that T T T P *(JAC *JAC)*P = R *R, where P is a permutation matrix and JAC is the final calcu- lated Jacobian. Column J of P is column IPVT(J) (see below) of the identity matrix. The lower part of FJAC contains information generated during the computation of R.

## `LDFJAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable which specifies the leading dimension of the array FJAC. For IOPT=1 and 2, must not be less than M. For IOPT=3, LDFJAC must not be less than N. LT. M, or for IOPT=3 LDFJAC. N, or FTOL.

## `FTOL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a non-negative input variable. Termination occurs when both the actual and predicted relative reductions in the sum of squares are at most FTOL. Therefore, FTOL measures the relative error desired in the sum of squares. Section 4 con- tains more details about FTOL.

## `XTOL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a non-negative input variable. Termination occurs when the relative error between two consecutive iterates is at most Therefore, XTOL measures the relative error desired in the approximate solution. Section 4 contains more details about XTOL.

## `GTOL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a non-negative input variable. Termination occurs when the cosine of the angle between FVEC and any column of the Jacobian is at most GTOL in absolute value. Therefore, GTOL measures the orthogonality desired between the function vector and the columns of the Jacobian. Section 4 contains more details about GTOL.

## `MAXFEV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a positive integer input variable. Termination occurs when the number of calls to FCN to evaluate the functions has reached MAXFEV.

## `EPSFCN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an input variable used in determining a suitable step for the forward-difference approximation. This approximation assumes that the relative errors in the functions are of the order of EPSFCN. If EPSFCN is less than the machine preci- sion, it is assumed that the relative errors in the functions are of the order of the machine precision. If IOPT=2 or 3, then EPSFCN can be ignored (treat it as a dummy argument).

## `DIAG`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an array of length N. If MODE = 1 (see below), DIAG is internally set. If MODE = 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables.

## `MODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer input variable. If MODE = 1, the variables will be scaled internally. If MODE = 2, the scaling is speci- fied by the input DIAG. Other values of MODE are equivalent to MODE = 1.

## `FACTOR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a positive input variable used in determining the ini- tial step bound. This bound is set to the product of FACTOR and the Euclidean norm of DIAG*X if nonzero, or else to FACTOR itself. In most cases FACTOR should lie in the interval (. 1,100. ). 100.

## `NPRINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example) and.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) REAL FTOL,XTOL,GTOL,EPSFCN,FACTOR REAL X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows. INFO = 0 improper input parameters. INFO = 1 both actual and predicted relative reductions in the sum of squares are at most FTOL.

## `NFEV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer output variable set to the number of calls to FCN for function evaluation.

## `NJEV`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer output variable set to the number of evaluations of the full Jacobian. If IOPT=2, only one call to.

## `IPVT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

is an integer output array of length N. IPVT defines a permutation matrix P such that JAC*P = Q*R, where JAC is the final calculated Jacobian, Q is orthogonal (not stored), and R is upper triangular with diagonal elements of nonincreasing magnitude. Column J of P is column IPVT(J) of the identity matrix.

## `QTF`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an output array of length N which contains the first N elements of the vector (Q transpose)*FVEC.

## `WA1`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

work arrays of length N.

## `WA2`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

work arrays of length N.

## `WA3`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

work arrays of length N.

## `WA4`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is a work array of length M. 4. Successful Completion. The accuracy of SNLS1 is controlled by the convergence parame- ters FTOL, XTOL, and GTOL. These parameters are used in tests which make three types of comparisons between the approximation X and a solution XSOL. SNLS1 terminates when any of the tests is satisfied.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 improper input parameters. |
| `INFO` | `1` | 1 both actual and predicted relative reductions in the sum of squares are at most FTOL. |
| `INFO` | `2` | 2 relative error between two consecutive iterates is at most XTOL. |
| `INFO` | `3` | 3 conditions for INFO = 1 and INFO = 2 both hold. |
| `INFO` | `4` | 4 the cosine of the angle between FVEC and any column of the Jacobian is at most GTOL in absolute value. |
| `INFO` | `5` | 5 number of calls to FCN for function evaluation has reached MAXFEV. |
| `INFO` | `6` | 6 FTOL is too small. No further reduction in the sum of squares is possible. |
| `INFO` | `7` | 7 XTOL is too small. No further improvement in the approximate solution X is possible. |
| `INFO` | `8` | 8 GTOL is too small. FVEC is orthogonal to the columns of the Jacobian to machine precision. Sections 4 and 5 contain more details about INFO. fied). Unless high precision solutions are required, the recommended value for FTOL is the square root of the machine Second Convergence Test. If D is the diagonal matrix whose entries are defined by the array DIAG, then this test attempts to guarantee that ENORM(D*(X-XSOL)) .LE. XTOL*ENORM(D*XSOL). |
| `INFO` | `10` | 10**(-K), then the larger components of D*X have K significant decimal digits and fied). There is a danger that the smaller components of D*X |
| `INFO` | `1` | 1, then the accuracy of the components of X is usually related to their sensitivity. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. Third Convergence Test. This test is satisfied when the cosine of the angle between FVEC and any column of the Jacobian at X is at most GTOL in absolute value. There is no clear rela- tionship between this test and the accuracy of SNLS1, and furthermore, the test is equally well satisfied at other crit- ical points, namely maximizers and saddle points. Therefore, |
| `INFO` | `4` | 4) should be examined carefully. The recommended value for GTOL is zero. 5. Unsuccessful Completion. Unsuccessful termination of SNLS1 can be due to improper input parameters, arithmetic interrupts, or an excessive number of function evaluations. Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1 |
| `INFO` | `1` | or 2 |

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `FJAC`: not a workspace argument
- `LDFJAC`: not a workspace argument
- `DIAG`: not a workspace argument
- `IPVT`: not a workspace argument
- `QTF`: not a workspace argument
- `WA1`: work arrays of length N.
- `WA2`: work arrays of length N.
- `WA3`: work arrays of length N.
- `WA4`: is a work array of length M. 4. Successful Completion. The accuracy of SNLS1 is controlled by the convergence parame- ters FTOL, XTOL, and GTOL. These parameters are used in tests which make three types of comparisons between the approximation X and a solution XSOL. SNLS1 terminates when any of the tests is satisfied.

# ABI notes

- Canonical Rust path: `slatec_sys::least_squares::snls1`
- Original SLATEC routine: `SNLS1`
- Native symbol: `snls1_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SNLS1](https://www.netlib.org/slatec/src/snls1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
