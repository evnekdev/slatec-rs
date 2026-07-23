# POLFIT

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit discrete data in a least squares sense by polynomials in one variable.

## Description

Given a collection of points X(I) and a set of values Y(I) which correspond to some function or measurement at each of the X(I), subroutine POLFIT computes the weighted least-squares polynomial fits of all degrees up to some degree either specified by the user or determined by the routine. The fits thus obtained are in orthogonal polynomial form. Subroutine PVALUE may then be called to evaluate the fitted polynomials and any of their derivatives at any point. The subroutine PCOEF may be used to express the polynomial fits as powers of (X-C) for any specified point C. The parameters for POLFIT are

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A1A2`
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

- Canonical provider: `main-src/src/polfit.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/polfit.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/polfit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/polfit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [POLFIT](https://www.netlib.org/slatec/src/polfit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the number of data points. The arrays X, Y and W must be dimensioned at least N (N. GE. 1). 0. LE. |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | array of values of the independent variable. These values may appear in any order and need not all be distinct. |
| 3 | `Y` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | array of corresponding function values. |
| 4 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | array of positive values to be used as weights. If is negative, POLFIT will set all the weights to 1. 0, which means unweighted least squares error will be minimized. To minimize relative error, the user should set the weights to: W(I) = 1. 0/Y(I)**2, I = 1,. ,N. |
| 5 | `MAXDEG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | maximum degree to be allowed for polynomial fit. may be any non-negative integer less than N. Note -- MAXDEG cannot be equal to N-1 when a statistical test is to be used for degree selection, i. e. , when input value of EPS is negative. may result in passing the test. |
| 6 | `NDEG` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | degree of the highest degree fit computed. |
| 7 | `EPS` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | specifies the criterion to be used in determining the degree of fit to be computed. (1) If EPS is input negative, POLFIT chooses the degree based on a statistical F test of significance. One of three possible significance levels will be used:. 01,. 05 or. 10. |
| 8 | `R` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | vector of dimension at least NDEG containing values of the fit of degree NDEG at each of the X(I). Except when the statistical test is used, these values are more accurate than results from subroutine PVALUE normally are. |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | error flag with the following possible values. 1 -- indicates normal execution, i. e. , either (1) the input value of EPS was negative, and the computed polynomial fit of degree NDEG satisfies the specified F test, or (2) the input value of EPS was 0. , and the fits of all degrees up to MAXDEG are complete, or (3) the input value of EPS was positive, and the polynomial of degree NDEG satisfies the RMS error requirement. 2 -- invalid input parameter. |
| 10 | `A` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | work and output array having at least 3N+3MAXDEG+3 locations Note - POLFIT calculates all fits of degrees up to and including NDEG. Any or all of these fits can be evaluated or expressed as powers of (X-C) using PVALUE and PCOEF after just one call to POLFIT. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::polfit`. Native symbol: `polfit_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::polfit`
- Public declaration feature: `approximation`
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
