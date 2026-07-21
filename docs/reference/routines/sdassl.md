# SDASSL

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

This code solves a system of differential/algebraic equations of the form G(T,Y,YPRIME) = 0.

## Description

Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) Subroutine SDASSL uses the backward differentiation formulas of orders one through five to solve a system of the above form for Y and

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A2`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::dassl::DaeSession::<f32, F>::advance_to`

## Providers

- Canonical provider: `main-src/src/sdassl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdassl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdassl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdassl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SDASSL](https://www.netlib.org/slatec/src/sdassl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `RES` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | This is a subroutine which you provide to define the differential/algebraic system. Provide a subroutine of the form SUBROUTINE RES(T,Y,YPRIME,DELTA,IRES,RPAR,IPAR) to define the system of differential/algebraic equations which is to be solved. For the given values of T,Y and YPRIME, the subroutine should return the residual of the differential/algebraic DELTA = G(T,Y,YPRIME) (DELTA(*) is a vector of length NEQ which is output for RES. ) Subroutine RES must not alter T,Y or YPRIME. You must declare the name RES in an external statement in your program that calls SDASSL. You must dimension Y,YPRIME and DELTA in RES. |
| 2 | `NEQ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This is the number of equations to be solved. Set it to the number of differential equations. (NEQ. GE. 1). |
| 3 | `T` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | This is the current value of the independent variable. Set it to the initial point of the integration. must be defined as a variable. |
| 4 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | This array contains the solution components at T. Set this vector to the initial values of the NEQ solution components at the initial point. You must dimension Y of length at least NEQ in your calling program. |
| 5 | `YPRIME` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | This array contains the derivatives of the solution components at T. Values for Y and YPRIME at the initial time must be given as input. These values must be consistent, (that is, if T,Y,YPRIME are the given initial values, they must satisfy G(T,Y,YPRIME) = 0. ). The subroutine solves the system from T to TOUT. It is easy to continue the solution to get results at additional TOUT. |
| 6 | `TOUT` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | This is a point at which a solution is desired. Set it to the first point at which a solution is desired. You can not take TOUT = T. integration either forward in T (TOUT. GT. T) or backward in T (TOUT. |
| 7 | `INFO` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (15) | The basic task of the code is to solve the system from T to TOUT and return an answer at TOUT. INFO is an integer array which is used to communicate exactly how you want this task to be carried out. (See below for details. ) N must be greater than or equal to 15. Use the INFO array to give the code more details about how you want your problem solved. This array should be dimensioned of length 15, though SDASSL uses only the first eleven entries. |
| 8 | `RTOL` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. Caution: In Fortran 77, a scalar is not the same as an array of length 1. Some compilers may object to using scalars for RTOL,ATOL. You must assign relative (RTOL) and absolute (ATOL error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as variables because the code may change them. |
| 9 | `ATOL` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. Caution: In Fortran 77, a scalar is not the same as an array of length 1. Some compilers may object to using scalars for RTOL,ATOL. You must assign relative (RTOL) and absolute (ATOL error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as variables because the code may change them. |
| 10 | `IDID` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This scalar quantity is an indicator reporting what the code did. You must monitor this integer variable to decide what action to take next. |
| 11 | `RWORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A real work array of length LRW which provides the code with needed storage space. HMAX **** HO **** Dimension this real work array of length LRW in your calling program. |
| 12 | `LRW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The length of RWORK. (See below for required length. ) Set it to the declared length of the RWORK array. You must have. GE. 40+(MAXORD+4)*NEQ+NEQ**2 for the full (dense) JACOBIAN case (when INFO(6)=0), or. |
| 13 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | An integer work array of length LIW which provides the code with needed storage space. ML MU **** MAXORD **** Dimension this integer work array of length LIW in your calling program. |
| 14 | `LIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The length of IWORK. (See below for required length. ) Set it to the declared length of the IWORK array. You must have LIW. GE. 20+NEQ. |
| 15 | `RPAR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | These are real and integer parameter arrays which you can use for communication between your calling program and the RES subroutine (and the JAC subroutine) and subroutine RES. They are not altered by SDASSL. If you do not need RPAR or IPAR, ignore these parameters by treat- ing them as dummy arguments. If you do choose to use them, dimension them in your calling program and in RES as arrays of appropriate length. These are parameter arrays, of real and integer type, respectively. You can use them for communication between your program that calls SDASSL and the RES subroutine (and the JAC subroutine). |
| 16 | `IPAR` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These are real and integer parameter arrays which you can use for communication between your calling program and the RES subroutine (and the JAC subroutine) and subroutine RES. They are not altered by SDASSL. If you do not need RPAR or IPAR, ignore these parameters by treat- ing them as dummy arguments. If you do choose to use them, dimension them in your calling program and in RES as arrays of appropriate length. These are parameter arrays, of real and integer type, respectively. You can use them for communication between your program that calls SDASSL and the RES subroutine (and the JAC subroutine). |
| 17 | `JAC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | This is the name of a subroutine which you may choose to provide for defining a matrix of partial derivatives described below. If you have set INFO(5)=0, you can ignore this parameter by treating it as a dummy argument. Otherwise, you must provide a subroutine of the form SUBROUTINE JAC(T,Y,YPRIME,PD,CJ,RPAR,IPAR) to define the matrix of partial derivatives PD=DG/DY+CJ*DG/DYPRIME CJ is a scalar which is input to JAC. For the given values of T,Y,YPRIME, the subroutine must evaluate the non-zero partial derivatives for each equation and each solution component, and store these values in the matrix PD. The elements of PD are set to zero before each call to JAC so only non-zero elements need to be defined. Subroutine JAC must not alter T,Y,(*),YPRIME(*), or CJ. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 No - Not applicable here. See below for continuation calls. **** is specified by the error tolerances RTOL and ATOL. The simplest use is to take them both to be scalars. To obtain more flexibility, they can both be vectors. The code must be told your choice. Are both error tolerances RTOL, ATOL scalars ... and input scalars for both RTOL and ATOL |
| `INFO` | `1` | 1 and input arrays for both RTOL and ATOL **** of TOUT by steps. If you wish, it will return the computed solution and derivative at the next intermediate step (the intermediate-output mode) or TOUT, whichever comes first. This is a good way to proceed if you want to see the behavior of the solution. If you must have solutions at a great many specific TOUT points, this code will compute them efficiently. Do you want the solution only at TOUT (and not at the next intermediate step) ... |
| `INFO` | `0` | 0 |
| `INFO` | `1` | 1 **** values TOUT efficiently, this code may integrate past TOUT and interpolate to obtain the result at TOUT. Sometimes it is not possible to integrate beyond some point TSTOP because the equation changes there or it is not defined past TSTOP. Then you must tell the code not to go past. Can the integration be carried out without any restrictions on the independent variable T ... |
| `INFO` | `1` | and define the stopping point TSTOP by setting RWORK(1)=TSTOP **** necessary to use a matrix of partial derivatives of the system of differential equations. If you do not provide a subroutine to evaluate it analytically (see description of the item JAC in the call list), it will be approximated by numerical differencing in this code. although it is less trouble for you to have the code compute partial derivatives by numerical differencing, the solution will be more reliable if you provide the derivatives via JAC. Sometimes numerical differencing is cheaper than evaluating derivatives in JAC and sometimes it is not - this depends on your problem. Do you want the code to evaluate the partial derivatives automatically by numerical differences ... |
| `INFO` | `1` | and provide subroutine JAC for evaluating the matrix of partial derivatives **** partial derivatives, DG/DY + CJ*DG/DYPRIME, (here CJ is a scalar determined by SDASSL) is banded and the code is told this. In this case, the storage needed will be greatly reduced, numerical differencing will be performed much cheaper, and a number of important algorithms will execute much faster. The differential equation is said to have half-bandwidths ML (lower) and MU (upper) if equation i involves only unknowns Y(J) with I-ML .LE. J .LE. I+MU ,2,...,NEQ. Thus, ML and MU are the widths of the lower and upper parts of the band, respectively, with the main diagonal being excluded. If you do not indicate that the equation has a banded matrix of partial derivatives, the code works with a full matrix of NEQ**2 elements (stored in the conventional way). Computations with banded matrices cost less time and storage than with full matrices if 2*ML+MU .LT. NEQ. If you tell the code that the matrix of partial derivatives has a banded structure and you want to provide subroutine JAC to compute the partial derivatives, then you must be careful to store the elements of the matrix in the special form indicated in the description of JAC. Do you want to solve the problem using a full (dense) matrix (and not a special banded structure) ... |
| `INFO` | `1` | and provide the lower (ML) and upper (MU) bandwidths by setting stepsize, so that the code will avoid passing over very large regions. Do you want the code to decide on its own maximum stepsize? |
| `INFO` | `1` | and define HMAX by setting may occasionally suffer from severe scaling difficulties on the first step. If you know a great deal about the scaling of your problem, you can help to alleviate this problem by specifying an initial stepsize HO. Do you want the code to define its own initial stepsize? |
| `INFO` | `1` | and define HO by setting you can save some locations by restricting the maximum order MAXORD. the default value is 5. for each order decrease below 5, the code requires NEQ fewer locations, however it is likely to be slower. In any case, you must have 1 .LE. MAXORD .LE. 5 Do you want the maximum order to default to 5? |
| `INFO` | `1` | and define MAXORD by setting will always be nonnegative, it may help to set this parameter. However, it is probably best to try the code without using this option first, and only to use this option if that doesn't work very well. Do you want the code to solve the problem without invoking any special nonnegativity constraints? |
| `INFO` | `1` | Y, and YPRIME to be consistent. That is, |
| `INFO` | `0` | 0 at the initial time. If you do not know the initial derivative precisely, you can let SDASSL try to compute it. Are the initial T, Y, YPRIME consistent? |
| `INFO` | `1` | 1, and set YPRIME to an initial approximation to YPRIME. (If you have no idea what |
| `INFO` | `0` | -- Full (dense) matrix *** Give PD a first dimension of NEQ. When you evaluate the (non-zero) partial derivative of equation I with respect to variable J, you must store it in PD according to PD(I,J) = "DG(I)/DY(J)+CJ*DG(I)/DYPRIME(J)" |
| `INFO` | `1` | -- Banded JACOBIAN with ML lower and MU upper diagonal bands (refer to INFO(6) description of ML and MU) *** Give PD a first dimension of 2*ML+MU+1. when you evaluate the (non-zero) partial derivative of equation I with respect to variable J, you must store it in PD according to IROW = I - J + ML + MU + 1 PD(IROW,J) = "DG(I)/DY(J)+CJ*DG(I)/DYPRIME(J)" |

### Storage and workspace requirements

`RWORK`: A real work array of length LRW which provides the code with needed storage space. HMAX **** HO **** Dimension this real work array of length LRW in your calling program.

`IWORK`: An integer work array of length LIW which provides the code with needed storage space. ML MU **** MAXORD **** Dimension this integer work array of length LIW in your calling program.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::dassl::sdassl`. Native symbol: `sdassl_`. Declaration feature: `dassl`. Provider feature: `dassl`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::dassl::sdassl`
- Public declaration feature: `dassl`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::dassl::DaeSession::<f32, F>::advance_to`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
