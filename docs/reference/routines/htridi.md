# HTRIDI

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Reduce a complex Hermitian matrix to a real symmetric tridiagonal matrix using unitary similarity transformations.

## Description

This subroutine is a translation of a complex analogue of the ALGOL procedure TRED1, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine reduces a COMPLEX HERMITIAN matrix to a real symmetric tridiagonal matrix using unitary similarity transformations.

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
- GAMS classifications: `D4C1B1`
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

- Canonical provider: `lin/htridi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/htridi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/htridi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [HTRIDI](https://www.netlib.org/slatec/lin/htridi.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, AR and AI, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM. |
| 3 | `AR` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex Hermitian input matrix. Only the lower triangle of the matrix need be supplied. AR and AI are two- dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). some information about the unitary trans- formations used in the reduction in the strict lower triangle of AR and the full lower triangle of AI. The rest of the matrices are unaltered. |
| 4 | `AI` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex Hermitian input matrix. Only the lower triangle of the matrix need be supplied. AR and AI are two- dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). some information about the unitary trans- formations used in the reduction in the strict lower triangle of AR and the full lower triangle of AI. The rest of the matrices are unaltered. |
| 5 | `D` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the diagonal elements of the real symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). |
| 6 | `E` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the subdiagonal elements of the real tridiagonal matrix in its last N-1 positions. E(1) is set to zero. is a one-dimensional REAL array, dimensioned E(N). |
| 7 | `E2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the squares of the corresponding elements of E. is set to zero. E2 may coincide with E if the squares are not needed. E2 is a one-dimensional REAL array, dimensioned E2(N). |
| 8 | `TAU` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (2, *) | contains further information about the transformations. is a one-dimensional REAL array, dimensioned TAU(2,N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::htridi`. Native symbol: `htridi_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::htridi`
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
