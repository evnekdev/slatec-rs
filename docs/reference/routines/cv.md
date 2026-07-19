# CV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram FC.

## Description

CV( ) is a companion function subprogram for FC( ). The documentation for FC( ) has complete usage instructions. CV( ) is used to evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram, FC( ). The variance function defines the square of the probable error of the fitted curve at any point, XVAL. One can use the square root of this variance function to determine a probable error band around the fitted curve. CV( ) is used after a call to FC( ). MODE, an input variable to FC( ), is used to indicate if the variance function is desired. In order to use CV( ), MODE must equal 2 or 4 on input to FC( ). MODE is also used as an output flag from FC( ). Check to make sure that MODE = 0 after calling FC( ), indicating a successful constrained curve fit. The array SDDATA, as input to FC( ), must also be defined with the standard deviation or uncertainty of the Y values to use CV( ). To evaluate the variance function after calling FC( ) as stated above, use CV( ) as shown here VAR=CV(XVAL,NDATA,NCONST,NORD,NBKPT,BKPT,W) The variance function is given by VAR=(transpose of B(XVAL))*C*B(XVAL)/MAX(NDATA-N,1) where N = NBKPT - NORD. The vector B(XVAL) is the B-spline basis function values at X=XVAL. The covariance matrix, C, of the solution coefficients accounts only for the least squares equations and the explicitly stated equality constraints. This fact must be considered when interpreting the variance function from a data fitting problem that has inequality constraints on the fitted curve. All the variables in the calling sequence for CV( ) are used in FC( ) except the variable XVAL. Do not change the values of these variables between the call to FC( ) and the use of CV( ). The following is a brief description of the variables XVAL The point where the variance is desired. NDATA The number of discrete (X,Y) pairs for which FC( ) calculated a piece-wise polynomial curve. NCONST The number of conditions that constrained the B-spline in FC( ). NORD The order of the B-spline used in FC( ). The value of NORD must satisfy 1 < NORD < 20 . (The order of the spline is one more than the degree of the piece-wise polynomial defined on each interval. This is consistent with the B-spline package convention. For example, NORD=4 when we are using piece-wise cubics.) NBKPT The number of knots in the array BKPT(*). The value of NBKPT must satisfy NBKPT .GE. 2*NORD. BKPT(*) The real array of knots. Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,...,NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required by FC( ) to compute the functions used to fit the data. W(*) Real work array as used in FC( ). See FC( ) for the required length of W(*). The contents of W(*) must not be modified by the user if the variance function is desired.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Probability and statistics`
- Mathematical domain: `probability-statistics`
- Package provenance: `unknown`
- GAMS classifications: `L7A3`
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

- Canonical provider: `main-src/src/cv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
