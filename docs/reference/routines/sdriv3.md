# SDRIV3

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The function of SDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. Other important options are available. SDRIV3 uses single precision arithmetic.

## Description

I. ABSTRACT ....................................................... The primary function of SDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. In addition, SDRIV3 may be used to solve: 1. The initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a non-singular matrix depending on Y and T. 2. The hybrid differential/algebraic initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a vector (whose values may depend upon Y and T) some of whose components will be zero corresponding to those equations which are algebraic rather than differential. SDRIV3 is to be called once for each output point of T.

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
- Safe Rust paths: `slatec::ode::OdeSession::<f32, F, E>::integrate_to`

## Providers

- Canonical provider: `main-src/src/sdriv3.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdriv3.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdriv3.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdriv3.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SDRIV3](https://www.netlib.org/slatec/src/sdriv3.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The number of dependent functions whose solution is desired. N must not be altered during a problem. |
| 2 | `T` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. |
| 3 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routines F, JACOBN, FA, USERS, and G. Thus parameters required by those routines can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i. |
| 4 | `F` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) REAL Y(*), YDOT(*) YDOT(1) =. YDOT(N) =. END (Sample) This computes YDOT = F(Y,T), the right hand side of the differential equations. Here Y is a vector of length at least N. |
| 5 | `NSTATE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An integer describing the status of integration. The meaning of NSTATE is as follows: 1 (Input) Means the first call to the routine. This value must be set by the user. On all subsequent calls the value of NSTATE should be tested by the user, but must not be altered. (As a convenience to the user who may wish to put out the initial conditions, SDRIV3 can be called with NSTATE=1, and. |
| 6 | `TOUT` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | T. In this case the program will return with NSTATE unchanged, i. e. , NSTATE=1. ) 2 (Output) Means a successful integration. If a normal continuation is desired (i. |
| 7 | `NTASK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) An index specifying the manner of returning the solution, according to the following: 1 Means SDRIV3 will integrate past TOUT and interpolate the solution. This is the most efficient mode. 2 Means SDRIV3 will return the solution after each internal integration step, or at TOUT, whichever comes first. In the latter case, the program integrates exactly to TOUT. 3 Means SDRIV3 will adjust its internal step to reach TOUT exactly (useful if a singularity exists beyond TOUT. ). |
| 8 | `NROOT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The number of equations whose roots are desired. If NROOT is zero, the root search is not active. This option is useful for obtaining output at points which are not known in advance, but depend upon the solution, e. g. , when some solution component takes on a specified value. The root search is carried out using the user-written function G (see description of G below. |
| 9 | `EPS` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | On input, the requested relative accuracy in all solution components. EPS = 0 is allowed. On output, the adjusted relative accuracy if the input value was too small. The value of EPS should be set as large as is reasonable, because the amount of work done by SDRIV3 increases as EPS decreases. |
| 10 | `EWT` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (Input) Problem zero, i. e. , the smallest, nonzero, physically meaningful value for the solution. (Array, possibly of length one. See following description of is ignored. 2 Means YWT(I) = ABS(Y(I)), (Relative error control) 3 Means YWT(I) = MAX(ABS(Y(I)), EWT(1)). |
| 11 | `IERROR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Setting EWT smaller than necessary can adversely affect the running time. (Input) Error control indicator. A value of 3 is suggested for most problems. Other choices and detailed explanations of EWT and IERROR are given below for those who may need extra flexibility. These last three input quantities EPS, EWT and IERROR control the accuracy of the computed solution. EWT and are used internally to compute an array YWT. |
| 12 | `MINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The integration method indicator. 1 Means the Adams methods, and is used for non-stiff problems. 2 Means the stiff methods of Gear (i. e. , the backward differentiation formulas), and is used for stiff problems. 3 Means the program dynamically selects the Adams methods when the problem is non-stiff and the Gear methods when the problem is stiff. |
| 13 | `MITER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The iteration method indicator. 0 Means functional iteration. This value is suggested for non-stiff problems. 1 Means chord method with analytic Jacobian. In this case, the user supplies subroutine JACOBN (see description below). 2 Means chord method with Jacobian calculated internally by finite differences. |
| 14 | `IMPL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The implicit method indicator. 0 Means solving dY(I)/dT = F(Y(I),T). 1 Means solving A*dY(I)/dT = F(Y(I),T), non- singular A (see description of FA below. ) Only MINT = 1 or 2, and MITER = 1, 2, 3, 4, or 5 are allowed for this option. 2,3 Means solving certain systems of hybrid differential/algebraic equations (see description of FA below. ) Only MINT = 2 and 0. |
| 15 | `ML` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The lower half-bandwidth in the case of a banded A or Jacobian matrix. (I. e. , maximum(R-C) for nonzero A(R,C). ). |
| 16 | `MU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The upper half-bandwidth in the case of a banded A or Jacobian matrix. (I. e. , maximum(C-R). ) The left hand side of the I-th equation is a linear combination of dY(I-ML)/dT, dY(I-ML+1)/dT,. , dY(I)/dT,. |
| 17 | `MXORD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The maximum order desired. This is. LE. 12 for the Adams methods and. 5 for the Gear methods. Normal value is 12 and 5, respectively. |
| 18 | `HMAX` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (Input) The maximum magnitude of the step size that will be used for the problem. This is useful for ensuring that important details are not missed. If this is not the case, a large value, such as the interval length, is suggested. |
| 19 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is an array of LENW real words used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as REAL WORK(. ) The following table gives the required minimum value for the length of WORK, depending on the value of IMPL and. |
| 20 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input). |
| 21 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | is an integer array of length LENIW used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as INTEGER IWORK(. ) The length of IWORK should be at least 50 if MITER is 0 or 3, or N+50 if MITER is 1, 2, 4, or 5, or MINT is 3, and LENIW should be set to the value used. The contents of IWORK should not be disturbed between calls to SDRIV3. |
| 22 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input). |
| 23 | `JACOBN` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | A subroutine supplied by the user, if MITER is 1 or 4. If this is the case, the name must be declared EXTERNAL in the user's calling program. Given a system of N differential equations, it is meaningful to speak about the partial derivative of the I-th right hand side with respect to the J-th dependent variable. In general there are N*N such quantities. Often however the equations can be ordered so that the I-th differential equation only involves dependent variables with index near I, e. g. |
| 24 | `FA` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | A subroutine supplied by the user if IMPL is not zero, and. |
| 25 | `NDE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The number of differential equations. This is required only for IMPL = 2 or 3, with NDE. LT. N. |
| 26 | `MXSTEP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Input) The maximum number of internal steps allowed on one call to SDRIV3. |
| 27 | `G` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | A real FORTRAN function supplied by the user if NROOT is not 0. In this case, the name must be declared EXTERNAL in the user's calling program. G is repeatedly called with different values of IROOT to obtain the value of each of the NROOT equations for which a root is desired. G is of the form: REAL FUNCTION G (N, T, Y, IROOT) REAL Y(*) GO TO (10,. ), IROOT 10 G =. END (Sample) Here, Y is a vector of length at least N, whose first N components are the solution components at the point T. |
| 28 | `USERS` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | A subroutine supplied by the user, if MITER is 3. If this is the case, the name must be declared EXTERNAL in the user's calling program. The routine USERS is called by SDRIV3 when certain linear systems must be solved. The user may choose any method to form, store and solve these systems in order to obtain the solution result that is returned to SDRIV3. In particular, this allows sparse matrix methods to be used. The call sequence for this routine is: SUBROUTINE USERS (Y, YH, YWT, SAVE1, SAVE2, T, H, EL, 8 IMPL, N, NDE, IFLAG) REAL Y(*), YH(*), YWT(*), SAVE1(*), 8 SAVE2(*), T, H, EL The input variable IFLAG indicates what action is to be taken. |
| 29 | `IERFLG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An error flag. The error number associated with a diagnostic message (see Section III-A below) is the same as the corresponding value of IERFLG. The meaning of 0 The routine completed successfully. (No message is issued. ) 3 (Warning) The number of steps required to reach TOUT exceeds MXSTEP. 4 (Warning) The value of EPS is too small. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `1` | 1. (Absolute error control) |

### Storage and workspace requirements

`WORK`: is an array of LENW real words used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as REAL WORK(...) The following table gives the required minimum value for the length of WORK, depending on the value of IMPL and

`IWORK`: is an integer array of length LENIW used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as INTEGER IWORK(...) The length of IWORK should be at least 50 if MITER is 0 or 3, or N+50 if MITER is 1, 2, 4, or 5, or MINT is 3, and LENIW should be set to the value used. The contents of IWORK should not be disturbed between calls to SDRIV3.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::sdriv3`. Native symbol: `sdriv3_`. Declaration feature: `ode-sdrive-expert`. Provider feature: `ode-sdrive-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::ode::sdriv3`
- Public declaration feature: `ode-sdrive-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::ode::OdeSession::<f32, F, E>::integrate_to`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
