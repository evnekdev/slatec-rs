# POLYVL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of a polynomial and its first NDER derivatives where the polynomial was produced by a previous call to POLINT.

## Description

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Abstract Subroutine POLYVL calculates the value of the polynomial and its first NDER derivatives where the polynomial was produced by a previous call to POLINT. The variable N and the arrays X and C must not be altered between the call to POLINT and the call to POLYVL. ****** Dimensioning Information ******* YP must be dimensioned by at least NDER X must be dimensioned by at least N (see the abstract ) C must be dimensioned by at least N (see the abstract ) WORK must be dimensioned by at least 2*N if NDER is .GT. 0. *** Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. ***** Input parameters NDER - the number of derivatives to be evaluated XX - the argument at which the polynomial and its derivatives are to be evaluated. N - ***** * N, X, and C must not be altered between the call X - * to POLINT and the call to POLYVL. C - ***** ***** Output Parameters YFIT - the value of the polynomial at XX YP - the derivatives of the polynomial at XX. The derivative of order J at XX is stored in YP(J) , J = 1,...,NDER. IERR - Output error flag with the following possible values. = 1 indicates normal execution ***** Storage Parameters WORK = this is an array to provide internal working storage for POLYVL. It must be dimensioned by at least 2*N if NDER is .GT. 0. If NDER=0, WORK does not need to be a dimensioned variable.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E3`
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

- Canonical provider: `main-src/src/polyvl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/polyvl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/polyvl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::interpolation::numerical::polyvl`
- Current legacy Rust paths: `none`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
