# DNSQ

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find a zero of a system of a N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of DNSQ is to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD and HYBRDJ. 2. Subroutine and Type Statements. SUBROUTINE DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR DOUBLE PRECISION XTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. Parameters. Parameters designated as input parameters must be specified on entry to DNSQ and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNSQ.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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
- Safe Rust paths: `slatec::nonlinear::solve_system_expert, slatec::nonlinear::solve_system_with_jacobian`

## Providers

- Canonical provider: `main-src/src/dnsq.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnsq.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnsq.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnsq.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DNSQ](https://www.netlib.org/slatec/src/dnsq.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates the functions.  FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) 0 are made. is set as follows. |
| 2 | `JAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | supplied subroutine which calculates 2, then the code will approximate the Jacobian by forward-differencing. is set as follows. 2, then NJEV is set to zero.) |
| 3 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG DOUBLE PRECISION X(N),FVEC(N),FJAC(LDFJAC,N) 2, JAC can be ignored (treat it as a dummy argument). is an input variable which specifies how the Jacobian will 1, then the user must supply the 2, then the code will approximate the Jacobian by forward-differencing. 1, set ML to at 1, set MU to at 1, then EPSFCN can be ignored (treat it as a dummy 2, then NJEV is set to zero.) 2, then the validity of the answer can be checked, for example, by rerunning DNSQ with a tighter tolerance. Convergence Test.  If DENORM(Z) denotes the Euclidean norm of a vector Z and D is the diagonal matrix whose entries are defined by the array DIAG, then this test attempts to guarantee that 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as 1).  Unless FCN and JAC can be evaluated quickly, the timing of DNSQ will be strongly influenced by the time spent in FCN and JAC. Storage.  DNSQ requires (3*N**2 + 17*N)/2 single precision storage locations, in addition to the storage required by the program.  There are no internally declared storage arrays. Long Description: 7. Example. The problem is to determine the values of X(1), X(2), ..., X(9), which solve the system of tridiagonal equations 2 |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions and variables. contain an initial estimate of the solution vector.  On output X contains the final estimate of the solution vector. contains the functions evaluated at the output X. contains the orthogonal contains the orthogonal matrix Q produced by the QR factorization of the final matrix Q produced by the QR factorization of the final approximate Jacobian. approximate Jacobian. 1. 1. 1 (see below), DIAG is contains the vector (Q transpose)*FVEC. 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as 9 C C     THE FOLLOWING STARTING VALUES PROVIDE A ROUGH SOLUTION. C DO 10 J = 1, 9 X(K+1) |
| 5 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | contain an initial contain an initial estimate of the solution vector.  On output X contains the estimate of the solution vector.  On output X contains the final estimate of the solution vector. final estimate of the solution vector. XSOL)) .LE. XTOL*DENORM(D*XSOL). -2*X(2)                   = -1 -2*X(2)                   = -1 + (3-2*X(I))*X(I)         -2*X(I+1) = -1, I=2-8 + (3-2*X(9))*X(9) = -1 C     ********** PROGRAM TEST C C     Driver for DNSQ example. C INTEGER J,IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,LR, NWRITE DOUBLE PRECISION XTOL,EPSFCN,FACTOR,FNORM DOUBLE PRECISION X(9),FVEC(9),DIAG(9),FJAC(9,9),R(45),QTF(9), 1.E0 10    CONTINUE C 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) INTEGER K DOUBLE PRECISION ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     INSERT PRINT STATEMENTS HERE WHEN NPRINT IS POSITIVE. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO 1) TEMP2 = ZERO |
| 6 | `FVEC` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | contains the function values at X and should not be altered. contains the functions evaluated at the output X. is set to 2. This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed earlier by DNSQ, TEMP1 - TWO*TEMP2 + ONE 10    CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. Final L2 norm of the residuals  0.1192636E-07 Number of function evaluations        14 Exit parameter                         1 Final approximate solution -0.5706545E+00 -0.6816283E+00 -0.7017325E+00 -0.7042129E+00 -0.7013690E+00 -0.6918656E+00 -0.6657920E+00 -0.5960342E+00 -0.4164121E+00 |
| 7 | `FJAC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDFJAC, *) | contains the function values at X and should not be altered. contains the orthogonal matrix Q produced by the QR factorization of the final approximate Jacobian. |
| 8 | `LDFJAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC. 9 |
| 9 | `XTOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a nonnegative input variable.  Termination occurs when the relative error between two consecutive iterates is at most Therefore, XTOL measures the relative error desired in the approximate solution.  Section 4 contains more details about XTOL. This parameter is used in a test which makes a comparison between the approximation X and a solution XSOL.  DNSQ terminates when the test is satisfied.  If the convergence parameter is less than the machine precision (as defined by the function D1MACH(4)), then DNSQ only attempts to satisfy the test defined by the machine precision.  Further progress is not usually possible. The test assumes that the functions are reasonably well behaved, and, if the Jacobian is supplied by the user, that the functions and the Jacobian are coded consistently.  If these conditions are not satisfied, then DNSQ may incorrectly indicate convergence.  The coding of the Jacobian can be checked by the K), then the larger components of D*X have K significant decimal digits and .LT. 0.E0, or MAXFEV .LE. 0, or ML .LT. 0, or MU .LT. 0, or FACTOR .LE. 0.E0, or LR .LT. (N*(N+1))/2. Arithmetic Interrupts.  If these interrupts occur in the FCN subroutine during an early stage of the computation, they may be caused by an unacceptable choice of X by DNSQ.  In this case, it may be possible to remedy the situation by rerunning DNSQ with a smaller value of FACTOR. Excessive Number of Function Evaluations.  A reasonable value SQRT(D1MACH(4)) C |
| 10 | `MAXFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable.  Termination occurs when the number of calls to FCN is at least MAXFEV by the end of an iteration. 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as 2000 |
| 11 | `ML` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a nonnegative integer input variable which specifies the number of subdiagonals within the band of the Jacobian matrix. 1 |
| 12 | `MU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a nonnegative integer input variable which specifies the number of superdiagonals within the band of the Jacobian 1 |
| 13 | `EPSFCN` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is an input variable used in determining a suitable step for the forward-difference approximation.  This approximation assumes that the relative errors in the functions are of the is less than the machine is less than the machine precision, it is assumed that the relative errors in the precision, it is assumed that the relative errors in the functions are of the order of the machine precision.  If functions are of the order of the machine precision.  If 0.E0 |
| 14 | `DIAG` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1 (see below), DIAG is 1.E0 20    CONTINUE |
| 15 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 (see below), DIAG is 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables. 1, the variables 1, the variables 2, the scaling is specified by the input DIAG.  Other values of MODE are 1. 2 DO 20 J = 1, 9 |
| 16 | `FACTOR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a positive input variable used in determining the initial step bound.  This bound is set to the product of and the Euclidean norm of DIAG*X if nonzero, or else to should lie in the should lie in the interval (.1,100.).  100. is a generally recommended value. interval (.1,100.).  100. is a generally recommended value. 1.E2 |
| 17 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer input variable that enables controlled printing of iterates if it is positive.  In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. appropriate print statements must be added to FCN(see example).  If NPRINT 0 C CALL DNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV,ML,MU, |
| 18 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See is set as follows. 0  Improper input parameters. 1  Relative error between two consecutive iterates is at most XTOL. 2  Number of calls to FCN has reached or exceeded 3  XTOL is too small.  No further improvement in the approximate solution X is possible. 4  Iteration is not making good progress, as measured by the improvement from the last five Jacobian evaluations. 5  Iteration is not making good progress, as measured by the improvement from the last ten iterations. Sections 4 and 5 contain more details about INFO. is set to 1.  There is a danger that the smaller components of D*X may have large relative errors, but the fast rate of convergence of DNSQ usually avoids this possibility. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. 5. Unsuccessful Completion. Unsuccessful termination of DNSQ can be due to improper input parameters, arithmetic interrupts, an excessive number of function evaluations, or lack of good progress. Improper Input Parameters.  INFO is set to 0 if IOPT .LT .1, or IOPT .GT. 2, or N .LE. 0, or LDFJAC .LT. N, or is set to 2. This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed earlier by DNSQ, 4 or INFO = 5. Lack of Good Progress.  DNSQ searches for a zero of the system by minimizing the sum of the squares of the functions.  In so doing, it can become trapped in a region where the minimum does not correspond to a zero of the system and, in this situation, the iteration eventually fails to make good progress.  In particular, this will happen if the system does not have a zero.  If the system has a zero, rerunning DNSQ from a different starting point may be helpful. 6. Characteristics of The Algorithm. DNSQ is a modification of the Powell Hybrid method.  Two of its main characteristics involve the choice of the correction as a convex combination of the Newton and scaled gradient directions, and the updating of the Jacobian by the rank-1 method of Broyden.  The choice of the correction guarantees (under reasonable conditions) global convergence for starting points far from the solution and a fast rate of convergence.  The Jacobian is calculated at the starting point by either the user-supplied subroutine or a forward-difference approximation, but it is not recalculated until the rank-1 method fails to produce satisfactory progress. Timing.  The time required by DNSQ to solve a given problem depends on N, the behavior of the functions, the accuracy requested, and the starting point.  The number of arithmetic operations needed by DNSQ is about 11.5*(N**2) to process each evaluation of the functions (call to FCN) and 1.3*(N**3) to process each evaluation of the Jacobian (call to JAC, 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) INTEGER K DOUBLE PRECISION ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     INSERT PRINT STATEMENTS HERE WHEN NPRINT IS POSITIVE. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO |
| 19 | `NFEV` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable set to the number of calls to 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG DOUBLE PRECISION X(N),FVEC(N) INTEGER K DOUBLE PRECISION ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     INSERT PRINT STATEMENTS HERE WHEN NPRINT IS POSITIVE. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO |
| 20 | `NJEV` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable set to the number of calls to |
| 21 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | contains the upper triangular matrix produced by the QR factorization of the final approximate Jacobian, stored rowwise. FNORM = DENORM(N,FVEC) |
| 22 | `LR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | contains the upper triangular matrix produced by the QR factorization of the final approximate Jacobian, stored rowwise. is a positive integer input variable not less than 45 C C     SET XTOL TO THE SQUARE ROOT OF THE MACHINE PRECISION. C     UNLESS HIGH PRECISION SOLUTIONS ARE REQUIRED, C     THIS IS THE RECOMMENDED SETTING. C FNORM = DENORM(N,FVEC) |
| 23 | `QTF` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | contains the vector (Q transpose)*FVEC. FNORM = DENORM(N,FVEC) |
| 24 | `WA1` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter DOUBLE PRECISION DENORM,D1MACH EXTERNAL FCN DATA NWRITE /6/ C FNORM = DENORM(N,FVEC) |
| 25 | `WA2` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter DOUBLE PRECISION DENORM,D1MACH EXTERNAL FCN DATA NWRITE /6/ C FNORM = DENORM(N,FVEC) |
| 26 | `WA3` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter DOUBLE PRECISION DENORM,D1MACH EXTERNAL FCN DATA NWRITE /6/ C FNORM = DENORM(N,FVEC) |
| 27 | `WA4` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | are work arrays of length N. 4. Successful completion. The accuracy of DNSQ is controlled by the convergence parameter DOUBLE PRECISION DENORM,D1MACH EXTERNAL FCN DATA NWRITE /6/ C FNORM = DENORM(N,FVEC) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::nonlinear::dnsq`. Native symbol: `dnsq_`. Declaration feature: `nonlinear-expert`. Provider feature: `nonlinear-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::nonlinear::dnsq`
- Public declaration feature: `nonlinear-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::nonlinear::solve_system_expert`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
