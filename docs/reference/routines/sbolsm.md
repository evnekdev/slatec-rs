# SBOLSM

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SBOCLS and SBOLS

## Description

Solve E*X = F (least squares sense) with bounds on selected X values. The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SBOCLS, SBOLS`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sbolsm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sbolsm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sbolsm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sbolsm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Approximation](../families/approximation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (MDW, *) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MINPUT` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NCOLS` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BL` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BU` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IOPT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Solve E*X = F (least squares sense) with bounds on selected X values. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RW` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WW` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SCL` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IBASIS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IBB` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) | The user must have DIMENSION statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(NCOLS), WW(NCOLS), SCL(NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IBASIS(NCOLS), IBB(NCOLS) (Here NX=number of extra locations required for options 1,...,7; NX=0 for no options; here NI=number of extra locations possibly required for options 1-7; NI=0 for no options; NI=14 if all the options are simultaneously in use.) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
