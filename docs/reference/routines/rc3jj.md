# RC3JJ

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the 3j symbol f(L1) = ( L1 L2 L3) (-M2-M3 M2 M3) for all allowed values of L1, the other parameters being held fixed.

## Description

Usage: REAL L2, L3, M2, M3, L1MIN, L1MAX, THRCOF(NDIM) INTEGER NDIM, IER CALL RC3JJ (L2, L3, M2, M3, L1MIN, L1MAX, THRCOF, NDIM, IER) Although conventionally the parameters of the vector addition coefficients satisfy certain restrictions, such as being integers or integers plus 1/2, the restrictions imposed on input to this subroutine are somewhat weaker. See, for example, Section 27.9 of Abramowitz and Stegun or Appendix C of Volume II of A. Messiah. The restrictions imposed by this subroutine are 1. L2 .GE. ABS(M2) and L3 .GE. ABS(M3); 2. L2+ABS(M2) and L3+ABS(M3) must be integers; 3. L1MAX-L1MIN must be a non-negative integer, where L1MAX=L2+L3 and L1MIN=MAX(ABS(L2-L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C19`
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

- Canonical provider: `main-src/src/rc3jj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rc3jj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rc3jj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rc3jj.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [RC3JJ](https://www.netlib.org/slatec/src/rc3jj.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `L2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `L3` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `M2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `M3` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `L1MIN` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | 1), I=1,2,...,L1MAX+L1MIN+1. NDIM :IN    Declared length of THRCOF in calling program. |
| 6 | `L1MAX` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `THRCOF` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NDIM) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `NDIM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `IER` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IER=1 Either L2.LT.ABS(M2) or L3.LT.ABS(M3). IER=2 Either L2+ABS(M2) or L3+ABS(M3) non-integer. IER=3 L1MAX-L1MIN not an integer. IER=4 L1MAX less than L1MIN. IER=5 NDIM less than L1MAX-L1MIN+1. SLATEC standards. These changes were done by D. W. Lozier, M. A. McClain and J. M. Smith of the National Institute of Standards and Technology, formerly NBS.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::rc3jj`. Native symbol: `rc3jj_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::rc3jj`
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
