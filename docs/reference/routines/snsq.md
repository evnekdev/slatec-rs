# SNSQ

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find a zero of a system of a N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of SNSQ is to find a zero of a system of N non- linear functions in N variables by a modification of the Powell hybrid method. The user must provide a subroutine which calcu- lates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD and HYBRDJ. 2. Subroutine and Type Statements. SUBROUTINE SNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV,

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::nonlinear::solve_system_expert_f32, slatec::nonlinear::solve_system_with_jacobian_f32`

## Providers

- Canonical provider: `main-src/src/snsq.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/snsq.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/snsq.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/snsq.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SNSQ](https://www.netlib.org/slatec/src/snsq.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) Calculate the functions at X and return this vector in FVEC. RETURN END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of SNSQ. In this case, set IFLAG to a negative integer. |
| 2 | `JAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the name of the user-supplied subroutine which calculates the Jacobian. If IOPT=1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG REAL X(N),FVEC(N),FJAC(LDFJAC,N) Calculate the Jacobian at X and return this matrix in FJAC. FVEC contains the function values at X and should not be altered. RETURN END The value of IFLAG should not be changed by JAC unless the user wants to terminate execution of SNSQ. In this case, set IFLAG to a negative integer. |
| 3 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an input variable which specifies how the Jacobian will be calculated. If IOPT=1, then the user must supply the Jacobian through the subroutine JAC. If IOPT=2, then the code will approximate the Jacobian by forward-differencing. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions and variables. |
| 5 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector. -1. E0 10 CONTINUE LDFJAC = 9 LR = 45 C Set XTOL to the square root of the machine precision. C Unless high precision solutions are required, C this is the recommended setting. |
| 6 | `FVEC` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an output array of length N which contains the functions evaluated at the output X. TEMP - TEMP1 - TWO*TEMP2 + ONE 10 CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS 0. 1192636E-07 NUMBER OF FUNCTION EVALUATIONS 14 EXIT PARAMETER 1 FINAL APPROXIMATE SOLUTION -0. 5706545E+00 -0. 6816283E+00 -0. |
| 7 | `FJAC` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDFJAC, *) | is an output N by N array which contains the orthogonal matrix Q produced by the QR factorization of the final approx- imate Jacobian. |
| 8 | `LDFJAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC. |
| 9 | `XTOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | is a non-negative input variable. Termination occurs when the relative error between two consecutive iterates is at most Therefore, XTOL measures the relative error desired in the approximate solution. Section 4 contains more details about XTOL. This parameter is used in a test which makes a comparison between the approximation X and a solution XSOL. SNSQ termi- nates when the test is satisfied. If the convergence parameter is less than the machine precision (as defined by the function R1MACH(4)), then SNSQ only attempts to satisfy the test defined by the machine precision. |
| 10 | `MAXFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable. Termination occurs when the number of calls to FCN is at least MAXFEV by the end of an iteration. INFO = 3 XTOL is too small. No further improvement in the approximate solution X is possible. INFO = 4 iteration is not making good progress, as measured by the improvement from the last five Jacobian eval- uations. INFO = 5 iteration is not making good progress, as measured by the improvement from the last ten iterations. |
| 11 | `ML` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is a non-negative integer input variable which specifies the number of subdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded or IOPT=1, set ML to at least N - 1. |
| 12 | `MU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is a non-negative integer input variable which specifies the number of superdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded or IOPT=1, set MU to at least N - 1. |
| 13 | `EPSFCN` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an input variable used in determining a suitable step for the forward-difference approximation. This approximation assumes that the relative errors in the functions are of the order of EPSFCN. If EPSFCN is less than the machine preci- sion, it is assumed that the relative errors in the functions are of the order of the machine precision. If IOPT=1, then EPSFCN can be ignored (treat it as a dummy argument). 0. E0 MODE = 2 DO 20 J = 1, 9. |
| 14 | `DIAG` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an array of length N. If MODE = 1 (see below), DIAG is internally set. If MODE = 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables. 1. E0 20 CONTINUE. |
| 15 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an integer input variable. If MODE = 1, the variables will be scaled internally. If MODE = 2, the scaling is speci- fied by the input DIAG. Other values of MODE are equivalent to MODE = 1. |
| 16 | `FACTOR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is a positive input variable used in determining the ini- tial step bound. This bound is set to the product of FACTOR and the Euclidean norm of DIAG*X if nonzero, or else to FACTOR itself. In most cases FACTOR should lie in the interval (. 1,100. ). 100. |
| 17 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iteration thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN(see example). If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made. |
| 18 | `INFO` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows. INFO = 0 improper input parameters. INFO = 1 relative error between two consecutive iterates is at most XTOL. |
| 19 | `NFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an integer output variable set to the number of calls to. |
| 20 | `NJEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an integer output variable set to the number of calls to. |
| 21 | `R` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (LR) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an output array of length LR which contains the upper triangular matrix produced by the QR factorization of the final approximate Jacobian, stored rowwise. FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. 7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0. E0,1. E0,2. |
| 22 | `LR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is a positive integer input variable not less than (N*(N+1))/2. FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. 7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0. E0,1. E0,2. |
| 23 | `QTF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), is an output array of length N which contains the vector (Q TRANSPOSE)*FVEC. FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. 7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0. E0,1. E0,2. |
| 24 | `WA1` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), EXTERNAL FCN,JAC work arrays of length N. 4. Successful Completion. The accuracy of SNSQ is controlled by the convergence parameter REAL ENORM,R1MACH DATA NWRITE /6/ IOPT = 2 N = 9 C The following starting values provide a rough solution. DO 10 J = 1, 9 FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. |
| 25 | `WA2` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), EXTERNAL FCN,JAC work arrays of length N. 4. Successful Completion. The accuracy of SNSQ is controlled by the convergence parameter REAL ENORM,R1MACH DATA NWRITE /6/ IOPT = 2 N = 9 C The following starting values provide a rough solution. DO 10 J = 1, 9 FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. |
| 26 | `WA3` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), EXTERNAL FCN,JAC work arrays of length N. 4. Successful Completion. The accuracy of SNSQ is controlled by the convergence parameter REAL ENORM,R1MACH DATA NWRITE /6/ IOPT = 2 N = 9 C The following starting values provide a rough solution. DO 10 J = 1, 9 FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. |
| 27 | `WA4` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), EXTERNAL FCN,JAC work arrays of length N. 4. Successful Completion. The accuracy of SNSQ is controlled by the convergence parameter REAL ENORM,R1MACH DATA NWRITE /6/ IOPT = 2 N = 9 C The following starting values provide a rough solution. DO 10 J = 1, 9 FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,NFEV,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 improper input parameters. |
| `INFO` | `1` | 1 relative error between two consecutive iterates is at most XTOL. |
| `INFO` | `2` | 2 number of calls to FCN has reached or exceeded |
| `INFO` | `3` | 3 XTOL is too small. No further improvement in the approximate solution X is possible. |
| `INFO` | `4` | 4 iteration is not making good progress, as measured by the improvement from the last five Jacobian eval- uations. |
| `INFO` | `5` | 5 iteration is not making good progress, as measured by the improvement from the last ten iterations. Sections 4 and 5 contain more details about INFO. nents of D*X may have large relative errors, but the fast rate of convergence of SNSQ usually avoids this possibility. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. 5. Unsuccessful Completion. Unsuccessful termination of SNSQ can be due to improper input parameters, arithmetic interrupts, an excessive number of func- tion evaluations, or lack of good progress. Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1, |
| `INFO` | `>0` | 2, or N .LE. 0, or LDFJAC .LT. N, or |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::nonlinear::snsq`. Native symbol: `snsq_`. Declaration feature: `nonlinear-expert`. Provider feature: `nonlinear-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::nonlinear::snsq`
- Public declaration feature: `nonlinear-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::nonlinear::solve_system_expert_f32`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
