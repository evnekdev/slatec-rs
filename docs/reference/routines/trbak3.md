# TRBAK3

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED3.

## Description

This subroutine is a translation of the ALGOL procedure TRBAK3, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a REAL SYMMETRIC matrix by back transforming those of the corresponding symmetric tridiagonal matrix determined by TRED3.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C4`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/trbak3.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/trbak3.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [TRBAK3](https://www.netlib.org/slatec/lin/trbak3.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `NV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER variable set equal to the dimension of the array A as specified in the calling program. NV must not be less than N*(N+1)/2. |
| 4 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains information about the orthogonal transformations used in the reduction by TRED3 in its first N*(N+1)/2 positions. A is a one-dimensional REAL array, dimensioned. |
| 5 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of columns of Z to be back transformed. is an INTEGER variable. |
| 6 | `Z` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array, dimensioned Z(NM,M). contains the transformed eigenvectors in its first M columns. Note that TRBAK3 preserves vector Euclidean norms. Questions and comments should be directed to b. s. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::trbak3`. Native symbol: `trbak3_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::trbak3`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
