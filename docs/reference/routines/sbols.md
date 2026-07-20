# SBOLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve the problem E*X = F (in the least squares sense) with bounds on selected X values.

## Description

The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.)

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
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::bounded_least_squares::solve_bounded_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/sbols.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sbols.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sbols.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sbols.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (MDW, *) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MROWS` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NCOLS` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BL` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BU` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IOPT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RW` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. | The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::approximation::sbols`. Native symbol: `sbols_`. Feature: `approximation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::sbols`
- Compatibility aliases: `slatec_sys::approximation::numerical::sbols`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::bounded_least_squares::solve_bounded_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
