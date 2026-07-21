# DEFC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram DFC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978).

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
- GAMS classifications: `K1A1A1`
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

- Canonical provider: `main-src/src/defc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/defc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/defc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/defc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence plus source-hash-guarded authored corrections
- Exact Netlib source: [DEFC](https://www.netlib.org/slatec/src/defc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDATA` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and. |
| 2 | `XDATA` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable array of `NDATA` abscissae for the discrete observations. The selected source permits unsorted input; native code does not retain the pointer. |
| 3 | `YDATA` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable array of `NDATA` ordinates paired with `XDATA`. These values are the observed data used in the weighted B-spline fit and are not retained. |
| 4 | `SDDATA` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and No sorting of XDATA(*) is required. Any non-negative value of NDATA is allowed. A negative value of NDATA is an error. A zero value for any entry of SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. |
| 5 | `NORD` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required. |
| 6 | `NBKPT` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required. |
| 7 | `BKPT` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The NBKPT knots of the B-spline of order NORD are in the array BKPT(*). Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,. , NORD-1 and I=NBKPT-NORD+2,. ,NBKPT, are required to compute the functions used to fit the data. No sorting of BKPT(*) is required. |
| 8 | `MDEIN` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1 The first time that DEFC( ) has been entered. There are NDATA points to process. =2 This is another entry to DEFC(). The sub- program DEFC( ) has been entered with MDEIN=1 exactly once before for this problem. There are NDATA new additional points to merge and process with any previous points. |
| 9 | `MDEOUT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An output flag that indicates the status of the curve fit. =-1 A usage error of DEFC( ) occurred. The offending condition is noted with the SLATEC. |
| 10 | `COEFF` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Writable B-spline coefficient array. When `MDEOUT=1`, it receives the `NBKPT-NORD` fitted coefficients; when the fit is incomplete or invalid, the selected source sets its entries to zero. |
| 11 | `LW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error. This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy. GE. |
| 12 | `W` | `workspace-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Persistent work array whose allocated length is supplied by `LW`. Its contents must not be modified by the caller between `MDEIN=1,2,2,...` calls; its leading `(NBKPT-NORD+3)*(NORD+1)` entries can be passed to `DFC` as an old problem when `MDEOUT` is 1 or 2. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::defc`. Native symbol: `defc_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::defc`
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
