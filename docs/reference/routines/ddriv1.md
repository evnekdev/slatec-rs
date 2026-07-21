# DDRIV1

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The function of DDRIV1 is to solve N (200 or fewer) ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. DDRIV1 uses double precision arithmetic.

## Description

Version 92.1 I. CHOOSING THE CORRECT ROUTINE ................................... SDRIV DDRIV CDRIV These are the generic names for three packages for solving initial value problems for ordinary differential equations. SDRIV uses single precision arithmetic. DDRIV uses double precision arithmetic. CDRIV allows complex-valued differential equations, integrated with respect to a single, real, independent variable. As an aid in selecting the proper program, the following is a discussion of the important options or restrictions associated with each program: A. DDRIV1 should be tried first for those routine problems with no more than 200 differential equations (DDRIV2 and DDRIV3 have no such restriction.) Internally this routine has two important technical defaults: 1. Numerical approximation of the Jacobian matrix of the right hand side is used. 2. The stiff solver option is used. Most users of DDRIV1 should not have to concern themselves with these details. B. DDRIV2 should be considered for those problems for which DDRIV1 is inadequate. For example, DDRIV1 may have difficulty with problems having zero initial conditions and zero derivatives. In this case DDRIV2, with an appropriate value of the parameter EWT, should perform more efficiently. DDRIV2 provides three important additional options: 1. The nonstiff equation solver (as well as the stiff solver) is available. 2. The root-finding option is available. 3. The program can dynamically select either the non-stiff or the stiff methods. Internally this routine also defaults to the numerical approximation of the Jacobian matrix of the right hand side. C. DDRIV3 is the most flexible, and hence the most complex, of the programs. Its important additional features include: 1. The ability to exploit band structure in the Jacobian matrix. 2. The ability to solve some implicit differential equations, i.e., those having the form: A(Y,T)*dY/dT = F(Y,T). 3. The option of integrating in the one step mode. 4. The option of allowing the user to provide a routine which computes the analytic Jacobian matrix of the right 5. The option of allowing the user to provide a routine which does all the matrix algebra associated with corrections to the solution components.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/ddriv1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddriv1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddriv1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddriv1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DDRIV1](https://www.netlib.org/slatec/src/ddriv1.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input equation count. The convenience driver requires `1 <= N <= 200`; callback-local changes to its own `N` do not change this argument. |
| 2 | `T` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Mutable independent variable: initial point on the first call and returned solution point thereafter. |
| 3 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Mutable length-at-least-`N` solution vector. It supplies initial data and is overwritten; it also provides the RHS callback's live solution view. |
| 4 | `F` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | Required synchronous RHS subroutine callback `F(N,T,Y,YDOT)`. `Y` is readable and `YDOT` is writable for `N` elements; setting callback-local `N` to zero requests a controlled stop. It has no user-data pointer and must not unwind. |
| 5 | `TOUT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input requested output point. The sign of `MSTATE` chooses interpolation past it or an exact step to it. |
| 6 | `MSTATE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output continuation state. Initialize to `+1` or `-1`; then preserve its magnitude except to restart. Outputs `2` for normal completion, `3..7` for documented warning, abort, interpolation, or recoverable states. |
| 7 | `EPS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input/output relative accuracy request. The routine may raise an overambitious value before continuation. |
| 8 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Mutable persistent real workspace with at least `N*N + 11*N + 300` elements; do not disturb it between continuation calls. |
| 9 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared `WORK` length; it must meet the stated formula. |
| 10 | `IERFLG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output diagnostic status. Zero is normal; the selected prologue documents warnings and recoverable setup or continuation errors. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and workspace requirements

`WORK`: Mutable persistent real workspace with at least `N*N + 11*N + 300` elements; do not disturb it between continuation calls.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::ddriv1`. Native symbol: `ddriv1_`. Declaration feature: `ode-sdrive-expert`. Provider feature: `ode-sdrive-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::ode::ddriv1`
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
