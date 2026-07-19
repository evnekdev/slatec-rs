# DBINTK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a spline which interpolates given data.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBINTK is the SPLINT routine of the reference. DBINTK produces the B-spline coefficients, BCOEF, of the B-spline of order K with knots T(I), I=1,...,N+K, which takes on the value Y(I) at X(I), I=1,...,N. The spline or any of its derivatives can be evaluated by calls to DBVALU. The I-th equation of the linear system A*BCOEF = B for the coefficients of the interpolant enforces interpolation at X(I), I=1,...,N. Hence, B(I) = Y(I), for all I, and A is a band matrix with 2K-1 bands if A is invertible. The matrix A is generated row by row and stored, diagonal by diagonal, in the rows of Q, with the main diagonal going into row K. The banded system is then solved by a call to DBNFAC (which constructs the triangular factorization for A and stores it again in Q), followed by a call to DBNSLV (which then obtains the solution BCOEF by substitution). DBNFAC does no pivoting, since the total positivity of the matrix A makes this unnecessary. The linear system to be solved is (theoretically) invertible if and only if T(I) .LT. X(I) .LT. T(I+K), for all I. Equality is permitted on the left for I=1 and on the right for I=N when K knots are used at X(1) or X(N). Otherwise, violation of this condition is certain to lead to an error. Description of Arguments Input X,Y,T are double precision X - vector of length N containing data point abscissa in strictly increasing order. Y - corresponding vector of length N containing data point ordinates. T - knot vector of length N+K Since T(1),..,T(K) .LE. X(1) and T(N+1),..,T(N+K) .GE. X(N), this leaves only N-K knots (not necessarily X(I) values) interior to (X(1),X(N)) N - number of data points, N .GE. K K - order of the spline, K .GE. 1 Output BCOEF,Q,WORK are double precision BCOEF - a vector of length N containing the B-spline coefficients Q - a work vector of length (2*K-1)*N, containing the triangular factorization of the coefficient matrix of the linear system being solved. The coefficients for the interpolant of an additional data set (X(I),YY(I)), I=1,...,N with the same abscissa can be obtained by loading YY into BCOEF and then executing CALL DBNSLV (Q,2K-1,N,K-1,K-1,BCOEF) WORK - work vector of length 2*K Error Conditions Improper input is a fatal error Singular system of equations is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbintk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbintk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbintk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbintk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::bspline::dbintk`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
