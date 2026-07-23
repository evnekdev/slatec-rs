# BESI

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.

## Description

BESI computes an N member sequence of I Bessel functions I/sub(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity, and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values not covered by one of these formulae, the order is incremented by an integer so that one of these formulae apply. Backward recursion is used to reduce orders by integer values. The asymptotic expansion for X to infinity is used only when the entire sequence (specifically the last member) lies within the region covered by the expansion. Leading terms of these expansions are used to test for over or underflow where appropriate. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all are set to zero. An overflow cannot occur with scaling.

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

- Canonical provider: `main-src/src/besi.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/besi.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/besi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/besi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [BESI](https://www.netlib.org/slatec/src/besi.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | X. GE. 0. 0E0. |
| 2 | `ALPHA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | order of first member of the sequence,. GE. 0. 0E0. |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a parameter to indicate the scaling option 1 returns 2 returns. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of members in the sequence, N. GE. 1. |
| 5 | `Y` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | I/sub(ALPHA+K-1)/(X), K=1,. ,N EXP(-X)*I/sub(ALPHA+K-1)/(X), a vector whose first N components contain values for I/sub(ALPHA+K-1)/(X) or scaled values for EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,. ,N depending on KODE 0. 0E0, K=N-NZ+1,. ,N. |
| 6 | `NZ` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of components of Y set to zero due to underflow, NZ=0 , normal return, computation completed. NE. 0, last NZ components of Y set to zero,. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | , normal return, computation completed |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::bessel::besi`. Native symbol: `besi_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::bessel::besi`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
