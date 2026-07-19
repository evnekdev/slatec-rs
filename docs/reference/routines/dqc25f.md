# DQC25F

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To compute the integral I=Integral of F(X) over (A,B) Where W(X) = COS(OMEGA*X) or W(X)=SIN(OMEGA*X) and to compute J = Integral of ABS(F) over (A,B). For small value of OMEGA or small intervals (A,B) the 15-point GAUSS-KRONRO Rule is used. Otherwise a generalized CLENSHAW-CURTIS method is used.

## Description

Integration rules for functions with COS or SIN factor Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the calling program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the WEIGHT function INTEGR - Integer Indicates which WEIGHT function is to be used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) NRMOM - Integer The length of interval (A,B) is equal to the length of the original integration interval divided by 2**NRMOM (we suppose that the routine is used in an adaptive integration process, otherwise set NRMOM = 0). NRMOM must be zero at the first call. MAXP1 - Integer Gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(BB-AA)*2**(-L), L = 0,1,2, ..., MAXP1-2. KSAVE - Integer Key which is one when the moments for the current interval have been computed ON RETURN RESULT - Double precision Approximation to the integral I ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations RESABS - Double precision Approximation to the integral J RESASC - Double precision Approximation to the integral of ABS(F-I/(B-A)) ON ENTRY AND RETURN MOMCOM - Integer For each interval length we need to compute the Chebyshev moments. MOMCOM counts the number of intervals for which these moments have already been computed. If NRMOM.LT.MOMCOM or KSAVE = 1, the Chebyshev moments for the interval (A,B) have already been computed and stored, otherwise we compute them and we increase MOMCOM. CHEBMO - Double precision Array of dimension at least (MAXP1,25) containing the modified Chebyshev moments for the first MOMCOM MOMCOM interval lengths ......................................................................

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqc25f.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqc25f.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqc25f.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqc25f.f) — `verified_cached`
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
