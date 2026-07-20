# DBSPPP

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Convert the B-representation of a B-spline to the piecewise polynomial (PP) form.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPPP is the BSPLPP routine of the reference. DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. Here XI(*), the break point array of length LXI, is the knot array T(*) with multiplicities removed. The columns of the matrix C(I,J) contain the right Taylor derivatives for the polynomial expansion about XI(J) for the intervals XI(J) .LE. X .LE. XI(J+1), I=1,K, J=1,LXI. Function DPPVAL makes this evaluation at a specified point X in XI(1) .LE. X .LE. XI(LXI+1) Description of Arguments Input T,A are double precision T - knot vector of length N+K A - B-spline coefficient vector of length N N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 LDC - leading dimension of C, LDC .GE. K Output C,XI,WORK are double precision C - matrix of dimension at least (K,LXI) containing right derivatives at break points XI - XI break point vector of length LXI+1 LXI - number of break points, LXI .LE. N-K+1 WORK - work vector of length K*(N+3) Error Conditions Improper input is a fatal error

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

- Canonical provider: `main-src/src/dbsppp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbsppp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbsppp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbsppp.f) — `verified_cached`
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
| `T` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Amos Abstract **** a double precision routine **** DBSPPP is the BSPLPP routine of the reference. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDC` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | 1 LDC - leading dimension of C, LDC .GE. | 1 LDC - leading dimension of C, LDC .GE. Leading dimension: 1 LDC - leading dimension of C, LDC .GE. Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDC, *) | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XI` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LXI` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | K Output C,XI,WORK are double precision C - matrix of dimension at least (K,LXI) containing right derivatives at break points XI - XI break point vector of length LXI+1 LXI - number of break points, LXI .LE. | K Output C,XI,WORK are double precision C - matrix of dimension at least (K,LXI) containing right derivatives at break points XI - XI break point vector of length LXI+1 LXI - number of break points, LXI .LE. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::interpolation::dbsppp`. Native symbol: `dbsppp_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbsppp`
- Compatibility aliases: `slatec_sys::interpolation::numerical::dbsppp`
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
