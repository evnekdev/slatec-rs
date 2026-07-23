# XNRMP

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials.

## Description

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (DXNRMP is double-precision version) XNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree

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
- GAMS classifications: `C3A2`
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

- Canonical provider: `main-src/src/xnrmp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xnrmp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xnrmp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xnrmp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [XNRMP](https://www.netlib.org/slatec/src/xnrmp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NU` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are non-negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J. M. Smith, F. W. |
| 2 | `MU1` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | GE. 0 specifies the lowest-order normalized Legendre polynomial that is wanted. |
| 3 | `MU2` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | GE. MU1 specifies the highest-order normalized Leg- endre polynomial that is wanted. |
| 4 | `SARG` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input real argument. With `MODE=1` it is the Legendre argument and must lie in [-1,1]; with `MODE=2` it is an angle strictly between -pi and pi and the routine uses its cosine. |
| 5 | `MODE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 and -1. 0. LE. SARG. 1. 0 specifies that Normalized Legendre(NU, MU, SARG) is wanted for MU = MU1, MU1 + 1,. |
| 6 | `SPN` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Writable extended-range mantissa array. Together with `IPN`, element `I` represents the normalized Legendre value for order `MU1+I-1` in the selected extended-range representation. |
| 7 | `IPN` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | 0 the value of the normalized Legendre polynomial is contained entirely in SPN(I) and subsequent single-precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I). NE. 0 the corre- sponding value of the normalized Legendre polynomial cannot be represented in single-precision because of overflow or under- flow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user should try using double pre- cision if it has a wider exponent range. |
| 8 | `ISIG` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Writable output estimate of decimal digits lost to rounding in the extended-range normalized-Legendre calculation. Subtract it from the significant digits in the input argument to estimate retained result precision away from zeros. |
| 9 | `IERROR` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=112 or 113, invalid input was provided to XNRMP. If IERROR=101,102,103, or 104, invalid input was provided to XSET. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. |
| `IERROR` | `112` | or 113, invalid input was provided to XNRMP. |
| `IERROR` | `101` | ,102,103, or 104, invalid input was provided to XSET. |
| `IERROR` | `105` | or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the |

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::xnrmp`. Native symbol: `xnrmp_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::xnrmp`
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
