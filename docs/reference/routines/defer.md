# DEFER

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SEPELI

## Description

This subroutine first approximates the truncation error given by TRUN1(X,Y)=DLX**2*TX+DLY**2*TY where TX=AFUN(X)*UXXXX/12.0+BFUN(X)*UXXX/6.0 on the interior and at the boundaries if periodic (here UXXX,UXXXX are the third and fourth partial derivatives of U with respect to X). TX is of the form AFUN(X)/3.0*(UXXXX/4.0+UXXX/DLX) at X=A or X=B if the boundary condition there is mixed. TX=0.0 along specified boundaries. TY has symmetric form in Y with X,AFUN(X),BFUN(X) replaced by Y,DFUN(Y),EFUN(Y). The second order solution in USOL is used to approximate (via second order finite differencing) the truncation error and the result is added to the right hand side in GRHS and then transferred to USOL to be used as a new right hand side when calling BLKTRI for a fourth order solution.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SEPELI`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/defer.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/defer.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/defer.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/defer.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
