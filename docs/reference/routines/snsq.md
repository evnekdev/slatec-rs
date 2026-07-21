# SNSQ

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find a zero of a system of a N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of SNSQ is to find a zero of a system of N non- linear functions in N variables by a modification of the Powell hybrid method. The user must provide a subroutine which calcu- lates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD and HYBRDJ. 2. Subroutine and Type Statements. SUBROUTINE SNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV, ML,MU,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV, NJEV,R,LR,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,NJEV,LR REAL XTOL,EPSFCN,FACTOR REAL X(N),FVEC(N),DIAG(N),FJAC(LDFJAC,N),R(LR),QTF(N), WA1(N),WA2(N),WA3(N),WA4(N) EXTERNAL FCN,JAC 3. Parameters.

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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [SNSQ](https://www.netlib.org/slatec/src/snsq.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates the functions.  FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) ---------- Calculate the functions at X and return this vector in FVEC. ---------- RETURN END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of SNSQ.  In this case, set IFLAG to a negative integer. |
| 2 | `JAC` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | supplied subroutine which calculates 2, then the code will approximate the Jacobian by forward-differencing. N is a positive integer input variable set to the number of functions and variables. X is an array of length N.  On input, X must contain an initial estimate of the solution vector.  On output, X contains the final estimate of the solution vector. FVEC is an output array of length N which contains the functions evaluated at the output X. FJAC is an output N by N array which contains the orthogonal matrix Q produced by the QR factorization of the final approx- imate Jacobian. LDFJAC is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC. |
| 3 | `IOPT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG REAL X(N),FVEC(N),FJAC(LDFJAC,N) ---------- Calculate the Jacobian at X and return this matrix in FJAC.  FVEC contains the function values at X and should not be altered. ---------- RETURN END The value of IFLAG should not be changed by JAC unless the user wants to terminate execution of SNSQ.  In this case, set IFLAG to a negative integer. 2, JAC can be ignored (treat it as a dummy argument). IOPT is an input variable which specifies how the Jacobian will 1, then the user must supply the 2, then the code will approximate the Jacobian by forward-differencing. N is a positive integer input variable set to the number of functions and variables. X is an array of length N.  On input, X must contain an initial estimate of the solution vector.  On output, X contains the final estimate of the solution vector. FVEC is an output array of length N which contains the functions evaluated at the output X. FJAC is an output N by N array which contains the orthogonal matrix Q produced by the QR factorization of the final approx- imate Jacobian. LDFJAC is a positive integer input variable not less than N which specifies the leading dimension of the array FJAC. 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as measured by the progress of FVEC, and INFO is set to 2.  This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed earlier by SNSQ, 1).  Unless FCN and JAC can be evaluated quickly, the timing of SNSQ will be strongly influenced by the time spent in FCN and JAC. Storage.  SNSQ requires (3*N**2 + 17*N)/2 single precision storage locations, in addition to the storage required by the program.  There are no internally declared storage arrays. 7. Example. The problem is to determine the values of X(1), X(2), ..., X(9), which solve the system of tridiagonal equations (3-2*X(1))*X(1)           -2*X(2)                   = -1 -X(I-1) + (3-2*X(I))*X(I)         -2*X(I+1) = -1, I=2-8 -X(8) + (3-2*X(9))*X(9) = -1 C     ********** PROGRAM TEST C C     Driver for SNSQ example. C INTEGER J,IOPT,N,MAXFEV,ML,MU,MODE,NPRINT,INFO,NFEV,LDFJAC,LR, NWRITE REAL XTOL,EPSFCN,FACTOR,FNORM REAL X(9),FVEC(9),DIAG(9),FJAC(9,9),R(45),QTF(9), WA1(9),WA2(9),WA3(9),WA4(9) REAL ENORM,R1MACH EXTERNAL FCN DATA NWRITE /6/ C 2 |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as measured by the progress of FVEC, and INFO is set to 2.  This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed earlier by SNSQ, 9 C C     The following starting values provide a rough solution. C DO 10 J = 1, 9 X(K+1) |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1.E0 10    CONTINUE C 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO 1) TEMP2 = ZERO |
| 6 | `FVEC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | TEMP1 - TWO*TEMP2 + ONE 10    CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS  0.1192636E-07 NUMBER OF FUNCTION EVALUATIONS        14 EXIT PARAMETER                         1 FINAL APPROXIMATE SOLUTION -0.5706545E+00 -0.6816283E+00 -0.7017325E+00 -0.7042129E+00 -0.7013690E+00 -0.6918656E+00 -0.6657920E+00 -0.5960342E+00 -0.4164121E+00 |
| 7 | `FJAC` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDFJAC, *) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `LDFJAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 9 |
| 9 | `XTOL` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | negative input variable.  Termination occurs when SQRT(R1MACH(4)) C |
| 10 | `MAXFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 and 200*(N+1) for IOPT=2. If the number of calls to FCN reaches MAXFEV, then this indicates that the routine is converging very slowly as measured by the progress of FVEC, and INFO is set to 2.  This situation should be unusual because, as indicated below, lack of good progress is usually diagnosed earlier by SNSQ, 2000 |
| 11 | `ML` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 |
| 12 | `MU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 |
| 13 | `EPSFCN` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 0.E0 |
| 14 | `DIAG` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1.E0 20    CONTINUE |
| 15 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2 DO 20 J = 1, 9 |
| 16 | `FACTOR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | 1.E2 |
| 17 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 C CALL SNSQ(FCN,JAC,IOPT,N,X,FVEC,FJAC,LDFJAC,XTOL,MAXFEV,ML,MU, EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO,NFEV,NJEV, R,LR,QTF,WA1,WA2,WA3,WA4) FNORM = ENORM(N,FVEC) |
| 18 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 4 or INFO = 5. Lack of Good Progress.  SNSQ searches for a zero of the system by minimizing the sum of the squares of the functions.  In so doing, it can become trapped in a region where the minimum does not correspond to a zero of the system and, in this situ- ation, the iteration eventually fails to make good progress. In particular, this will happen if the system does not have a zero.  If the system has a zero, rerunning SNSQ from a dif- ferent starting point may be helpful. 6. Characteristics of the Algorithm. SNSQ is a modification of the Powell hybrid method.  Two of its main characteristics involve the choice of the correction as a convex combination of the Newton and scaled gradient directions, and the updating of the Jacobian by the rank-1 method of Broy- den.  The choice of the correction guarantees (under reasonable conditions) global convergence for starting points far from the solution and a fast rate of convergence.  The Jacobian is calculated at the starting point by either the user-supplied subroutine or a forward-difference approximation, but it is not recalculated until the rank-1 method fails to produce satis- factory progress. Timing.  The time required by SNSQ to solve a given problem depends on N, the behavior of the functions, the accuracy requested, and the starting point.  The number of arithmetic operations needed by SNSQ is about 11.5*(N**2) to process each evaluation of the functions (call to FCN) and 1.3*(N**3) to process each evaluation of the Jacobian (call to JAC, 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO |
| 19 | `NFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' NUMBER OF FUNCTION EVALUATIONS',I10 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15.7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0.E0,1.E0,2.E0,3.E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE DO 10 K = 1, N TEMP = (THREE - TWO*X(K))*X(K) TEMP1 = ZERO |
| 20 | `NJEV` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 21 | `R` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (LR) | Array argument classified by fixed-form executable read/write analysis. |
| 22 | `LR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 45 C C     Set XTOL to the square root of the machine precision. C     Unless high precision solutions are required, C     this is the recommended setting. C |
| 23 | `QTF` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 24 | `WA1` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 25 | `WA2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 26 | `WA3` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 27 | `WA4` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

the approximate solution.  Section 4 contains more details about XTOL. MAXFEV is a positive integer input variable.  Termination occurs when the number of calls to FCN is at least MAXFEV by the end of an iteration. ML is a non-negative integer input variable which specifies the number of subdiagonals within the band of the Jacobian matrix. If the Jacobian is not banded or IOPT=1, set ML to at least N - 1. MU is a non-negative integer input variable which specifies the number of superdiagonals within the band of the Jacobian matrix.  If the Jacobian is not banded or IOPT=1, set MU to at least N - 1. EPSFCN is an input variable used in determining a suitable step for the forward-difference approximation.  This approximation order of EPSFCN.  If EPSFCN is less than the machine preci- are of the order of the machine precision.  If IOPT=1, then EPSFCN can be ignored (treat it as a dummy argument). DIAG is an array of length N.  If MODE = 1 (see below), DIAG is internally set.  If MODE = 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables. MODE is an integer input variable.  If MODE = 1, the variables will be scaled internally.  If MODE = 2, the scaling is speci- fied by the input DIAG.  Other values of MODE are equivalent to MODE = 1. FACTOR is a positive input variable used in determining the ini- tial step bound.  This bound is set to the product of FACTOR and the Euclidean norm of DIAG*X if nonzero, or else to FACTOR itself.  In most cases FACTOR should lie in the interval (.1,100.).  100. is a generally recommended value. NPRINT is an integer input variable that enables controlled printing of iterates if it is positive.  In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iteration thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN(see example).  If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made. INFO is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See description of FCN and JAC. Otherwise, INFO is set as follows. at most XTOL. INFO = 2  number of calls to FCN has reached or exceeded MAXFEV. INFO = 3  XTOL is too small.  No further improvement in the approximate solution X is possible. INFO = 4  iteration is not making good progress, as measured by the improvement from the last five Jacobian eval- uations. INFO = 5  iteration is not making good progress, as measured by the improvement from the last ten iterations. Sections 4 and 5 contain more details about INFO. NFEV is an integer output variable set to the number of calls to FCN. NJEV is an integer output variable set to the number of calls to JAC. (If IOPT=2, then NJEV is set to zero.) R is an output array of length LR which contains the upper triangular matrix produced by the QR factorization of the final approximate Jacobian, stored rowwise. LR is a positive integer input variable not less than (N*(N+1))/2. QTF is an output array of length N which contains the vector (Q TRANSPOSE)*FVEC. WA1, WA2, WA3, and WA4 are work arrays of length N. 4. Successful Completion. The accuracy of SNSQ is controlled by the convergence parameter XTOL.  This parameter is used in a test which makes a comparison between the approximation X and a solution XSOL.  SNSQ termi- nates when the test is satisfied.  If the convergence parameter is less than the machine precision (as defined by the function R1MACH(4)), then SNSQ only attempts to satisfy the test defined by the machine precision.  Further progress is not usually possible. The test assumes that the functions are reasonably well behaved, and, if the Jacobian is supplied by the user, that the functions and the Jacobian are coded consistently.  If these conditions are not satisfied, then SNSQ may incorrectly indicate conver- gence.  The coding of the Jacobian can be checked by the subroutine CHKDER. If the Jacobian is coded correctly or IOPT=2, then the validity of the answer can be checked, for example, by rerunning SNSQ with a tighter tolerance. Convergence Test.  If ENORM(Z) denotes the Euclidean norm of a vector Z and D is the diagonal matrix whose entries are defined by the array DIAG, then this test attempts to guaran- tee that ENORM(D*(X-XSOL)) .LE. XTOL*ENORM(D*XSOL). If this condition is satisfied with XTOL = 10**(-K), then the larger components of D*X have K significant decimal digits and INFO is set to 1.  There is a danger that the smaller compo- of convergence of SNSQ usually avoids this possibility. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. 5. Unsuccessful Completion. Unsuccessful termination of SNSQ can be due to improper input parameters, arithmetic interrupts, an excessive number of func- tion evaluations, or lack of good progress.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
