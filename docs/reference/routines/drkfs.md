# DRKFS

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DDERKF

## Description

Fehlberg Fourth-Fifth Order Runge-Kutta Method ********************************************************************** DRKFS integrates a system of first order ordinary differential equations as described in the comments for DDERKF . The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. Accordingly, these variables and the array YP should not be altered. Items of possible interest are H - An appropriate step size to be used for the next step TOLFAC - Factor of change in the tolerances YP - Derivative of solution vector at T KSTEPS - Counter on the number of steps attempted **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DDERKF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/drkfs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/drkfs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/drkfs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/drkfs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `DF` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Items of possible interest are H - An appropriate step size to be used for the next step TOLFAC - Factor of change in the tolerances YP - Derivative of solution vector at T KSTEPS - Counter on the number of steps attempted ********************************************************************** | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOUT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (15) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RTOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ATOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDID` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOLFAC` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YP` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F1` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F2` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F3` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F4` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F5` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. | The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOLD` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DTSIGN` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `U26` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RER` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INIT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KSTEPS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KOP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IQUIT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `STIFF` | unavailable | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NONSTF` | unavailable | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NTSTEP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NSTIFS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `unsupported-abi`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
