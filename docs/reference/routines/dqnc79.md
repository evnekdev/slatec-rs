# DQNC79

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `FUN` | callback | `DOUBLE PRECISION` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Description of Arguments --Input--* FUN, A, B, ERR are DOUBLE PRECISION * FUN - name of external function to be integrated. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract *** a DOUBLE PRECISION routine *** DQNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. | Abstract *** a DOUBLE PRECISION routine *** DQNC79 is a general purpose program for evaluation of one dimensional integrals of user defined functions. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Description of Arguments --Input--* FUN, A, B, ERR are DOUBLE PRECISION * FUN - name of external function to be integrated. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ERR` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Description of Arguments --Input--* FUN, A, B, ERR are DOUBLE PRECISION * FUN - name of external function to be integrated. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ANS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | One measure of the reliability of DQNC79 is the output parameter K, giving the number of integrand evaluations that were needed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::dqnc79`. Native symbol: `dqnc79_`. Feature: `quadrature-nonadaptive`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqnc79`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-nonadaptive`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_nc79`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
