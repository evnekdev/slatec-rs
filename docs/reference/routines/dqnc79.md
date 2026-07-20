# DQNC79

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Integrate a function using a 7-point adaptive Newton-Cotes quadrature rule.

## Description

Abstract *** a DOUBLE PRECISION routine *** DQNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. DQNC79 will pick its own points for evaluation of the integrand and these will vary from problem to problem. Thus, DQNC79 is not designed to integrate over data sets. Moderately smooth integrands will be integrated efficiently and reliably. For problems with strong singularities, oscillations etc., the user may wish to use more sophisticated routines such as those in QUADPACK. One measure of the reliability of DQNC79 is the output parameter K, giving the number of integrand evaluations that were needed. Description of Arguments --Input--* FUN, A, B, ERR are DOUBLE PRECISION * FUN - name of external function to be integrated. This name must be in an EXTERNAL statement in your calling program. You must write a Fortran function to evaluate FUN. This should be of the form DOUBLE PRECISION FUNCTION FUN (X) C C X can vary from A to B C FUN(X) should be finite for all X on interval. C FUN = ... RETURN

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
- GAMS classifications: `H2A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_nc79`

## Providers

- Canonical provider: `main-src/src/dqnc79.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqnc79.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqnc79.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqnc79.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::quadrature::dqnc79`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `slatec::quadrature::integrate_nc79`
- Exclusion or deferment reason: `pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes`
<!-- raw-api-status:end -->
