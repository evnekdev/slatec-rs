# DEFEHL

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DERKF

## Description

Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. DEFEHL advances the solution over the fixed step H and returns the fifth order (sixth order accurate locally) solution approximation at T+H in the array YS(*). F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. The formulas have been grouped to control loss of significance. DEFEHL should be called with an H not smaller than 13 units of roundoff in T so that the various independent arguments can be distinguished. This subroutine has been written with all variables and statement numbers entirely compatible with DERKFS. For greater efficiency, the call to DEFEHL can be replaced by the module beginning with line 222 and extending to the last line just before the return statement. **********************************************************************

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
- Parent-family evidence: `DERKF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/defehl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/defehl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/defehl.f) — `verified_cached`
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
| `F` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | DEFEHL advances the solution over the fixed step H and returns the fifth order (sixth order accurate locally) solution approximation at T+H in the array YS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YP` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DEFEHL integrates a system of NEQ first order ordinary differential equations of the form dU/DX = F(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = F(T,Y), are given at the starting point X=T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F1` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. | F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F2` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F3` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F4` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `F5` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. | F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YS` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | DEFEHL advances the solution over the fixed step H and returns the fifth order (sixth order accurate locally) solution approximation at T+H in the array YS(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
