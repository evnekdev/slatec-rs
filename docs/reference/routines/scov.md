# SCOV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either SNLS1 or SNLS1E.

## Description

1. Purpose. SCOV calculates the covariance matrix for a nonlinear data fitting problem. It is intended to be used after a successful return from either SNLS1 or SNLS1E. SCOV and SNLS1 (and SNLS1E) have compatible parameters. The required external subroutine, FCN, is the same for all three codes, SCOV, SNLS1, and SNLS1E. 2. Subroutine and Type Statements. SUBROUTINE SCOV(FCN,IOPT,M,N,X,FVEC,R,LDR,INFO, WA1,WA2,WA3,WA4) INTEGER IOPT,M,N,LDR,INFO REAL X(N),FVEC(M),R(LDR,N),WA1(N),WA2(N),WA3(N),WA4(M) EXTERNAL FCN 3. Parameters. FCN is the name of the user-supplied subroutine which calculates the functions. If the user wants to supply the Jacobian (IOPT=2 or 3), then FCN must be written to calculate the Jacobian, as well as the functions. See the explanation of the IOPT argument below. FCN must be declared in an EXTERNAL statement in the calling program and should be written as follows. SUBROUTINE FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC) INTEGER IFLAG,LDFJAC,M,N REAL X(N),FVEC(M) FJAC and LDFJAC may be ignored , if IOPT=1. REAL FJAC(LDFJAC,N) , if IOPT=2. REAL FJAC(N) , if IOPT=3. IFLAG will never be zero when FCN is called by SCOV. RETURN If IFLAG=1, calculate the functions at X and return this vector in FVEC. RETURN If IFLAG=2, calculate the full Jacobian at X and return this matrix in FJAC. Note that IFLAG will never be 2 unless IOPT=2. FVEC contains the function values at X and must not be altered. FJAC(I,J) must be set to the derivative of FVEC(I) with respect to X(J). RETURN If IFLAG=3, calculate the LDFJAC-th row of the Jacobian and return this vector in FJAC. Note that IFLAG will never be 3 unless IOPT=3. FJAC(J) must be set to the derivative of FVEC(LDFJAC) with respect to X(J). RETURN

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::least_squares::covariance_from_expert_fit_f32, slatec::least_squares::estimate_covariance_f32, slatec::least_squares::estimate_covariance_finite_difference_f32`

## Providers

- Canonical provider: `main-src/src/scov.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/scov.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/scov.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/scov.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `batch_d_public_driver`
- Canonical Rust path: `slatec_sys::least_squares::scov`
- Current legacy Rust paths: `none`
- Public declaration feature: `least-squares-covariance`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::least_squares::covariance_from_expert_fit_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
