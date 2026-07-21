# AVINT

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Integrate a function tabulated at arbitrarily spaced abscissas using overlapping parabolas.

## Description

AVINT integrates a function tabulated at arbitrarily spaced abscissas. The limits of integration need not coincide with the tabulated abscissas. A method of overlapping parabolas fitted to the data is used provided that there are at least 3 abscissas between the limits of integration. AVINT also handles two special cases. If the limits of integration are equal, AVINT returns a result of zero regardless of the number of tabulated values. If there are only two function values, AVINT uses the trapezoid rule. Description of Parameters The user must dimension all arrays appearing in the call list X(N), Y(N).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A1B2`
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

- Canonical provider: `main-src/src/avint.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/avint.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/avint.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/avint.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [AVINT](https://www.netlib.org/slatec/src/avint.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | real array of abscissas, which must be in increasing order. |
| 2 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | real array of functional values. i.e., Y(I)=FUNC(X(I)). |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the integer number of function values supplied. XUP. |
| 4 | `XLO` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | XUP. real lower limit of integration. |
| 5 | `XUP` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | real upper limit of integration. Must have XLO .LE. XUP. |
| 6 | `ANS` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | computed approximate value of integral 2,3,4,or 5. AVINT is documented completely in SC-M-69-335 Original program from "Numerical Integration" by Davis & Rabinowitz. Adaptation and modifications for Sandia Mathematical Program |
| 7 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a status code --normal code =1 means the requested integration was performed. --abnormal codes =2 means XUP was less than XLO. =3 means the number of X(I) between XLO and XUP (inclusive) was less than 3 and neither of the two special cases described in the Abstract occurred. No integration was performed. =4 means the restriction X(I+1) .GT. X(I) was violated. =5 means the number N of function values was .LT. 2. 2,3,4,or 5. AVINT is documented completely in SC-M-69-335 Original program from "Numerical Integration" by Davis & Rabinowitz. Adaptation and modifications for Sandia Mathematical Program |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::avint`. Native symbol: `avint_`. Declaration feature: `quadrature`. Provider feature: `quadrature-direct`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32,mut_f32,mut_f32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::avint`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
