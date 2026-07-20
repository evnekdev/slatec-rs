# QK15I

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The original (infinite integration range is mapped onto the interval (0,1) and (A,B) is a part of (0,1). it is the purpose to compute I = Integral of transformed integrand over (A,B), J = Integral of ABS(Transformed Integrand) over (A,B).

## Description

Integration Rule Standard Fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand FUNCTION F(X). The actual name for F needs to be Declared E X T E R N A L in the calling program. BOUN - Real Finite bound of original integration Range (SET TO ZERO IF INF = +2) INF - Integer If INF = -1, the original interval is (-INFINITY,BOUND), If INF = +1, the original interval is (BOUND,+INFINITY), If INF = +2, the original interval is (-INFINITY,+INFINITY) AND The integral is computed as the sum of two integrals, one over (-INFINITY,0) and one over (0,+INFINITY). A - Real Lower limit for integration over subrange of (0,1) B - Real Upper limit for integration over subrange of (0,1) ON RETURN RESULT - Real Approximation to the integral I Result is computed by applying the 15-POINT KRONROD RULE(RESK) obtained by optimal addition of abscissae to the 7-POINT GAUSS RULE(RESG). ABSERR - Real Estimate of the modulus of the absolute error, WHICH SHOULD EQUAL or EXCEED ABS(I-RESULT) RESABS - Real Approximation to the integral J RESASC - Real Approximation to the integral of ABS((TRANSFORMED INTEGRAND)-I/(B-A)) over (A,B)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A3A2`
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

- Canonical provider: `main-src/src/qk15i.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qk15i.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qk15i.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qk15i.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `batch_b_automated_public`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qk15i`
- Current legacy Rust paths: `none`
- Public declaration feature: `batch-b-quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `representative_batch_smoke_only`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
