# Purpose

1. Purpose. The purpose of DNLS1E is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. This is done by using the more general least-squares solver DNLS1. The user must provide a subroutine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER1, LMDIF1, and LMSTR1. 2. Subroutine and Type Statements. SUBROUTINE DNLS1E(FCN,IOPT,M,N,X,FVEC,TOL,NPRINT,

# Description

This canonical unsafe binding exposes original SLATEC routine `DNLS1E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DNLS1E](https://www.netlib.org/slatec/src/dnls1e.f).

# Arguments

## `FCN`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

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

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector.

## `FVEC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an output array of length M which contains the functions evaluated at the output X. should not be altered. If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made.

## `TOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

is a non-negative input variable. Termination occurs when the algorithm estimates either that the relative error in the sum of squares is at most TOL or that the relative error between X and the solution is at most TOL. Section 4 contains more details about TOL.

## `NPRINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example) and.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC.

## `IW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is an INTEGER work array of length N.

## `WA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is a work array of length LWA.

## `LWA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is a positive integer input variable not less than N*(M+5)+M for IOPT=1 and 2 or N*(N+5)+M for IOPT=3. 4. Successful Completion.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 improper input parameters. |
| `INFO` | `1` | 1 algorithm estimates that the relative error in the sum of squares is at most TOL. |
| `INFO` | `2` | 2 algorithm estimates that the relative error between X and the solution is at most TOL. |
| `INFO` | `3` | 3 conditions for INFO = 1 and INFO = 2 both hold. |
| `INFO` | `4` | 4 FVEC is orthogonal to the columns of the Jacobian to machine precision. |
| `INFO` | `5` | 5 number of calls to FCN has reached 100*(N+1) |
| `INFO` | `2` | or 3 or 200*(N+1) for IOPT=1. |
| `INFO` | `6` | 6 TOL is too small. No further reduction in the sum of squares is possible. |
| `INFO` | `7` | 7 TOL is too small. No further improvement in the approximate solution X is possible. Sections 4 and 5 contain more details about INFO. fied). Second Convergence Test. If D is a diagonal matrix (implicitly generated by DNLS1E) whose entries contain scale factors for the variables, then this test attempts to guarantee that ENORM(D*(X-XSOL)) .LE. TOL*ENORM(D*XSOL). |
| `INFO` | `10` | 10**(-K), then the larger components of D*X have K significant decimal digits and fied). There is a danger that the smaller components of D*X may have large relative errors, but the choice of D is such that the accuracy of the components of X is usually related to their sensitivity. Third Convergence Test. This test is satisfied when FVEC is orthogonal to the columns of the Jacobian to machine preci- sion. There is no clear relationship between this test and the accuracy of DNLS1E, and furthermore, the test is equally well satisfied at other critical points, namely maximizers and saddle points. Therefore, termination caused by this test |
| `INFO` | `4` | 4) should be examined carefully. 5. Unsuccessful Completion. Unsuccessful termination of DNLS1E can be due to improper input parameters, arithmetic interrupts, or an excessive number of function evaluations. Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1 |
| `INFO` | `>0` | 3, or N .LE. 0, or M .LT. N, or TOL .LT. 0.E0, |
| `INFO` | `1` | or 2 LWA .LT. N*(M+5)+M, or for IOPT=3 |

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `IW`: INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is an INTEGER work array of length N.
- `WA`: INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. is a work array of length LWA.

# ABI notes

- Canonical Rust path: `slatec_sys::least_squares::dnls1e`
- Original SLATEC routine: `DNLS1E`
- Native symbol: `dnls1e_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DNLS1E](https://www.netlib.org/slatec/src/dnls1e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
