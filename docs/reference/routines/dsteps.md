# DSTEPS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Integrate a system of first order ordinary differential equations one step.

## Description

Written by L. F. Shampine and M. K. Gordon Abstract Subroutine DSTEPS is normally used indirectly through subroutine DDEABM . Because DDEABM suffices for most problems and is much easier to use, using it should be considered before using DSTEPS alone. Subroutine DSTEPS integrates a system of NEQN first order ordinary differential equations one step, normally from X to X+H, using a modified divided difference form of the Adams Pece formulas. Local extrapolation is used to improve absolute stability and accuracy. The code adjusts its order and step size to control the local error per unit step in a generalized sense. Special devices are included to control roundoff error and to detect when the user is requesting too much accuracy. This code is completely explained and documented in the text, Computer Solution of Ordinary Differential Equations, The Initial Value Problem by L. F. Shampine and M. K. Gordon. Further details on use of this code are available in "Solving Ordinary Differential Equations with ODE, STEP, and INTRP", by L. F. Shampine and M. K. Gordon, SLA-73-1060. The parameters represent -DF -- subroutine to evaluate derivatives NEQN -- number of equations to be integrated Y(*) -- solution vector at X X -- independent variable H -- appropriate step size for next step. Normally determined by code EPS -- local error tolerance WT(*) -- vector of weights for error criterion START -- logical variable set .TRUE. for first step, .FALSE. otherwise HOLD -- step size used for last successful step K -- appropriate order for next step (determined by code) KOLD -- order used for last successful step CRASH -- logical variable set .TRUE. when no step can be taken, .FALSE. otherwise. YP(*) -- derivative of solution vector at X after successful step KSTEPS -- counter on attempted steps TWOU -- 2.*U where U is machine unit roundoff quantity FOURU -- 4.*U where U is machine unit roundoff quantity RPAR,IPAR -- parameter arrays which you may choose to use for communication between your program and subroutine F. They are not altered or used by DSTEPS. The variables X,XOLD,KOLD,KGI and IVC and the arrays Y,PHI,ALPHA,G, W,P,IV and GI are required for the interpolation subroutine SINTRP. The remaining variables and arrays are included in the call list only to eliminate local retention of variables between calls. Input to DSTEPS First call -- The user must provide storage in his calling program for all arrays in the call list, namely DIMENSION Y(NEQN),WT(NEQN),PHI(NEQN,16),P(NEQN),YP(NEQN),PSI(12), 1 ALPHA(12),BETA(12),SIG(13),V(12),W(12),G(13),GI(11),IV(10), 2 RPAR(*),IPAR(*) **Note** The user must also declare START , CRASH , PHASE1 and NORND logical variables and DF an EXTERNAL subroutine, supply the subroutine DF(X,Y,YP) to evaluate DY(I)/DX = YP(I) = DF(X,Y(1),Y(2),...,Y(NEQN)) and initialize only the following parameters. NEQN -- number of equations to be integrated Y(*) -- vector of initial values of dependent variables X -- initial value of the independent variable H -- nominal step size indicating direction of integration and maximum size of step. Must be variable EPS -- local error tolerance per step. Must be variable WT(*) -- vector of non-zero weights for error criterion START -- .TRUE. YP(*) -- vector of initial derivative values KSTEPS -- set KSTEPS to zero TWOU -- 2.*U where U is machine unit roundoff quantity FOURU -- 4.*U where U is machine unit roundoff quantity Define U to be the machine unit roundoff quantity by calling the function routine D1MACH, U = D1MACH(4), or by computing U so that U is the smallest positive number such that 1.0+U .GT. 1.0. DSTEPS requires that the L2 norm of the vector with components LOCAL ERROR(L)/WT(L) be less than EPS for a successful step. The array WT allows the user to specify an error test appropriate for his problem. For example, WT(L) = 1.0 specifies absolute error, = ABS(Y(L)) error relative to the most recent value of the L-th component of the solution, = ABS(YP(L)) error relative to the most recent value of the L-th component of the derivative, = MAX(WT(L),ABS(Y(L))) error relative to the largest magnitude of L-th component obtained so far, = ABS(Y(L))*RELERR/EPS + ABSERR/EPS specifies a mixed relative-absolute test where RELERR is relative error, ABSERR is absolute error and EPS = MAX(RELERR,ABSERR) . Subsequent calls -- Subroutine DSTEPS is designed so that all information needed to continue the integration, including the step size H and the order K , is returned with each step. With the exception of the step size, the error tolerance, and the weights, none of the parameters should be altered. The array WT must be updated after each step to maintain relative error tests like those above. Normally the integration is continued just beyond the desired endpoint and the solution interpolated there with subroutine SINTRP . If it is impossible to integrate beyond the endpoint, the step size may be reduced to hit the endpoint since the code will not take a step larger than the H input. Changing the direction of integration, i.e., the sign of H , requires the user set START = .TRUE. before calling DSTEPS again. This is the only situation in which START should be altered. Output from DSTEPS Successful Step -- The subroutine returns after each successful step with START and CRASH set .FALSE. . X represents the independent variable advanced one step of length HOLD from its value on input and Y the solution vector at the new value of X . All other parameters represent information corresponding to the new X needed to continue the integration. Unsuccessful Step -- When the error tolerance is too small for the machine precision, the subroutine returns without taking a step and CRASH = .TRUE. . An appropriate step size and error tolerance for continuing are estimated and all other information is restored as upon input before returning. To continue with the larger tolerance, the user just calls the code again. A restart is neither required nor desirable.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A1B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dsteps.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dsteps.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dsteps.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dsteps.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
