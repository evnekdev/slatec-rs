# CDRIV1

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The function of CDRIV1 is to solve N (200 or fewer) ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. CDRIV1 allows complex-valued differential equations.

## Description

Version 92.1 I. CHOOSING THE CORRECT ROUTINE ................................... SDRIV DDRIV CDRIV These are the generic names for three packages for solving initial value problems for ordinary differential equations. SDRIV uses single precision arithmetic. DDRIV uses double precision arithmetic. CDRIV allows complex-valued differential equations, integrated with respect to a single, real, independent variable. As an aid in selecting the proper program, the following is a discussion of the important options or restrictions associated with each program: A. CDRIV1 should be tried first for those routine problems with no more than 200 differential equations (CDRIV2 and CDRIV3 have no such restriction.) Internally this routine has two important technical defaults: 1. Numerical approximation of the Jacobian matrix of the right hand side is used. 2. The stiff solver option is used. Most users of CDRIV1 should not have to concern themselves with these details. B. CDRIV2 should be considered for those problems for which CDRIV1 is inadequate. For example, CDRIV1 may have difficulty with problems having zero initial conditions and zero derivatives. In this case CDRIV2, with an appropriate value of the parameter EWT, should perform more efficiently. CDRIV2 provides three important additional options: 1. The nonstiff equation solver (as well as the stiff solver) is available. 2. The root-finding option is available. 3. The program can dynamically select either the non-stiff or the stiff methods. Internally this routine also defaults to the numerical approximation of the Jacobian matrix of the right hand side. C. CDRIV3 is the most flexible, and hence the most complex, of the programs. Its important additional features include: 1. The ability to exploit band structure in the Jacobian matrix. 2. The ability to solve some implicit differential equations, i.e., those having the form: A(Y,T)*dY/dT = F(Y,T). 3. The option of integrating in the one step mode. 4. The option of allowing the user to provide a routine which computes the analytic Jacobian matrix of the right 5. The option of allowing the user to provide a routine which does all the matrix algebra associated with corrections to the solution components.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A2`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cdriv1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cdriv1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cdriv1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cdriv1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [CDRIV1](https://www.netlib.org/slatec/src/cdriv1.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input equation count, constrained to `1..=200` by this convenience driver. |
| 2 | `T` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Mutable real independent variable. |
| 3 | `Y` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Mutable length-at-least-`N` `Complex32` solution vector in selected GNU Fortran complex layout. |
| 4 | `F` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | Required synchronous complex RHS subroutine callback `F(N,T,Y,YDOT)`. `Y` is readable and `YDOT` writable for `N` complex values; it has no user-data pointer and must not unwind. |
| 5 | `TOUT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Input real output point for the current integration call. |
| 6 | `MSTATE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output continuation state with the same source-documented completion and recovery protocol as SDRIV1. |
| 7 | `EPS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input/output real relative accuracy request. |
| 8 | `WORK` | `workspace-output` | `workspace` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Mutable persistent `Complex32` workspace with at least `N*N + 11*N + 300` elements. |
| 9 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared `WORK` length meeting the stated formula. |
| 10 | `IERFLG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output diagnostic status for the selected legacy driver. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and workspace requirements

`WORK`: Mutable persistent `Complex32` workspace with at least `N*N + 11*N + 300` elements.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::cdriv1`. Native symbol: `cdriv1_`. Declaration feature: `ode-sdrive-expert`. Provider feature: `ode-sdrive-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::ode::cdriv1`
- Public declaration feature: `ode-sdrive-expert`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
