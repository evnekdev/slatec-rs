# DNLS1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm.

## Description

1. Purpose. The purpose of DNLS1 is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. The user must provide a subroutine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER, LMDIF, and LMSTR. 2. Subroutine and Type Statements. SUBROUTINE DNLS1(FCN,IOPT,M,N,X,FVEC,FJAC,LDFJAC,FTOL,XTOL, * GTOL,MAXFEV,EPSFCN,DIAG,MODE,FACTOR,NPRINT,INFO * ,NFEV,NJEV,IPVT,QTF,WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDFJAC,MAXFEV,MODE,NPRINT,INFO,NFEV,NJEV INTEGER IPVT(N) DOUBLE PRECISION FTOL,XTOL,GTOL,EPSFCN,FACTOR DOUBLE PRECISION X(N),FVEC(M),FJAC(LDFJAC,N),DIAG(N),QTF(N), * WA1(N),WA2(N),WA3(N),WA4(M) 3. Parameters. Parameters designated as input parameters must be specified on entry to DNLS1 and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1. FCN is the name of the user-supplied subroutine which calculate the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed (NPRINT positive), then FCN must do the printing. See the explanation of NPRINT below. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N DOUBLE PRECISION X(N),FVEC(M) FJAC and LDFJAC may be ignored , if IOPT=1. DOUBLE PRECISION FJAC(LDFJAC,N) , if IOPT=2. DOUBLE PRECISION FJAC(N) , if IOPT=3. If IFLAG=0, the values in X and FVEC are available for printing. See the explanation of NPRINT below. IFLAG will never be zero unless NPRINT is positive. The values of X and FVEC must not be changed. RETURN If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC. Note that IFLAG will never be 2 unless IOPT=2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FVEC contains the function values at X and must not be altered. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN

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
- GAMS classifications: `K1B1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::least_squares::least_squares_expert, slatec::least_squares::least_squares_with_jacobian`

## Providers

- Canonical provider: `main-src/src/dnls1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnls1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnls1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnls1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `batch_d_public_driver`
- Canonical Rust path: `slatec_sys::least_squares::dnls1`
- Current legacy Rust paths: `none`
- Public declaration feature: `least-squares-nonlinear-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::least_squares::least_squares_expert`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
