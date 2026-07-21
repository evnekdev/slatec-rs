# TQLRAT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues of symmetric tridiagonal matrix using a rational variant of the QL method.

## Description

This subroutine is a translation of the ALGOL procedure TQLRAT. This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QL method. On Input N is the order of the matrix. N is an INTEGER variable. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E2 contains the squares of the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E2(1) is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). On Output D contains the eigenvalues in ascending order. If an

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
- GAMS classifications: `D4A5`
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

- Canonical provider: `lin/tqlrat.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tqlrat.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/tqlrat.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [TQLRAT](https://www.netlib.org/slatec/lin/tqlrat.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `E2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

ordered for indices 1, 2, ..., IERR-1, but may not be the smallest eigenvalues. E2 has been destroyed. IERR is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after 30 iterations. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::tqlrat`. Native symbol: `tqlrat_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tqlrat`
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
