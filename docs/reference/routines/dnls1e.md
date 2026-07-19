# DNLS1E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

An easy-to-use code which minimizes the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm.

## Description

1. Purpose. The purpose of DNLS1E is to minimize the sum of the squares of M nonlinear functions in N variables by a modification of the Levenberg-Marquardt algorithm. This is done by using the more general least-squares solver DNLS1. The user must provide a subroutine which calculates the functions. The user has the option of how the Jacobian will be supplied. The user can supply the full Jacobian, or the rows of the Jacobian (to avoid storing the full Jacobian), or let the code approximate the Jacobian by forward-differencing. This code is the combination of the MINPACK codes (Argonne) LMDER1, LMDIF1, and LMSTR1. 2. Subroutine and Type Statements. SUBROUTINE DNLS1E(FCN,IOPT,M,N,X,FVEC,TOL,NPRINT, * INFO,IW,WA,LWA) INTEGER IOPT,M,N,NPRINT,INFO,LWAC,IW(N) DOUBLE PRECISION TOL,X(N),FVEC(M),WA(LWA) EXTERNAL FCN 3. Parameters. ALL TYPE REAL parameters are DOUBLE PRECISION Parameters designated as input parameters must be specified on entry to DNLS1E and are not changed on exit, while parameters designated as output parameters need not be specified on entry and are set to appropriate values on exit from DNLS1E. FCN is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. If the user wants the iterates printed (NPRINT positive), then FCN must do the printing. See the explanation of NPRINT below. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N DOUBLE PRECISION X(N),FVEC(M) FJAC and LDFJAC may be ignored , if IOPT=1. DOUBLE PRECISION FJAC(LDFJAC,N) , if IOPT=2. DOUBLE PRECISION FJAC(N) , if IOPT=3. If IFLAG=0, the values in X and FVEC are available for printing. See the explanation of NPRINT below. IFLAG will never be zero unless NPRINT is positive. The values of X and FVEC must not be changed. RETURN If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC. Note that IFLAG will never be 2 unless IOPT=2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FVEC contains the function values at X and must not be altered. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN

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
- Safe Rust paths: `slatec::least_squares::least_squares`

## Providers

- Canonical provider: `main-src/src/dnls1e.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dnls1e.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dnls1e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dnls1e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
