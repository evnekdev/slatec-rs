# CDRIV2

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The function of CDRIV2 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. CDRIV2 allows complex-valued differential equations.

## Description

I. PARAMETERS ..................................................... The user should use parameter names in the call sequence of CDRIV2 for those quantities whose value may be altered by CDRIV2. The parameters in the call sequence are: N = (Input) The number of differential equations. T = (Real) The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = (Complex) The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided

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

- Canonical provider: `main-src/src/cdriv2.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cdriv2.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cdriv2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cdriv2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [CDRIV2](https://www.netlib.org/slatec/src/cdriv2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input positive equation count. |
| 2 | `T` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Mutable real independent variable. |
| 3 | `Y` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Mutable length-at-least-`N` `Complex32` solution vector shared with callbacks. |
| 4 | `F` | `callback` | `callback` | `UNKNOWN` | `reviewed unsafe extern callback function pointer` | scalar | Required synchronous complex RHS subroutine callback `F(N,T,Y,YDOT)`. It has no user-data pointer and must not unwind. |
| 5 | `TOUT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Input real output point for the current continuation call. |
| 6 | `MSTATE` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output continuation state; root and controlled-stop values follow the selected CDRIV2 prologue. |
| 7 | `NROOT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input count of real root equations; zero disables `G` and otherwise records a one-based root index in `IWORK(6)`. |
| 8 | `EPS` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input/output real relative accuracy request. |
| 9 | `EWT` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input real error-weight scale used for complex solution error control. |
| 10 | `MINT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input Adams (`1`), Gear (`2`), or automatic (`3`) method selector; restart before changing it. |
| 11 | `WORK` | `workspace-output` | `workspace` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Mutable persistent `Complex32` workspace. Its minimum is `16*N + 2*NROOT + 250`, `N*N + 10*N + 2*NROOT + 250`, or `N*N + 17*N + 2*NROOT + 250` for `MINT=1`, `2`, or `3` respectively. |
| 12 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared complex workspace length. |
| 13 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Mutable persistent integer workspace with at least `50` elements for `MINT=1` or `N+50` otherwise. |
| 14 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared integer workspace length. |
| 15 | `G` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Optional synchronous real root-function callback `G(N,T,Y,IROOT)` over the complex solution vector. It is used only when `NROOT` is nonzero, returns an `f32` directly, has no user-data pointer, and must not unwind. |
| 16 | `IERFLG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output diagnostic status for warnings and recoverable failures. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and workspace requirements

`WORK`: Mutable persistent `Complex32` workspace. Its minimum is `16*N + 2*NROOT + 250`, `N*N + 10*N + 2*NROOT + 250`, or `N*N + 17*N + 2*NROOT + 250` for `MINT=1`, `2`, or `3` respectively.

`IWORK`: Mutable persistent integer workspace with at least `50` elements for `MINT=1` or `N+50` otherwise.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::cdriv2`. Native symbol: `cdriv2_`. Declaration feature: `ode-sdrive-expert`. Provider feature: `ode-sdrive-expert`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::ode::cdriv2`
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
