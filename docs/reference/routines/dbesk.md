# DBESK

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.

## Description

Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. 0.0D0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from DBSKNU to start the recursion. If FNU .GE. NULIM, the uniform asymptotic expansion is used for orders FNU and FNU+1 to start the recursion. NULIM is 35 or 70 depending on whether N=1 or N .GE. 2. Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. Description of Arguments Input X,FNU are double precision X - X .GT. 0.0D0 FNU - order of the initial K function, FNU .GE. 0.0D0 KODE - a parameter to indicate the scaling option KODE=1 returns Y(I)= K/sub(FNU+I-1)/(X), I=1,...,N KODE=2 returns Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N N - number of members in the sequence, N .GE. 1 Output Y is double precision Y - a vector whose first N components contain values for the sequence Y(I)= k/sub(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - number of components of Y set to zero due to underflow with KODE=1, NZ=0 , normal return, computation completed NZ .NE. 0, first NZ components of Y set to zero due to underflow, Y(I)=0.0D0, I=1,...,NZ Error Conditions Improper input arguments - a fatal error Overflow - a fatal error Underflow with KODE=1 - a non-fatal error (NZ .NE. 0)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B3`
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

- Canonical provider: `main-src/src/dbesk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbesk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbesk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbesk.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `mangled_source_prologue`
- Description provenance: `source_prologue`
- Assessment: mechanical source-prologue checks found text that requires a documented repair or review
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | 0.0D0 KODE - a parameter to indicate the scaling option KODE=1 returns Y(I)= K/sub(FNU+I-1)/(X), I=1,...,N KODE=2 returns Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N N - number of members in the sequence, N .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Abstract **** a double precision routine **** DBESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,..,N for real X .GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | 0.0D0 KODE - a parameter to indicate the scaling option KODE=1 returns Y(I)= K/sub(FNU+I-1)/(X), I=1,...,N KODE=2 returns Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N N - number of members in the sequence, N .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | 1 Output Y is double precision Y - a vector whose first N components contain values for the sequence Y(I)= k/sub(FNU+I-1)/(X), I=1,...,N or Y(I)=EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N depending on KODE NZ - number of components of Y set to zero due to underflow with KODE=1, NZ=0 , normal return, computation completed NZ .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::special::bessel::dbesk`. Native symbol: `dbesk_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bessel::dbesk`
- Compatibility aliases: `slatec_sys::special::numerical::dbesk`
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
