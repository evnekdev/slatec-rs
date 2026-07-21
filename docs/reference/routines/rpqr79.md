# RPQR79

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find the zeros of a polynomial with real coefficients.

## Description

This routine computes all zeros of a polynomial of degree NDEG with real coefficients by computing the eigenvalues of the companion matrix. Description of Parameters The user must dimension all arrays appearing in the call list COEFF(NDEG+1), ROOT(NDEG), WORK(NDEG*(NDEG+2))

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
- GAMS classifications: `F1A1A`
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

- Canonical provider: `main-src/src/rpqr79.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rpqr79.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rpqr79.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rpqr79.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [RPQR79](https://www.netlib.org/slatec/src/rpqr79.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDEG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | degree of polynomial. |
| 2 | `COEFF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL coefficients in descending order. i. e. , P(Z)= COEFF(1)*(Z**NDEG) + COEFF(NDEG)*Z + COEFF(NDEG+1). |
| 3 | `ROOT` | `output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX vector of roots. |
| 4 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Output Error Code - Normal Code 0 means the roots were computed. - Abnormal Codes 1 more than 30 QR iterations on some eigenvalue of the companion matrix 2 COEFF(1)=0. 0 3 NDEG is invalid (less than or equal to 0). |
| 5 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL work array of dimension at least NDEG*(NDEG+2). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WORK`: REAL work array of dimension at least NDEG*(NDEG+2).

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::roots::complex::rpqr79`. Native symbol: `rpqr79_`. Declaration feature: `nonlinear-complex`. Provider feature: `nonlinear-complex`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::roots::complex::rpqr79`
- Public declaration feature: `nonlinear-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
