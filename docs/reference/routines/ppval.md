# PPVAL

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the IDERIV-th derivative of the B-spline from the PP-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract PPVAL is the PPVALU function of the reference. PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). The Taylor expansion about XI(J) for X in the interval XI(J) .LE. X .LT. XI(J+1) is evaluated, J=1,LXI. Right limiting values at X=XI(J) are obtained. PPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J), replace LXI by J-1 and set X=XI(J),J=2,LXI+1. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative, slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate, slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into`

## Providers

- Canonical provider: `main-src/src/ppval.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ppval.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ppval.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ppval.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Interpolation](../families/interpolation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `LDC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDC, *) | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LXI` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDERIV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INPPV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f32(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`.

### ABI and safety

Canonical path: `slatec_sys::interpolation::ppval`. Native symbol: `ppval_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f32(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::ppval`
- Compatibility aliases: `slatec_sys::interpolation::numerical::ppval`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
