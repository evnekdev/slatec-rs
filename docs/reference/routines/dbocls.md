# DBOCLS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve the bounded and constrained least squares problem consisting of solving the equation E*X = F (in the least squares sense) subject to the linear constraints C*X = Y.

## Description

**** All INPUT and OUTPUT real variables are DOUBLE PRECISION **** This subprogram solves the bounded and constrained least squares problem. The problem statement is: Solve E*X = F (least squares sense), subject to constraints C*X=Y. In this formulation both X and Y are unknowns, and both may have bounds on any of their components. This formulation of the problem allows the user to have equality and inequality constraints as well as simple bounds on the solution components. This constrained linear least squares subprogram solves E*X=F subject to C*X=Y, where E is MROWS by NCOLS, C is MCON by NCOLS. The user must have dimension statements of the form DIMENSION W(MDW,NCOLS+MCON+1), BL(NCOLS+MCON), BU(NCOLS+MCON), * X(2*(NCOLS+MCON)+2+NX), RW(6*NCOLS+5*MCON) INTEGER IND(NCOLS+MCON), IOPT(17+NI), IW(2*(NCOLS+MCON)) (here NX=number of extra locations required for the options; NX=0 if no options are in use. Also NI=number of extra locations for options 1-9.)

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
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares`

## Providers

- Canonical provider: `main-src/src/dbocls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbocls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbocls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbocls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
