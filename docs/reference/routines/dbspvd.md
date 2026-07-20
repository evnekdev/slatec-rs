# DBSPVD

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value and all derivatives of order less than NDERIV of all basis functions which do not vanish at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVD is the BSPLVD routine of the reference. DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. ILEFT is input such that T(ILEFT) .LE. X .LT. T(ILEFT+1). A call to INTRV(T,N+1,X, ILO,ILEFT,MFLAG) will produce the proper ILEFT. The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. These basis functions have indices ILEFT-K+I, I=1,K, K .LE. ILEFT .LE. N. The nonzero part of the I-th basis function lies in (T(I),T(I+K)), I=1,N). If X=T(ILEFT+1) then VNIKX contains left limiting values (left derivatives) at T(ILEFT+1). In particular, ILEFT = N produces left limiting values at the right end point X=T(N+1). To obtain left limiting values at T(I), I=K+1,N+1, set X= next lower distinct knot, call INTRV to get ILEFT, set X=T(I), and then call DBSPVD. Description of Arguments Input T,X are double precision T - knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 NDERIV - number of derivatives = NDERIV-1, 1 .LE. NDERIV .LE. K X - argument of basis functions, T(K) .LE. X .LE. T(N+1) ILEFT - largest integer such that T(ILEFT) .LE. X .LT. T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. WORK - a work vector of length (K+1)*(K+2)/2 Error Conditions Improper input is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E3`
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

- Canonical provider: `main-src/src/dbspvd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspvd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspvd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspvd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Interpolation](../families/interpolation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `T` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | ILEFT is input such that T(ILEFT) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. | The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDERIV` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. | DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. | DBSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ILEFT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ILEFT is input such that T(ILEFT) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDVNIK` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. | T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. Leading dimension: T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `VNIKX` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDVNIK, *) | The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. | The output of DBSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. | T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. Leading dimension: T(ILEFT+1) LDVNIK - leading dimension of matrix VNIKX Output VNIKX,WORK are double precision VNIKX - matrix of dimension at least (K,NDERIV) containing the nonzero basis functions at X and their derivatives columnwise. Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::interpolation::dbspvd`. Native symbol: `dbspvd_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbspvd`
- Compatibility aliases: `slatec_sys::interpolation::numerical::dbspvd`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
