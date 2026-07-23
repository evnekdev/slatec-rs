# DNLS1

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm.

## Description

1. Purpose. The purpose of DNLS1 is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. The user must provide a subrou- tine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER, LMDIF, and LMSTR. 2. Subroutine and Type Statements. SUBROUTINE DNLS1(FCN,IOPT,M,N,X,FVEC,FJAC,LDFJAC,FTOL,XTOL,

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
- Safe Rust paths: `slatec::least_squares::least_squares_expert, slatec::least_squares::least_squares_with_jacobian`

## Providers

- Canonical provider: `main-src/src/dnls1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnls1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnls1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnls1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DNLS1](https://www.netlib.org/slatec/src/dnls1.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | is the name of the user-supplied subroutine which calculate the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed (NPRINT positive), then must do the printing. See the explanation of NPRINT below. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. |
| 2 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). |
| 3 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of variables. N must not exceed M. |
| 5 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector. |
| 6 | `FVEC` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is an output array of length M which contains the functions evaluated at the output X. should not be altered. If NPRINT is not positive, no special calls to FCN with IFLAG = 0 are made. |
| 7 | `FJAC` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDFJAC, *) | is an output array. For IOPT=1 and 2, FJAC is an M by N array. For IOPT=3, FJAC is an N by N array. The upper N by N submatrix of FJAC contains an upper triangular matrix R with diagonal elements of nonincreasing magnitude such that T T T P *(JAC *JAC)*P = R *R, where P is a permutation matrix and JAC is the final calcu- lated Jacobian. Column J of P is column IPVT(J) (see below) of the identity matrix. The lower part of FJAC contains information generated during the computation of R. |
| 8 | `LDFJAC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable which specifies the leading dimension of the array FJAC. For IOPT=1 and 2, must not be less than M. For IOPT=3, LDFJAC must not be less than N. LT. M, or for IOPT=3 LDFJAC. N, or FTOL. |
| 9 | `FTOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a non-negative input variable. Termination occurs when both the actual and predicted relative reductions in the sum of squares are at most FTOL. Therefore, FTOL measures the relative error desired in the sum of squares. Section 4 con- tains more details about FTOL. |
| 10 | `XTOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a non-negative input variable. Termination occurs when the relative error between two consecutive iterates is at most Therefore, XTOL measures the relative error desired in the approximate solution. Section 4 contains more details about XTOL. |
| 11 | `GTOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a non-negative input variable. Termination occurs when the cosine of the angle between FVEC and any column of the Jacobian is at most GTOL in absolute value. Therefore, GTOL measures the orthogonality desired between the function vector and the columns of the Jacobian. Section 4 contains more details about GTOL. |
| 12 | `MAXFEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a positive integer input variable. Termination occurs when the number of calls to FCN to evaluate the functions has reached MAXFEV. |
| 13 | `EPSFCN` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an input variable used in determining a suitable step for the forward-difference approximation. This approximation assumes that the relative errors in the functions are of the order of EPSFCN. If EPSFCN is less than the machine preci- sion, it is assumed that the relative errors in the functions are of the order of the machine precision. If IOPT=2 or 3, then EPSFCN can be ignored (treat it as a dummy argument). |
| 14 | `DIAG` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an array of length N. If MODE = 1 (see below), DIAG is internally set. If MODE = 2, DIAG must contain positive entries that serve as implicit (multiplicative) scale factors for the variables. |
| 15 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer input variable. If MODE = 1, the variables will be scaled internally. If MODE = 2, the scaling is speci- fied by the input DIAG. Other values of MODE are equivalent to MODE = 1. |
| 16 | `FACTOR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is a positive input variable used in determining the ini- tial step bound. This bound is set to the product of FACTOR and the Euclidean norm of DIAG*X if nonzero, or else to FACTOR itself. In most cases FACTOR should lie in the interval (. 1,100. ). 100. |
| 17 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iterations thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example) and. |
| 18 | `INFO` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows INFO = 0 improper input parameters. INFO = 1 both actual and predicted relative reductions in the sum of squares are at most FTOL. INFO = 2 relative error between two consecutive iterates is at most XTOL. |
| 19 | `NFEV` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable set to the number of calls to FCN for function evaluation. |
| 20 | `NJEV` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable set to the number of evaluations of the full Jacobian. If IOPT=2, only one call to. |
| 21 | `IPVT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | is an integer output array of length N. IPVT defines a permutation matrix P such that JAC*P = Q*R, where JAC is the final calculated Jacobian, Q is orthogonal (not stored), and R is upper triangular with diagonal elements of nonincreasing magnitude. Column J of P is column IPVT(J) of the identity matrix. |
| 22 | `QTF` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is an output array of length N which contains the first N elements of the vector (Q transpose)*FVEC. |
| 23 | `WA1` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | work arrays of length N. |
| 24 | `WA2` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | work arrays of length N. |
| 25 | `WA3` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | work arrays of length N. |
| 26 | `WA4` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is a work array of length M. 4. Successful Completion. The accuracy of DNLS1 is controlled by the convergence parame- ters FTOL, XTOL, and GTOL. These parameters are used in tests which make three types of comparisons between the approximation X and a solution XSOL. DNLS1 terminates when any of the tests is satisfied. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

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
| `INFO` | `1` | 1, then the accuracy of the components of X is usually related to their sensitivity. Unless high precision solutions are required, the recommended value for XTOL is the square root of the machine precision. Third Convergence Test. This test is satisfied when the cosine of the angle between FVEC and any column of the Jacobian at X is at most GTOL in absolute value. There is no clear rela- tionship between this test and the accuracy of DNLS1, and furthermore, the test is equally well satisfied at other crit- ical points, namely maximizers and saddle points. Therefore, |
| `INFO` | `4` | 4) should be examined carefully. The recommended value for GTOL is zero. 5. Unsuccessful Completion. Unsuccessful termination of DNLS1 can be due to improper input parameters, arithmetic interrupts, or an excessive number of function evaluations. Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1 |
| `INFO` | `1` | or 2 |

### Storage and workspace requirements

`WA1`: work arrays of length N.

`WA2`: work arrays of length N.

`WA3`: work arrays of length N.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::least_squares::dnls1`. Native symbol: `dnls1_`. Declaration feature: `least-squares-nonlinear-expert`. Provider feature: `least-squares-nonlinear-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::least_squares::dnls1`
- Public declaration feature: `least-squares-nonlinear-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::least_squares::least_squares_expert`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
