# DDRIV2

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The function of DDRIV2 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. DDRIV2 uses double precision arithmetic.

## Description

I. PARAMETERS ..................................................... (REMEMBER--To run DDRIV2 correctly in double precision, ALL non-integer arguments in the call sequence, including arrays, MUST be declared double precision.) The user should use parameter names in the call sequence of DDRIV2 for those quantities whose value may be altered by DDRIV2. The parameters in the call sequence are: N = (Input) The number of differential equations. T = The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routines F and G. Thus parameters required by F and G can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i.e., after setting MSTATE to +1(-1).) F = A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) DOUBLE PRECISION Y(*), YDOT(*) . . YDOT(1) = ... . . YDOT(N) = ...

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

- Canonical provider: `main-src/src/ddriv2.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddriv2.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddriv2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddriv2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DDRIV2](https://www.netlib.org/slatec/src/ddriv2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input positive equation count; it remains fixed for a continuation problem. |
| 2 | `T` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Mutable independent variable: initial point on the first call and returned solution/root point thereafter. |
| 3 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Mutable length-at-least-`N` solution vector shared with the RHS and optional root callbacks. |
| 4 | `F` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | Required synchronous RHS subroutine callback `F(N,T,Y,YDOT)`. `Y` is readable, `YDOT` is writable for `N` elements, callback-local `N=0` requests a controlled stop, and the callback has no user-data pointer or unwind permission. |
| 5 | `TOUT` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input requested output point for the current continuation call. |
| 6 | `MSTATE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output continuation state. Initialize to `+1` or `-1`; normal completion is `2`, root detection is `5`, controlled callback stops are `6`/`7`, and other documented states require recovery or continuation. |
| 7 | `NROOT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of real root equations. Zero disables root search; otherwise `G` is invoked and the reported root index is stored in `IWORK(6)` using Fortran indexing. |
| 8 | `EPS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input/output relative accuracy request; the routine may raise a too-small value. |
| 9 | `EWT` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input error-weight scale used to form `max(abs(Y(I)), EWT)` for the selected error control. |
| 10 | `MINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input method selector: `1` Adams, `2` Gear, or `3` automatic selection. It must not change without restarting. |
| 11 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Mutable persistent real workspace. Its minimum is `16*N + 2*NROOT + 250` for `MINT=1`, `N*N + 10*N + 2*NROOT + 250` for `2`, or `N*N + 17*N + 2*NROOT + 250` for `3`. |
| 12 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared `WORK` length meeting the selected `MINT` formula. |
| 13 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Mutable persistent integer workspace, at least `50` elements for `MINT=1` or `N+50` for `MINT=2/3`. |
| 14 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared `IWORK` length meeting the selected `MINT` formula. |
| 15 | `G` | `callback` | `callback` | `DOUBLE PRECISION` | `reviewed unsafe extern callback function pointer` | scalar | Synchronous real root-function callback `G(N,T,Y,IROOT)`. It is used only when `NROOT` is nonzero, reads `Y[0..N]`, returns directly, may request a controlled stop through callback-local `N=0`, has no user-data pointer, and must not unwind. |
| 16 | `IERFLG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output diagnostic status corresponding to source-documented warnings and recoverable setup or continuation failures. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and workspace requirements

`WORK`: Mutable persistent real workspace. Its minimum is `16*N + 2*NROOT + 250` for `MINT=1`, `N*N + 10*N + 2*NROOT + 250` for `2`, or `N*N + 17*N + 2*NROOT + 250` for `3`.

`IWORK`: Mutable persistent integer workspace, at least `50` elements for `MINT=1` or `N+50` for `MINT=2/3`.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::ddriv2`. Native symbol: `ddriv2_`. Declaration feature: `ode-sdrive-expert`. Provider feature: `ode-sdrive-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::ode::ddriv2`
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
