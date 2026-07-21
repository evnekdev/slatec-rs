# DROT

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply a plane Givens rotation.

## Description

B L A S Subprogram Description of Parameters

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A8`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level1::drot, slatec::blas::level1::drot_strided`

## Providers

- Canonical provider: `lin/drot.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/drot.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DROT](https://www.netlib.org/slatec/lin/drot.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of elements in input vector(s). |
| 2 | `DX` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | double precision vector with N elements rotated vector DX (unchanged if N. LE. 0). |
| 3 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of DX. |
| 4 | `DY` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | double precision vector with N elements rotated vector DY (unchanged if N. LE. 0) Multiply the 2 x 2 matrix ( DC DS) times the 2 x N matrix (DX**T) (-DS DC) (DY**T) where **T indicates transpose. The elements of DX are in DX(LX+I*INCX), I = 0 to N-1, where LX = 1 if INCX. GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. |
| 5 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of DY. |
| 6 | `DC` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | D. P. element of rotation matrix. |
| 7 | `DS` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | D. P. element of rotation matrix. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::drot`. Native symbol: `drot_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::drot`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::blas::level1::drot`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
