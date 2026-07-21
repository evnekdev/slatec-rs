# CHIFA

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Factor a complex Hermitian matrix by elimination (symmetric pivoting).

## Description

CHIFA factors a complex Hermitian matrix by elimination with symmetric pivoting. To solve A*X = B , follow CHIFA by CHISL. To compute INVERSE(A)*C , follow CHIFA by CHISL. To compute DETERMINANT(A) , follow CHIFA by CHIDI. To compute INERTIA(A) , follow CHIFA by CHIDI. To compute INVERSE(A) , follow CHIFA by CHIDI. On Entry

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/chifa.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/chifa.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/chifa.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CHIFA](https://www.netlib.org/slatec/lin/chifa.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | COMPLEX(LDA,N) the Hermitian matrix to be factored. Only the diagonal and upper triangle are used. a block diagonal matrix and the multipliers which were used to obtain it. The factorization can be written A = U*D*CTRANS(U) where U is a product of permutation and unit upper triangular matrices , CTRANS(U) is the conjugate transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the leading dimension of the array A. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the matrix A. |
| 4 | `KPVT` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(N) an integer vector of pivot indices. |
| 5 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER = 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CHISL or CHIDI may divide by zero if called. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that CHISL or CHIDI may divide by zero if called. |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chifa`. Native symbol: `chifa_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chifa`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
