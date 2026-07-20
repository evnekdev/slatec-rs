# LSEI

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality and inequality constraints, and optionally compute a covariance matrix.

## Description

Abstract This subprogram solves a linearly constrained least squares problem with both equality and inequality constraints, and, if the user requests, obtains a covariance matrix of the solution parameters. Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. This subroutine solves the linearly constrained least squares problem EX = F, (E ME by N) (equations to be exactly satisfied) AX = B, (A MA by N) (equations to be approximately satisfied, least squares sense) GX .GE. H,(G MG by N) (inequality constraints) The inequalities GX .GE. H mean that every component of the product GX must be .GE. the corresponding component of H. In case the equality constraints cannot be satisfied, a generalized inverse solution residual vector length is obtained for F-EX. This is the minimal length possible for F-EX. Any values ME .GE. 0, MA .GE. 0, or MG .GE. 0 are permitted. The rank of the matrix E is estimated during the computation. We call this value KRANKE. It is an output parameter in IP(1) defined below. Using a generalized inverse solution of EX=F, a reduced least squares problem with inequality constraints is obtained. The tolerances used in these tests for determining the rank of E and the rank of the reduced least squares problem are given in Sandia Tech. Rept. SAND-78-1290. They can be modified by the user if new values are provided in the option list of the array PRGOPT(*). The user must dimension all arrays appearing in the call list.. W(MDW,N+1),PRGOPT(*),X(N),WS(2*(ME+N)+K+(MG+2)*(N+7)),IP(MG+2*N+2) where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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
- Safe Rust paths: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/lsei.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/lsei.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/lsei.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/lsei.f) — `verified_cached`
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
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (MDW, *) | W(MDW,N+1),PRGOPT(*),X(N),WS(2*(ME+N)+K+(MG+2)*(N+7)),IP(MG+2*N+2) where K=MAX(MA+MG,N). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | W(MDW,N+1),PRGOPT(*),X(N),WS(2*(ME+N)+K+(MG+2)*(N+7)),IP(MG+2*N+2) where K=MAX(MA+MG,N). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ME` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. | Suppose there are given matrices E, A and G of respective dimensions ME by N, MA by N and MG by N, and vectors F, B and H of respective lengths ME, MA and MG. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PRGOPT` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | They can be modified by the user if new values are provided in the option list of the array PRGOPT(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | W(MDW,N+1),PRGOPT(*),X(N),WS(2*(ME+N)+K+(MG+2)*(N+7)),IP(MG+2*N+2) where K=MAX(MA+MG,N). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORME` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORML` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WS` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | W(MDW,N+1),PRGOPT(*),X(N),WS(2*(ME+N)+K+(MG+2)*(N+7)),IP(MG+2*N+2) where K=MAX(MA+MG,N). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IP` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (3) | It is an output parameter in IP(1) defined below. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::approximation::lsei`. Native symbol: `lsei_`. Feature: `approximation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::lsei`
- Compatibility aliases: `slatec_sys::approximation::numerical::lsei`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
