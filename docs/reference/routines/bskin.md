# BSKIN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute repeated integrals of the K-zero Bessel function.

## Description

The following definitions are used in BSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... ____________________________________________________________________ BSKIN computes sequences of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and K=1,..., BSKIN computes the M-member sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10F`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/bskin.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bskin.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bskin.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bskin.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | The following definitions are used in BSKIN: Definition 1 KI(0,X) = K-zero Bessel function. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | for fixed X and N and K=1,..., BSKIN computes the M-member sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | for fixed X and N and K=1,..., BSKIN computes the M-member sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | for fixed X and N and K=1,..., BSKIN computes the M-member sequence Y(K) = KI(N+K-1,X) for KODE=1 or Y(K) = EXP(X)*KI(N+K-1,X) for KODE=2, for N.ge.0 and X.ge.0 (N and X cannot be zero simultaneously). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::special::bskin`. Native symbol: `bskin_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bskin`
- Compatibility aliases: `slatec_sys::special::numerical::bskin`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
