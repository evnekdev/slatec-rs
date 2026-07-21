# SNSQE

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

An easy-to-use code to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose.

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
- Safe Rust paths: `slatec::nonlinear::solve_system_f32`

## Providers

- Canonical provider: `main-src/src/snsqe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/snsqe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/snsqe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/snsqe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SNSQE](https://www.netlib.org/slatec/src/snsqe.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates the functions.  FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) 0 are made. is set as follows. 1 or 200*(N+1) for |
| 2 | `JAC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates 2, then the code will approximate the Jacobian by forward-differencing. is set as follows. |
| 3 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG REAL X(N),FVEC(N),FJAC(LDFJAC,N) 2, JAC can be ignored (treat it as a dummy argument). is an input variable which specifies how the Jacobian will 1, then the user must supply the 2, then the code will approximate the Jacobian by forward-differencing. 1 or 200*(N+1) for IOPT=2. 2, then the validity of the answer can be checked, for example, by rerunning SNSQE with a tighter tolerance. Convergence Test.  If ENORM(Z) denotes the Euclidean norm of a vector Z, then this test attempts to guarantee that .GT. 2, or N .LE. 0, or TOL .LT. 0.E0, or 1 or 200*(N+1) for 2, then this indicates that the routine is converging very slowly as measured by the progress of FVEC, and INFO is set to 2.  This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed 1 and supply the Jacobian. Lack of Good Progress.  SNSQE searches for a zero of the system by minimizing the sum of the squares of the functions.  In so doing, it can become trapped in a region where the minimum does not correspond to a zero of the system and, in this situ- ation, the iteration eventually fails to make good progress. In particular, this will happen if the system does not have a zero.  If the system has a zero, rerunning SNSQE from a dif- ferent starting point may be helpful. 6. Characteristics of the Algorithm. SNSQE is a modification of the Powell hybrid method.  Two of its main characteristics involve the choice of the correction as a convex combination of the Newton and scaled gradient direc- tions, and the updating of the Jacobian by the rank-1 method of Broyden.  The choice of the correction guarantees (under reason- able conditions) global convergence for starting points far from the solution and a fast rate of convergence.  The Jacobian is calculated at the starting point by either the user-supplied subroutine or a forward-difference approximation, but it is not recalculated until the rank-1 method fails to produce satis- factory progress. Timing.  The time required by SNSQE to solve a given problem depends on N, the behavior of the functions, the accuracy requested, and the starting point.  The number of arithmetic operations needed by SNSQE is about 11.5*(N**2) to process each evaluation of the functions (call to FCN) and 1.3*(N**3) to process each evaluation of the Jacobian (call to JAC, 1).  Unless FCN and JAC can be evaluated quickly, the timing of SNSQE will be strongly influenced by the time spent in FCN and JAC. Storage.  SNSQE requires (3*N**2 + 17*N)/2 single precision storage locations, in addition to the storage required by the program.  There are no internally declared storage arrays. 7. Example. The problem is to determine the values of X(1), X(2), ..., X(9), which solve the system of tridiagonal equations 2 |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | linear functions in N variables by a modification of the Powell hybrid method.  This is done by using the more general nonlinear equation solver SNSQ.  The user must provide a subroutine which calculates the functions.  The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation.  This code is the combination of the MINPACK codes (Argonne) HYBRD1 and HYBRJ1. 2. Subroutine and Type Statements. SUBROUTINE SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO, is a positive integer input variable set to the number of functions and variables. contain an initial estimate of the solution vector.  On output, X contains the final estimate of the solution vector. contains the functions evaluated at the output X. 1 or 200*(N+1) for IOPT=2. 4. Successful Completion. 4. Successful Completion. The accuracy of SNSQE is controlled by the convergence parame- The accuracy of SNSQE is controlled by the convergence parame- 1 or 200*(N+1) for 9 C C     The following starting values provide a rough solution. C DO 10 J = 1, 9 X(K+1) |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain an initial contain an initial estimate of the solution vector.  On output, X contains the estimate of the solution vector.  On output, X contains the final estimate of the solution vector. final estimate of the solution vector. and the solution is at most TOL. XSOL) .LE.  TOL*ENORM(XSOL). -2*X(2)                   = -1 -2*X(2)                   = -1 + (3-2*X(I))*X(I)         -2*X(I+1) = -1, I=2-8 + (3-2*X(9))*X(9) = -1 PROGRAM TEST C C     Driver for SNSQE example. C INTEGER J,N,IOPT,NPRINT,INFO,LWA,NWRITE REAL TOL,FNORM REAL X(9),FVEC(9),WA(180) REAL ENORM,R1MACH EXTERNAL FCN DATA NWRITE /6/ C 1.E0 10    CONTINUE 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO 1) TEMP2 = ZERO |
| 6 | `FVEC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the functions evaluated at the output X. TEMP1 - TWO*TEMP2 + ONE 10    CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS  0.1192636E-07 EXIT PARAMETER                         1 FINAL APPROXIMATE SOLUTION -0.5706545E+00 -0.6816283E+00 -0.7017325E+00 -0.7042129E+00 -0.7013690E+00 -0.6918656E+00 -0.6657920E+00 -0.5960342E+00 -0.4164121E+00 |
| 7 | `TOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | negative input variable.  Termination occurs when the algorithm estimates that the relative error between X and the solution is at most TOL.  Section 4 contains more details about TOL. ison between the approximation X and a solution XSOL.  SNSQE terminates when the test is satisfied.  If TOL is less than the machine precision (as defined by the function R1MACH(4)), then SNSQE attempts only to satisfy the test defined by the machine precision.  Further progress is not usually possible.  Unless high precision solutions are required, the recommended value for TOL is the square root of the machine precision. The test assumes that the functions are reasonably well behaved, and, if the Jacobian is supplied by the user, that the functions and the Jacobian  coded consistently.  If these conditions are not satisfied, SNSQE may incorrectly indicate convergence. The coding of the Jacobian can be checked by the subroutine K), then the larger components of X have K significant decimal digits and SQRT(R1MACH(4)) C CALL SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO,WA,LWA) FNORM = ENORM(N,FVEC) |
| 8 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer input variable that enables controlled printing of iterates if it is positive.  In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iteration thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example). If NPRINT 0 C C     Set TOL to the square root of the machine precision. C     Unless high precision solutions are required, C     this is the recommended setting. C |
| 9 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See is set as follows. 0  improper input parameters. 1  algorithm estimates that the relative error between 2  number of calls to FCN has reached or exceeded 3  TOL is too small.  No further improvement in the approximate solution X is possible. 4  iteration is not making good progress. Sections 4 and 5 contain more details about INFO. nents of X may have large relative errors, but the fast rate of convergence of SNSQE usually avoids this possibility. 5. Unsuccessful Completion. Unsuccessful termination of SNSQE can be due to improper input parameters, arithmetic interrupts, an excessive number of func- tion evaluations, errors in the functions, or lack of good prog- ress. Improper Input Parameters.  INFO is set to 0 if IOPT .LT. 1, or 4. 4).  The user should 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO |
| 10 | `WA` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (LWA) | INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC 3. Parameters. Parameters designated as input parameters must be specified on entry to SNSQE and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from SNSQE. is a work array of length LWA. |
| 11 | `LWA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC 3. Parameters. Parameters designated as input parameters must be specified on entry to SNSQE and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from SNSQE. is a positive integer input variable not less than .LT. (3*N**2+13*N)/2. Arithmetic Interrupts.  If these interrupts occur in the FCN subroutine during an early stage of the computation, they may be caused by an unacceptable choice of X by SNSQE.  In this case, it may be possible to remedy the situation by not evalu- ating the functions here, but instead setting the components of FVEC to numbers that exceed those in the initial FVEC. Excessive Number of Function Evaluations.  If the number of 180 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

in the forward-difference approximation to the Jacobian assumes that the relative errors in the functions are of the order of the machine precision.  If this is not the case,

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::nonlinear::snsqe`. Native symbol: `snsqe_`. Declaration feature: `nonlinear-easy`. Provider feature: `nonlinear-easy`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::nonlinear::snsqe`
- Public declaration feature: `nonlinear-easy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::nonlinear::solve_system_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
