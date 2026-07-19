# CDRIV3

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The function of CDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. Other important options are available. CDRIV3 allows complex-valued differential equations.

## Description

I. ABSTRACT ....................................................... The primary function of CDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. In addition, CDRIV3 may be used to solve: 1. The initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a non-singular matrix depending on Y and T. 2. The hybrid differential/algebraic initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a vector (whose values may depend upon Y and T) some of whose components will be zero corresponding to those equations which are algebraic rather than differential. CDRIV3 is to be called once for each output point of T. II. PARAMETERS .................................................... The user should use parameter names in the call sequence of CDRIV3 for those quantities whose value may be altered by CDRIV3. The parameters in the call sequence are: N = (Input) The number of dependent functions whose solution is desired. N must not be altered during a problem. T = (Real) The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = (Complex) The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided

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

- Canonical provider: `main-src/src/cdriv3.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cdriv3.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cdriv3.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cdriv3.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
