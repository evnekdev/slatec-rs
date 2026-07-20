# DEFE4

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SEPX4

## Description

This subroutine first approximates the truncation error given by TRUN1(X,Y)=DLX**2*TX+DLY**2*TY where TX=AFUN(X)*UXXXX/12.0+BFUN(X)*UXXX/6.0 on the interior and at the boundaries if periodic (here UXXX,UXXXX are the third and fourth partial derivatives of U with respect to X). TX is of the form AFUN(X)/3.0*(UXXXX/4.0+UXXX/DLX) at X=A or X=B if the boundary condition there is mixed. TX=0.0 along specified boundaries. TY has symmetric form in Y with X,AFUN(X),BFUN(X) replaced by Y,DFUN(Y),EFUN(Y). The second order solution in USOL is used to approximate (via second order finite differencing) the truncation error and the result is added to the right hand side in GRHS and then transferred to USOL to be used as a new right hand side when calling BLKTRI for a fourth order solution.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SEPX4`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/defe4.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/defe4.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/defe4.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/defe4.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `COFX` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDMN` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `USOL` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDMN, *) | The second order solution in USOL is used to approximate (via second order finite differencing) the truncation error and the result is added to the right hand side in GRHS and then transferred to USOL to be used as a new right hand side when calling BLKTRI for a fourth order solution. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `GRHS` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (IDMN, *) | The second order solution in USOL is used to approximate (via second order finite differencing) the truncation error and the result is added to the right hand side in GRHS and then transferred to USOL to be used as a new right hand side when calling BLKTRI for a fourth order solution. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
