# DBSKIN

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute repeated integrals of the K-zero Bessel function.

## Description

The following definitions are used in DBSKIN: Definition 1 KI(0,X) = K-zero Bessel function. Definition 2 KI(N,X) = Bickley Function = integral from X to infinity of KI(N-1,t)dt for X .ge. 0 and N = 1,2,... _____________________________________________________________________ DBSKIN computes a sequence of Bickley functions (repeated integrals of the K0 Bessel function); i.e. for fixed X and N and for K=1,..., DBSKIN computes the sequence

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

- Canonical provider: `main-src/src/dbskin.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbskin.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbskin.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbskin.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DBSKIN](https://www.netlib.org/slatec/src/dbskin.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Argument, X. ge. 0. 0D0. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Order of first member of the sequence N. ge. 0. |
| 3 | `KODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Selection parameter 1 returns Y(K)= KI(N+K-1,X), K=1,M = 2 returns Y(K)=EXP(X)*KI(N+K-1,X), K=1,M 1. Y(K)=0. 0D0, K=1,. ,M is returned 1 AND Y(K)=0. 0E0, K=1,. ,M IS RETURNED. |
| 4 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of members in the sequence, M. ge. 1 OUTPUT Y is a DOUBLE PRECISION VECTOR. |
| 5 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | KI(N+K-1,X) for KODE=1 EXP(X)*KI(N+K-1,X) for KODE=2, for N. ge. 0 and X. 0 (N and X cannot be zero simultaneously). INPUT X is DOUBLE PRECISION A vector of dimension at least M containing the sequence selected by KODE. |
| 6 | `NZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Underflow flag NZ = 0 means computation completed = 1 means an exponential underflow occurred on. |
| 7 | `IERR` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Error flag 0, Normal return, computation completed 1, Input error, no computation 2, Error, no computation Algorithm termination condition not met The nominal computational accuracy is the maximum of unit roundoff (=D1MACH(4)) and 1. 0D-18 since critical constants are given to only 18 digits. BSKIN is the single precision version of DBSKIN. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | 0 means computation completed |
| `NZ` | `1` | 1 means an exponential underflow occurred on |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dbskin`. Native symbol: `dbskin_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dbskin`
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
