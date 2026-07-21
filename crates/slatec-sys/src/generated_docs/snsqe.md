# Purpose

1. Purpose. The purpose of SNSQE is to find a zero of a system of N non- linear functions in N variables by a modification of the Powell hybrid method. This is done by using the more general nonlinear equation solver SNSQ. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD1 and HYBRJ1. 2. Subroutine and Type Statements. SUBROUTINE SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO,

# Description

This canonical unsafe binding exposes original SLATEC routine `SNSQE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SNSQE](https://www.netlib.org/slatec/src/snsqe.f).

# Arguments

## `FCN`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) Calculate the functions at X and return this vector in FVEC. RETURN END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of SNSQE. In this case, set IFLAG to a negative integer. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `JAC`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

is the name of the user-supplied subroutine which calculates the Jacobian. If IOPT=1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG REAL X(N),FVEC(N),FJAC(LDFJAC,N) Calculate the Jacobian at X and return this matrix in FJAC. FVEC contains the function values at X and should not be altered. RETURN END The value of IFLAG should not be changed by JAC unless the user wants to terminate execution of SNSQE. In this case, set IFLAG to a negative integer. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `IOPT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an input variable which specifies how the Jacobian will be calculated. If IOPT=1, then the user must supply the Jacobian through the subroutine JAC. If IOPT=2, then the code will approximate the Jacobian by forward-differencing. GT. 2, or N. LE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a positive integer input variable set to the number of functions and variables.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector. -1. E0 10 CONTINUE LWA = 180 NPRINT = 0 C Set TOL to the square root of the machine precision. C Unless high precision solutions are required, C this is the recommended setting.

## `FVEC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is an output array of length N which contains the functions evaluated at the output X. TEMP - TEMP1 - TWO*TEMP2 + ONE 10 CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS 0. 1192636E-07 EXIT PARAMETER 1 FINAL APPROXIMATE SOLUTION -0. 5706545E+00 -0. 6816283E+00 -0.

## `TOL`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a non-negative input variable. Termination occurs when the algorithm estimates that the relative error between X and the solution is at most TOL. Section 4 contains more details about TOL. SQRT(R1MACH(4)) CALL SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO,WA,LWA) FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. 7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.

## `NPRINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iteration thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example). If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made.

## `INFO`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows. INFO = 0 improper input parameters. INFO = 1 algorithm estimates that the relative error between X and the solution is at most TOL.

## `WA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (LWA).

INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC is a work array of length LWA.

## `LWA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC is a positive integer input variable not less than (3*N**2+13*N))/2. 4. Successful Completion. The accuracy of SNSQE is controlled by the convergence parame- ter TOL. This parameter is used in a test which makes a compar- ison between the approximation X and a solution XSOL. SNSQE terminates when the test is satisfied.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 improper input parameters. |
| `INFO` | `1` | 1 algorithm estimates that the relative error between X and the solution is at most TOL. |
| `INFO` | `2` | 2 number of calls to FCN has reached or exceeded |
| `INFO` | `1` | or 200*(N+1) for IOPT=2. |
| `INFO` | `3` | 3 TOL is too small. No further improvement in the approximate solution X is possible. |
| `INFO` | `4` | 4 iteration is not making good progress. Sections 4 and 5 contain more details about INFO. nents of X may have large relative errors, but the fast rate of convergence of SNSQE usually avoids this possibility. 5. Unsuccessful Completion. Unsuccessful termination of SNSQE can be due to improper input parameters, arithmetic interrupts, an excessive number of func- tion evaluations, errors in the functions, or lack of good prog- Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1, or |

# Workspace and array requirements

- `X`: not a workspace argument
- `FVEC`: not a workspace argument
- `WA`: INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC is a work array of length LWA.

# ABI notes

- Canonical Rust path: `slatec_sys::nonlinear::snsqe`
- Original SLATEC routine: `SNSQE`
- Native symbol: `snsqe_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SNSQE](https://www.netlib.org/slatec/src/snsqe.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
