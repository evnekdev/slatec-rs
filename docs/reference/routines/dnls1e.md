# DNLS1E

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

An easy-to-use code which minimizes the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm.

## Description

1. Purpose. The purpose of DNLS1E is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. This is done by using the more general least-squares solver DNLS1. The user must provide a subroutine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER1, LMDIF1, and LMSTR1. 2. Subroutine and Type Statements. SUBROUTINE DNLS1E(FCN,IOPT,M,N,X,FVEC,TOL,NPRINT, INFO,IW,WA,LWA) INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1B1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::least_squares::least_squares`

## Providers

- Canonical provider: `main-src/src/dnls1e.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnls1e.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnls1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnls1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DNLS1E](https://www.netlib.org/slatec/src/dnls1e.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | supplied subroutine which calculates the functions.  If the user wants to supply the Jacobian 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) 2 or 3 or 200*(N+1) 1, C     that is, if the user does not calculate the Jacobian. INTEGER I,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 2, C     that is, if the user calculates the full Jacobian. C INTEGER I,LDFJAC,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE IF(IFLAG.NE.1) GO TO 20 DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 3, C     that is, if the user calculates the Jacobian row by row. INTEGER I,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),FJAC(N),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE IF( IFLAG.NE.1) GO TO 20 DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 |
| 2 | `IOPT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions.  See the explanation of the IOPT argument below. If the user wants the iterates printed (NPRINT positive), then FCN must do the printing.  See the explanation of NPRINT below.  FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N DOUBLE PRECISION X(N),FVEC(M) ---------- 1. 2. 3. ---------- If IFLAG=0, the values in X and FVEC are available for printing.  See the explanation of NPRINT below. IFLAG will never be zero unless NPRINT is positive. The values of X and FVEC must not be changed. RETURN ---------- If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN ---------- If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC.  Note that IFLAG will never be 2 unless 2.  FVEC contains the function values at X and must not be altered.  FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN ---------- If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC.  Note that IFLAG will 3.  FVEC contains the function values at X and must not be altered.  FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN ---------- END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of DNLS1E.  In this case, set IFLAG to a negative integer. IOPT is an input variable which specifies how the Jacobian will 2 or 3, then the user must supply the Jacobian, as well as the function values, through the 2, the user supplies the full 3, the user supplies one row of the Jacobian with each call.  (In this manner, storage can be saved because the full Jacobian is not stored.) 1, the code will approximate the Jacobian by forward differencing. M is a positive integer input variable set to the number of functions. N is a positive integer input variable set to the number of variables.  N must not exceed M. X is an array of length N.  On input, X must contain an initial estimate of the solution vector.  On output, X contains the final estimate of the solution vector. FVEC is an output array of length M which contains the functions evaluated at the output X. 1 or 2 LWA .LT. N*(M+5)+M, or for IOPT=3 LWA .LT. N*(N+5)+M. Arithmetic Interrupts.  If these interrupts occur in the FCN subroutine during an early stage of the computation, they may be caused by an unacceptable choice of X by DNLS1E.  In this case, it may be possible to remedy the situation by not evalu- ating the functions here, but instead setting the components of FVEC to numbers that exceed those in the initial FVEC. Excessive Number of Function Evaluations.  If the number of 2 or 3 or 200*(N+1) 1, then this indicates that the routine is converging very slowly as measured by the progress of FVEC, and INFO is set to 5.  In this case, it may be helpful to restart DNLS1E, thereby forcing it to disregard old (and possibly harmful) information. 6. Characteristics of the Algorithm. DNLS1E is a modification of the Levenberg-Marquardt algorithm. Two of its main characteristics involve the proper use of implicitly scaled variables and an optimal choice for the cor- rection.  The use of implicitly scaled variables achieves scale invariance of DNLS1E and limits the size of the correction in any direction where the functions are changing rapidly.  The optimal choice of the correction guarantees (under reasonable conditions) global convergence from starting points far from the solution and a fast rate of convergence for problems with small residuals. Timing.  The time required by DNLS1E to solve a given problem 2 1 (N calls to FCN) and 3 (M calls to FCN).  Unless FCN can be evaluated quickly, the timing of DNLS1E will be strongly influenced by the time spent in FCN. 1 or 2 and 3 single precision storage locations and N integer storage locations, in addition to the storage required by the program.  There are no internally declared storage arrays. Long Description: 7. Example. The problem is to determine the values of X(1), X(2), and X(3) which provide the best fit (in the least squares sense) of 1 1, C     that is, if the user does not calculate the Jacobian. INTEGER I,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 2, FCN would be modified as follows to also calculate the full Jacobian when IFLAG=2. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) C 2, C     that is, if the user calculates the full Jacobian. C INTEGER I,LDFJAC,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE IF(IFLAG.NE.1) GO TO 20 DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 3, FJAC would be dimensioned as FJAC(3,3), LDFJAC would be set to 3, and FCN would be written as follows to calculate a row of the Jacobian when IFLAG=3. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) 3, C     that is, if the user calculates the Jacobian row by row. INTEGER I,M,N,IFLAG DOUBLE PRECISION X(N),FVEC(M),FJAC(N),Y(15) DOUBLE PRECISION TMP1,TMP2,TMP3,TMP4 DATA Y(1),Y(2),Y(3),Y(4),Y(5),Y(6),Y(7),Y(8), Y(9),Y(10),Y(11),Y(12),Y(13),Y(14),Y(15) /1.4E-1,1.8E-1,2.2E-1,2.5E-1,2.9E-1,3.2E-1,3.5E-1,3.9E-1, 3.7E-1,5.8E-1,7.3E-1,9.6E-1,1.34E0,2.1E0,4.39E0/ C IF (IFLAG .NE. 0) GO TO 5 C C     Insert print statements here when NPRINT is positive. C RETURN 5 CONTINUE IF( IFLAG.NE.1) GO TO 20 DO 10 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | racy requested, and the starting point.  The number of arith- metic operations needed by DNLS1E is about N**3 to process each evaluation of the functions (call to FCN) and to process 2 1 (N calls to FCN) and 3 (M calls to FCN).  Unless FCN can be evaluated quickly, the timing of DNLS1E will be strongly influenced by the time spent in FCN. 1 or 2 and 1 or 2 and 3 single precision storage locations and N integer storage locations, in addition to the storage required by the program.  There are no internally declared storage arrays. Long Description: 7. Example. The problem is to determine the values of X(1), X(2), and X(3) which provide the best fit (in the least squares sense) of 15 |
| 4 | `N` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2. 3. ---------- If IFLAG=0, the values in X and FVEC are available for printing.  See the explanation of NPRINT below. IFLAG will never be zero unless NPRINT is positive. The values of X and FVEC must not be changed. RETURN ---------- If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN ---------- If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC.  Note that IFLAG will never be 2 unless 2 or 3 or 200*(N+1) racy requested, and the starting point.  The number of arith- metic operations needed by DNLS1E is about N**3 to process each evaluation of the functions (call to FCN) and to process 2 1 (N calls to FCN) and 3 (M calls to FCN).  Unless FCN can be evaluated quickly, the timing of DNLS1E will be strongly influenced by the time spent in FCN. 1 or 2 and 1 or 2 and 3 single precision storage 3 single precision storage locations and N integer storage locations, in addition to locations and N integer storage locations, in addition to the storage required by the program.  There are no internally the storage required by the program.  There are no internally declared storage arrays. declared storage arrays. Long Description: Long Description: 7. Example. 7. Example. The problem is to determine the values of X(1), X(2), and X(3) The problem is to determine the values of X(1), X(2), and X(3) which provide the best fit (in the least squares sense) of which provide the best fit (in the least squares sense) of 3 C C     The following starting values provide a rough fit. C |
| 5 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1, 15 1, 15 1, 15 to the data to the data to the data Y = (0.14,0.18,0.22,0.25,0.29,0.32,0.35,0.39, Y = (0.14,0.18,0.22,0.25,0.29,0.32,0.35,0.39, Y = (0.14,0.18,0.22,0.25,0.29,0.32,0.35,0.39, 0.37,0.58,0.73,0.96,1.34,2.10,4.39), 0.37,0.58,0.73,0.96,1.34,2.10,4.39), 0.37,0.58,0.73,0.96,1.34,2.10,4.39), where U(I) = I, V(I) = 16 - I, and W(I) = MIN(U(I),V(I)).  The where U(I) = I, V(I) = 16 - I, and W(I) = MIN(U(I),V(I)).  The where U(I) = I, V(I) = 16 - I, and W(I) = MIN(U(I),V(I)).  The I-th component of FVEC is thus defined by I-th component of FVEC is thus defined by I-th component of FVEC is thus defined by Y(I) - (X(1) + U(I)/(V(I)*X(2) + W(I)*X(3))). Y(I) - (X(1) + U(I)/(V(I)*X(2) + W(I)*X(3))). Y(I) - (X(1) + U(I)/(V(I)*X(2) + W(I)*X(3))). PROGRAM TEST PROGRAM TEST PROGRAM TEST C C C C     Driver for DNLS1E example. C     Driver for DNLS1E example. C     Driver for DNLS1E example. C C C INTEGER I,IOPT,M,N,NPRINT,JNFO,LWA,NWRITE INTEGER I,IOPT,M,N,NPRINT,JNFO,LWA,NWRITE INTEGER I,IOPT,M,N,NPRINT,JNFO,LWA,NWRITE INTEGER IW(3) INTEGER IW(3) INTEGER IW(3) DOUBLE PRECISION TOL,FNORM,X(3),FVEC(15),WA(75) DOUBLE PRECISION TOL,FNORM,X(3),FVEC(15),WA(75) DOUBLE PRECISION TOL,FNORM,X(3),FVEC(15),WA(75) DOUBLE PRECISION DENORM,D1MACH DOUBLE PRECISION DENORM,D1MACH DOUBLE PRECISION DENORM,D1MACH EXTERNAL FCN EXTERNAL FCN EXTERNAL FCN DATA NWRITE /6/ DATA NWRITE /6/ DATA NWRITE /6/ C C C 1.E0 1.E0 1.E0 C 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' EXIT 5X,' FINAL APPROXIMATE SOLUTION' // 5X,3E15.7) END SUBROUTINE FCN(IFLAG,M,N,X,FVEC,DUM,IDUM) |
| 6 | `FVEC` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | (X(1) + TMP1/(X(2)*TMP2 + X(3)*TMP3)) 10    CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS  0.9063596E-01 EXIT PARAMETER                         1 FINAL APPROXIMATE SOLUTION 0.8241058E-01  0.1133037E+01  0.2343695E+01 (X(1) + TMP1/(X(2)*TMP2 + X(3)*TMP3)) 10    CONTINUE RETURN C C     Below, calculate the full Jacobian. C 20    CONTINUE C DO 30 I = 1, M TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 TMP4 = (X(2)*TMP2 + X(3)*TMP3)**2 FJAC(I,1) = -1.E0 FJAC(I,2) = TMP1*TMP2/TMP4 FJAC(I,3) = TMP1*TMP3/TMP4 30    CONTINUE RETURN END (X(1) + TMP1/(X(2)*TMP2 + X(3)*TMP3)) 10    CONTINUE RETURN C C     Below, calculate the LDFJAC-th row of the Jacobian. C 20 CONTINUE I = LDFJAC TMP1 = I TMP2 = 16 - I TMP3 = TMP1 IF (I .GT. 8) TMP3 = TMP2 TMP4 = (X(2)*TMP2 + X(3)*TMP3)**2 FJAC(1) = -1.E0 FJAC(2) = TMP1*TMP2/TMP4 FJAC(3) = TMP1*TMP3/TMP4 RETURN END |
| 7 | `TOL` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | negative input variable.  Termination occurs when SQRT(R1MACH(4)) C CALL DNLS1E(FCN,IOPT,M,N,X,FVEC,TOL,NPRINT, INFO,IW,WA,LWA) FNORM = ENORM(M,FVEC) |
| 8 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 C C     Set TOL to the square root of the machine precision. C     Unless high precision solutions are required, C     this is the recommended setting. C |
| 9 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15.7 // 5X,' EXIT 5X,' FINAL APPROXIMATE SOLUTION' // 5X,3E15.7) END SUBROUTINE FCN(IFLAG,M,N,X,FVEC,DUM,IDUM) |
| 10 | `IW` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `WA` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 12 | `LWA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 75 |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

between X and the solution is at most TOL.  Section 4 contains more details about TOL. NPRINT is an integer input variable that enables controlled printing of iterates if it is positive.  In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example) and FVEC should not be altered.  If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made. INFO is an integer output variable.  If the user has terminated execution, INFO is set to the (negative) value of IFLAG.  See description of FCN and JAC. Otherwise, INFO is set as follows. sum of squares is at most TOL. X and the solution is at most TOL. INFO = 3  conditions for INFO = 1 and INFO = 2 both hold. INFO = 4  FVEC is orthogonal to the columns of the Jacobian to machine precision. INFO = 5  number of calls to FCN has reached 100*(N+1) for IOPT=2 or 3 or 200*(N+1) for IOPT=1. INFO = 6  TOL is too small.  No further reduction in the sum of squares is possible. INFO = 7  TOL is too small.  No further improvement in the approximate solution X is possible. Sections 4 and 5 contain more details about INFO. IW is an INTEGER work array of length N. WA is a work array of length LWA. LWA is a positive integer input variable not less than N*(M+5)+M for IOPT=1 and 2 or N*(N+5)+M for IOPT=3. 4. Successful Completion. The accuracy of DNLS1E is controlled by the convergence parame- ter TOL.  This parameter is used in tests which make three types of comparisons between the approximation X and a solution XSOL. DNLS1E terminates when any of the tests is satisfied.  If TOL is less than the machine precision (as defined by the function R1MACH(4)), then DNLS1E only attempts to satisfy the test defined by the machine precision.  Further progress is not usu- ally possible.  Unless high precision solutions are required, the recommended value for TOL is the square root of the machine precision. The tests assume that the functions are reasonably well behaved, and, if the Jacobian is supplied by the user, that the functions and the Jacobian are coded consistently.  If these conditions are not satisfied, then DNLS1E may incorrectly indicate conver- gence.  If the Jacobian is coded correctly or IOPT=1, then the validity of the answer can be checked, for example, by rerunning DNLS1E with tighter tolerances. First Convergence Test.  If ENORM(Z) denotes the Euclidean norm of a vector Z, then this test attempts to guarantee that ENORM(FVEC) .LE. (1+TOL)*ENORM(FVECS), where FVECS denotes the functions evaluated at XSOL.  If this condition is satisfied with TOL = 10**(-K), then the final residual norm ENORM(FVEC) has K significant decimal digits and INFO is set to 1 (or to 3 if the second test is also satis- fied). Second Convergence Test.  If D is a diagonal matrix (implicitly generated by DNLS1E) whose entries contain scale factors for the variables, then this test attempts to guarantee that ENORM(D*(X-XSOL)) .LE.  TOL*ENORM(D*XSOL). If this condition is satisfied with TOL = 10**(-K), then the larger components of D*X have K significant decimal digits and INFO is set to 2 (or to 3 if the first test is also satis- fied).  There is a danger that the smaller components of D*X that the accuracy of the components of X is usually related to their sensitivity. Third Convergence Test.  This test is satisfied when FVEC is orthogonal to the columns of the Jacobian to machine preci- sion.  There is no clear relationship between this test and the accuracy of DNLS1E, and furthermore, the test is equally well satisfied at other critical points, namely maximizers and saddle points.  Therefore, termination caused by this test (INFO = 4) should be examined carefully. 5. Unsuccessful Completion. Unsuccessful termination of DNLS1E can be due to improper input parameters, arithmetic interrupts, or an excessive number of function evaluations.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::least_squares::dnls1e`. Native symbol: `dnls1e_`. Declaration feature: `least-squares-nonlinear-easy`. Provider feature: `least-squares-nonlinear-easy`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::least_squares::dnls1e`
- Public declaration feature: `least-squares-nonlinear-easy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::least_squares::least_squares`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
