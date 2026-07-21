# DPOLFT

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit discrete data in a least squares sense by polynomials in one variable.

## Description

Given a collection of points X(I) and a set of values Y(I) which correspond to some function or measurement at each of the X(I), subroutine DPOLFT computes the weighted least-squares polynomial fits of all degrees up to some degree either specified by the user or determined by the routine. The fits thus obtained are in orthogonal polynomial form. Subroutine DP1VLU may then be called to evaluate the fitted polynomials and any of their derivatives at any point. The subroutine DPCOEF may be used to express the polynomial fits as powers of (X-C) for any specified point C. The parameters for DPOLFT are Input -- All TYPE REAL variables are DOUBLE PRECISION N - the number of data points. The arrays X, Y and W must be dimensioned at least N (N .GE. 1). X - array of values of the independent variable. These values may appear in any order and need not all be distinct. Y - array of corresponding function values. W - array of positive values to be used as weights. If W(1) is negative, DPOLFT will set all the weights

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/dpolft.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpolft.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpolft.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpolft.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DPOLFT](https://www.netlib.org/slatec/src/dpolft.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `Y` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 5 | `MAXDEG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `NDEG` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `EPS` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `A` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

user should set the weights to:  W(I) = 1.0/Y(I)**2, I = 1,...,N . MAXDEG - maximum degree to be allowed for polynomial fit. MAXDEG  may be any non-negative integer less than  N. Note -- MAXDEG  cannot be equal to  N-1  when a statistical test is to be used for degree selection, i.e., when input value of  EPS  is negative. EPS -    specifies the criterion to be used in determining the degree of fit to be computed. (1)  If  EPS  is input negative,  DPOLFT  chooses the degree based on a statistical F test of significance.  One of three possible significance levels will be used:  .01, .05 or .10.  If  EPS=-1.0 , the routine will automatically select one of these levels based on the number of data points and the maximum degree to be considered.  If  EPS  is input as -.01, -.05, or -.10, a significance level of .01, .05, or .10, respectively, will be used. (2)  If  EPS  is set to 0.,  DPOLFT  computes the polynomials of degrees 0 through  MAXDEG . (3)  If  EPS  is input positive,  EPS  is the RMS fitted polynomial.  DPOLFT  will increase the degree of fit until this criterion is met or until the maximum degree is reached. Output -- All TYPE REAL variables are DOUBLE PRECISION NDEG -   degree of the highest degree fit computed. R -      vector of dimension at least NDEG containing values of the fit of degree  NDEG  at each of the  X(I) . Except when the statistical test is used, these values are more accurate than results from subroutine DP1VLU  normally are. 1 -- indicates normal execution, i.e., either (1)  the input value of  EPS  was negative, and the computed polynomial fit of degree  NDEG satisfies the specified F test, or (2)  the input value of  EPS  was 0., and the fits of all degrees up to  MAXDEG  are complete, or (3)  the input value of  EPS  was positive, and the polynomial of degree  NDEG  satisfies the RMS 2 -- invalid input parameter.  At least one of the input parameters has an illegal value and must be corrected before  DPOLFT  can proceed.  Valid input results when the following restrictions are observed N .GE. 1 0 .LE. MAXDEG .LE. N-1  for  EPS .GE. 0. 0 .LE. MAXDEG .LE. N-2  for  EPS .LT. 0. W(1)=-1.0  or  W(I) .GT. 0., I=1,...,N . polynomial of degree no greater than  MAXDEG .  Best fit found is of degree  MAXDEG . 4 -- cannot satisfy the test for significance using current value of  MAXDEG .  Statistically, the best fit found is of order  NORD .  (In this case, NDEG will have one of the values:  MAXDEG-2, MAXDEG-1, or MAXDEG).  Using a higher value of MAXDEG  may result in passing the test. A -      work and output array having at least 3N+3MAXDEG+3 locations Note - DPOLFT  calculates all fits of degrees up to and including NDEG .  Any or all of these fits can be evaluated or expressed as powers of (X-C) using  DP1VLU  and  DPCOEF after just one call to  DPOLFT .

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::dpolft`. Native symbol: `dpolft_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::dpolft`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
