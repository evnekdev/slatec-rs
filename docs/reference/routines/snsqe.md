# SNSQE

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

An easy-to-use code to find a zero of a system of N nonlinear functions in N variables by a modification of the Powell hybrid method.

## Description

1. Purpose. The purpose of SNSQE is to find a zero of a system of N non- linear functions in N variables by a modification of the Powell hybrid method. This is done by using the more general nonlinear equation solver SNSQ. The user must provide a subroutine which calculates the functions. The user has the option of either to provide a subroutine which calculates the Jacobian or to let the code calculate it by a forward-difference approximation. This code is the combination of the MINPACK codes (Argonne) HYBRD1 and HYBRJ1. 2. Subroutine and Type Statements. SUBROUTINE SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO,

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SNSQE](https://www.netlib.org/slatec/src/snsqe.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FCN` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | is the name of the user-supplied subroutine which calculates the functions. FCN must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) Calculate the functions at X and return this vector in FVEC. RETURN END The value of IFLAG should not be changed by FCN unless the user wants to terminate execution of SNSQE. In this case, set IFLAG to a negative integer. |
| 2 | `JAC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | is the name of the user-supplied subroutine which calculates the Jacobian. If IOPT=1, then JAC must be declared in an EXTERNAL statement in the user calling program, and should be written as follows. SUBROUTINE JAC(N,X,FVEC,FJAC,LDFJAC,IFLAG) INTEGER N,LDFJAC,IFLAG REAL X(N),FVEC(N),FJAC(LDFJAC,N) Calculate the Jacobian at X and return this matrix in FJAC. FVEC contains the function values at X and should not be altered. RETURN END The value of IFLAG should not be changed by JAC unless the user wants to terminate execution of SNSQE. In this case, set IFLAG to a negative integer. |
| 3 | `IOPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an input variable which specifies how the Jacobian will be calculated. If IOPT=1, then the user must supply the Jacobian through the subroutine JAC. If IOPT=2, then the code will approximate the Jacobian by forward-differencing. GT. 2, or N. LE. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is a positive integer input variable set to the number of functions and variables. |
| 5 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of length N. On input, X must contain an initial estimate of the solution vector. On output, X contains the final estimate of the solution vector. -1. E0 10 CONTINUE LWA = 180 NPRINT = 0 C Set TOL to the square root of the machine precision. C Unless high precision solutions are required, C this is the recommended setting. |
| 6 | `FVEC` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an output array of length N which contains the functions evaluated at the output X. TEMP - TEMP1 - TWO*TEMP2 + ONE 10 CONTINUE RETURN END Results obtained with different compilers or machines may be slightly different. FINAL L2 NORM OF THE RESIDUALS 0. 1192636E-07 EXIT PARAMETER 1 FINAL APPROXIMATE SOLUTION -0. 5706545E+00 -0. 6816283E+00 -0. |
| 7 | `TOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | is a non-negative input variable. Termination occurs when the algorithm estimates that the relative error between X and the solution is at most TOL. Section 4 contains more details about TOL. SQRT(R1MACH(4)) CALL SNSQE(FCN,JAC,IOPT,N,X,FVEC,TOL,NPRINT,INFO,WA,LWA) FNORM = ENORM(N,FVEC) WRITE (NWRITE,1000) FNORM,INFO,(X(J),J=1,N) STOP 1000 FORMAT (5X,' FINAL L2 NORM OF THE RESIDUALS',E15. 7 // 5X,' EXIT PARAMETER',16X,I10 // 5X,' FINAL APPROXIMATE SOLUTION' // (5X,3E15. 7)) END SUBROUTINE FCN(N,X,FVEC,IFLAG) INTEGER N,IFLAG REAL X(N),FVEC(N) INTEGER K REAL ONE,TEMP,TEMP1,TEMP2,THREE,TWO,ZERO DATA ZERO,ONE,TWO,THREE /0. |
| 8 | `NPRINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer input variable that enables controlled printing of iterates if it is positive. In this case, FCN is called with IFLAG = 0 at the beginning of the first iteration and every NPRINT iteration thereafter and immediately prior to return, with X and FVEC available for printing. Appropriate print statements must be added to FCN (see example). If NPRINT is not positive, no special calls of FCN with IFLAG = 0 are made. |
| 9 | `INFO` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an integer output variable. If the user has terminated execution, INFO is set to the (negative) value of IFLAG. See description of FCN and JAC. Otherwise, INFO is set as follows. INFO = 0 improper input parameters. INFO = 1 algorithm estimates that the relative error between X and the solution is at most TOL. |
| 10 | `WA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (LWA) | INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC is a work array of length LWA. |
| 11 | `LWA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER IOPT,N,NPRINT,INFO,LWA REAL TOL REAL X(N),FVEC(N),WA(LWA) EXTERNAL FCN,JAC is a positive integer input variable not less than (3*N**2+13*N))/2. 4. Successful Completion. The accuracy of SNSQE is controlled by the convergence parame- ter TOL. This parameter is used in a test which makes a compar- ison between the approximation X and a solution XSOL. SNSQE terminates when the test is satisfied. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 improper input parameters. |
| `INFO` | `1` | 1 algorithm estimates that the relative error between X and the solution is at most TOL. |
| `INFO` | `2` | 2 number of calls to FCN has reached or exceeded |
| `INFO` | `1` | or 200*(N+1) for IOPT=2. |
| `INFO` | `3` | 3 TOL is too small. No further improvement in the approximate solution X is possible. |
| `INFO` | `4` | 4 iteration is not making good progress. Sections 4 and 5 contain more details about INFO. nents of X may have large relative errors, but the fast rate of convergence of SNSQE usually avoids this possibility. 5. Unsuccessful Completion. Unsuccessful termination of SNSQE can be due to improper input parameters, arithmetic interrupts, an excessive number of func- tion evaluations, errors in the functions, or lack of good prog- Improper Input Parameters. INFO is set to 0 if IOPT .LT. 1, or |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

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
