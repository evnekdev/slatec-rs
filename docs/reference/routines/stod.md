# STOD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DEBDF

## Description

STOD integrates a system of first order odes over one step in the integrator package DEBDF. STOD performs one step of the integration of an initial value problem for a system of ordinary differential equations. Note.. STOD is independent of the value of the iteration method indicator MITER, when this is .NE. 0, and hence is independent of the type of chord method used, or the Jacobian structure. Communication with STOD is done with the following variables.. Y = An array of length .GE. n used as the Y argument in all calls to F and JAC. NEQ = Integer array containing problem size in NEQ(1), and passed as the NEQ argument in all calls to F and JAC. YH = An NYH by LMAX array containing the dependent variables and their approximate scaled derivatives, where LMAX = MAXORD + 1. YH(I,J+1) contains the approximate J-th derivative of Y(I), scaled by H**J/Factorial(j) (J = 0,1,...,NQ). On entry for the first step, the first two columns of YH must be set from the initial values. NYH = A constant integer .GE. N, the first dimension of YH. YH1 = A one-dimensional array occupying the same space as YH. EWT = An array of N elements with which the estimated local errors in YH are compared. SAVF = An array of working storage, of length N. ACOR = A work array of length N, used for the accumulated corrections. On a successful return, ACOR(I) contains the estimated one-step local error in Y(I). WM,IWM = Real and integer work arrays associated with matrix operations in chord iteration (MITER .NE. 0). PJAC = Name of routine to evaluate and preprocess Jacobian matrix if a chord method is being used. SLVS = Name of routine to solve linear system in chord iteration. H = The step size to be attempted on the next step. H is altered by the error control algorithm during the problem. H can be either positive or negative, but its sign must remain constant throughout the problem. HMIN = The minimum absolute value of the step size H to be used. HMXI = Inverse of the maximum absolute value of H to be used. HMXI = 0.0 is allowed and corresponds to an infinite HMAX. HMIN and HMXI may be changed at any time, but will not take effect until the next change of H is considered. TN = The independent variable. TN is updated on each step taken. JSTART = An integer used for input only, with the following values and meanings.. 0 Perform the first step. .GT.0 Take a new step continuing from the last. -1 Take the next step with a new value of H, MAXORD, N, METH, MITER, and/or matrix parameters. -2 Take the next step with a new value of H, but with other inputs unchanged. On return, JSTART is set to 1 to facilitate continuation. KFLAG = a completion code with the following meanings.. 0 The step was successful. -1 The requested error could not be achieved. -2 Corrector convergence could not be achieved. A return with KFLAG = -1 or -2 means either ABS(H) = HMIN or 10 consecutive failures occurred. On a return with KFLAG negative, the values of TN and the YH array are as of the beginning of the last step, and H is the last step size attempted. MAXORD = The maximum order of integration method to be allowed. METH/MITER = The method flags. See description in driver. N = The number of first-order differential equations.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DEBDF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/stod.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/stod.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/stod.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/stod.f) — `verified_cached`
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
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
