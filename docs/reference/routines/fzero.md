# FZERO

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Search for a zero of a function F(X) in a given interval (B,C). It is designed primarily for problems where F(B) and F(C) have opposite signs.

## Description

FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. 2.*(RW*ABS(B)+AE). The method used is an efficient combination of bisection and the secant rule and is due to T. J. Dekker. Description Of Arguments F :EXT - Name of the REAL external function. This name must be in an EXTERNAL statement in the calling program. F must be a function of one REAL argument. B :INOUT - One end of the REAL interval (B,C). The value returned for B usually is the better approximation to a zero of F. C :INOUT - The other end of the REAL interval (B,C) R :OUT - A (better) REAL guess of a zero of F which could help in speeding up convergence. If F(B) and F(R) have opposite signs, a root will be found in the interval (B,R); if not, but F(R) and F(C) have opposite signs, a root will be found in the interval (R,C); otherwise, the interval (B,C) will be searched for a possible root. When no better guess is known, it is recommended that r be set to B or C, since if R is not interior to the interval (B,C), it will be ignored. RE :IN - Relative error used for RW in the stopping criterion. If the requested RE is less than machine precision, then RW is set to approximately machine precision. AE :IN - Absolute error used in the stopping criterion. If the given interval (B,C) contains the origin, then a nonzero value should be chosen for AE. IFLAG :OUT - A status code. User must check IFLAG after each call. Control returns to the user from FZERO in all cases. 1 B is within the requested tolerance of a zero. The interval (B,C) collapsed to the requested tolerance, the function changes sign in (B,C), and F(X) decreased in magnitude as (B,C) collapsed. 2 F(B) = 0. However, the interval (B,C) may not have collapsed to the requested tolerance. 3 B may be near a singular point of F(X). The interval (B,C) collapsed to the requested tolerance and the function changes sign in (B,C), but F(X) increased in magnitude as (B,C) collapsed, i.e. ABS(F(B out)) .GT. MAX(ABS(F(B in)),ABS(F(C in))) 4 No change in sign of F(X) was found although the interval (B,C) collapsed to the requested tolerance. The user must examine this case and decide whether B is near a local minimum of F(X), or B is near a zero of even multiplicity, or neither of these. 5 Too many (.GT. 500) function evaluations used.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::roots::find_root_f32`

## Providers

- Canonical provider: `main-src/src/fzero.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fzero.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fzero.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/fzero.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Nonlinear equations](../families/nonlinear-equations.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | input/output | `REAL` (`implicit_rule`) | `*mut f32` | scalar | FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | FZERO searches for a zero of a REAL function F(X) between the given REAL values B and C until the width of the interval (B,C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B-C) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | C :INOUT - The other end of the REAL interval (B,C) R :OUT - A (better) REAL guess of a zero of F which could help in speeding up convergence. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RE` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | RE :IN - Relative error used for RW in the stopping criterion. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AE` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | 2.*(RW*ABS(B)+AE). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFLAG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IFLAG :OUT - A status code. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::roots::scalar::fzero`. Native symbol: `fzero_`. Feature: `raw-family-roots-scalar`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::roots::scalar::fzero`
- Compatibility aliases: `slatec_sys::roots::fzero`
- Public declaration feature: `raw-family-roots-scalar`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::roots::find_root_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
