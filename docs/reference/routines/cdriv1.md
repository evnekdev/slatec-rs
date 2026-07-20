# CDRIV1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The function of CDRIV1 is to solve N (200 or fewer) ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. CDRIV1 allows complex-valued differential equations.

## Description

Version 92.1 I. CHOOSING THE CORRECT ROUTINE ................................... SDRIV DDRIV CDRIV These are the generic names for three packages for solving initial value problems for ordinary differential equations. SDRIV uses single precision arithmetic. DDRIV uses double precision arithmetic. CDRIV allows complex-valued differential equations, integrated with respect to a single, real, independent variable. As an aid in selecting the proper program, the following is a discussion of the important options or restrictions associated with each program: A. CDRIV1 should be tried first for those routine problems with no more than 200 differential equations (CDRIV2 and CDRIV3 have no such restriction.) Internally this routine has two important technical defaults: 1. Numerical approximation of the Jacobian matrix of the right hand side is used. 2. The stiff solver option is used. Most users of CDRIV1 should not have to concern themselves with these details. B. CDRIV2 should be considered for those problems for which CDRIV1 is inadequate. For example, CDRIV1 may have difficulty with problems having zero initial conditions and zero derivatives. In this case CDRIV2, with an appropriate value of the parameter EWT, should perform more efficiently. CDRIV2 provides three important additional options: 1. The nonstiff equation solver (as well as the stiff solver) is available. 2. The root-finding option is available. 3. The program can dynamically select either the non-stiff or the stiff methods. Internally this routine also defaults to the numerical approximation of the Jacobian matrix of the right hand side. C. CDRIV3 is the most flexible, and hence the most complex, of the programs. Its important additional features include: 1. The ability to exploit band structure in the Jacobian matrix. 2. The ability to solve some implicit differential equations, i.e., those having the form: A(Y,T)*dY/dT = F(Y,T). 3. The option of integrating in the one step mode. 4. The option of allowing the user to provide a routine which computes the analytic Jacobian matrix of the right hand side. 5. The option of allowing the user to provide a routine which does all the matrix algebra associated with corrections to the solution components. II. PARAMETERS .................................................... The user should use parameter names in the call sequence of CDRIV1 for those quantities whose value may be altered by CDRIV1. The parameters in the call sequence are: N = (Input) The number of differential equations, N .LE. 200 T = (Real) The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = (Complex) The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routine F. Thus parameters required by F can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i.e., after setting MSTATE to +1(-1).) F = A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) COMPLEX Y(*), YDOT(*) . . YDOT(1) = ... . . YDOT(N) = ...

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A2`
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

- Canonical provider: `main-src/src/cdriv1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cdriv1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cdriv1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cdriv1.f) — `verified_cached`
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
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
