# SDRIV3

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The function of SDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. Other important options are available. SDRIV3 uses single precision arithmetic.

## Description

I. ABSTRACT ....................................................... The primary function of SDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. In addition, SDRIV3 may be used to solve: 1. The initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a non-singular matrix depending on Y and T. 2. The hybrid differential/algebraic initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a vector (whose values may depend upon Y and T) some of whose components will be zero corresponding to those equations which are algebraic rather than differential. SDRIV3 is to be called once for each output point of T. II. PARAMETERS .................................................... The user should use parameter names in the call sequence of SDRIV3 for those quantities whose value may be altered by SDRIV3. The parameters in the call sequence are: N = (Input) The number of dependent functions whose solution is desired. N must not be altered during a problem. T = The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routines F, JACOBN, FA, USERS, and G. Thus parameters required by those routines can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i.e., after setting NSTATE to 1 .) F = A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) REAL Y(*), YDOT(*) . . YDOT(1) = ... . . YDOT(N) = ...

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

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::ode::sdriv3`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `slatec::ode::OdeSession::<f32, F, E>::integrate_to`
- Exclusion or deferment reason: `pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes`
<!-- raw-api-status:end -->
