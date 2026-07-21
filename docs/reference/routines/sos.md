# SOS

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a square system of nonlinear equations.

## Description

SOS solves a system of NEQ simultaneous nonlinear equations in NEQ unknowns. That is, it solves the problem F(X)=0 where X is a vector with components X(1),...,X(NEQ) and F is a vector of nonlinear functions. Each equation is of the form F (X(1),...,X(NEQ))=0 for K=1,...,NEQ. The algorithm is based on an iterative method which is a variation of Newton's method using Gaussian elimination in a manner similar to the Gauss-Seidel process. Convergence is roughly quadratic. All partial derivatives required by the algorithm are approximated by first difference quotients. The convergence behavior of this code is affected by the ordering of the equations, and it is advantageous to place linear and mildly nonlinear equations first in the ordering. Actually, SOS is merely an interfacing routine for calling subroutine SOSEQS which embodies the solution algorithm. The purpose of this is to add greater flexibility and ease of use for the prospective user. SOSEQS calls the accompanying routine SOSSOL, which solves special triangular linear systems by back-substitution. The user must supply a function subprogram which evaluates the K-th equation only (K specified by SOSEQS) for each call to the subprogram. SOS represents an implementation of the mathematical algorithm described in the references below. It is a modification of the code SOSNLE written by H. A. Watts in 1973.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sos.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sos.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sos.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sos.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [SOS](https://www.netlib.org/slatec/src/sos.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FNC` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | Synchronous scalar equation callback. It receives a readable length-`NEQ` current or finite-difference-perturbed solution vector and a one-based equation index `K` in `1..=NEQ`, then returns equation `K`; it must not retain pointers or unwind through Fortran. |
| 2 | `NEQ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input number of equations and unknowns. It must be positive and is the required `X` length. |
| 3 | `X` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Mutable length-`NEQ` solution vector. It supplies the initial estimate and is overwritten with the current iterate; the native finite-difference calculation temporarily perturbs components before callback evaluations. |
| 4 | `RTOLX` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input nonnegative relative solution-increment tolerance used with `ATOLX` in the componentwise convergence test. |
| 5 | `ATOLX` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input nonnegative absolute solution-increment tolerance. A positive value is useful when a solution component can be zero. |
| 6 | `TOLF` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input nonnegative residual tolerance. Residual convergence requires every equation residual magnitude to be at most `TOLF`; its value must match equation scaling. |
| 7 | `IFLAG` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input/output control and status integer. Input `0` uses defaults and input `-1` enables `IW(1..=2)` controls; on return it reports the documented convergence or error status. |
| 8 | `RW` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Mutable real workspace of at least `1 + 6*NEQ + NEQ*(NEQ + 1)/2` elements. `RW(1)` receives the residual norm on return. |
| 9 | `LRW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared length of `RW`; it must be at least `1 + 6*NEQ + NEQ*(NEQ + 1)/2`. |
| 10 | `IW` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Mutable integer workspace of at least `3 + NEQ` elements. With input `IFLAG=-1`, `IW(1)=-1` requests iteration output and `IW(2)` supplies a positive iteration limit; `IW(3)` receives the iteration count. |
| 11 | `LIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Input declared length of `IW`; it must be at least `3 + NEQ`. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::nonlinear::systems::sos`. Native symbol: `sos_`. Declaration feature: `nonlinear-systems`. Provider feature: `nonlinear-systems`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::nonlinear::systems::sos`
- Public declaration feature: `nonlinear-systems`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
