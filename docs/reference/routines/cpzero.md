# CPZERO

[Family: Nonlinear equations](../families/nonlinear-equations.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find the zeros of a polynomial with complex coefficients.

## Description

Find the zeros of the complex polynomial P(Z)= A(1)*Z**N + A(2)*Z**(N-1) +...+ A(N+1)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- GAMS classifications: `F1A1B`
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

- Canonical provider: `main-src/src/cpzero.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cpzero.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cpzero.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cpzero.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CPZERO](https://www.netlib.org/slatec/src/cpzero.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `IN` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | degree of P(Z). |
| 2 | `A` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | complex vector containing coefficients of P(Z), coefficient of Z**(N+1-i). |
| 3 | `R` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | N word complex vector containing initial estimates for zeros if these are known. Ith zero,. |
| 4 | `T` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | 4(N+1) word array used for temporary storage. |
| 5 | `IFLG` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | flag to indicate if initial estimates of zeros are input. If IFLG. EQ. 0, no estimates are input. NE. 0, the vector R contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated. |
| 6 | `S` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | an N word array bound for R(I). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::roots::complex::cpzero`. Native symbol: `cpzero_`. Declaration feature: `nonlinear-complex`. Provider feature: `nonlinear-complex`. ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::roots::complex::cpzero`
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
