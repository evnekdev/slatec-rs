# DEFC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram DFC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DEFC](https://www.netlib.org/slatec/src/defc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Output.. All TYPE REAL variables are DOUBLE PRECISION |
| 2 | `XDATA` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable array of `NDATA` abscissae for the discrete observations. The selected source permits unsorted input; native code does not retain the pointer. |
| 3 | `YDATA` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Readable array of `NDATA` ordinates paired with `XDATA`. These values are the observed data used in the weighted B-spline fit and are not retained. |
| 4 | `SDDATA` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The NDATA discrete (X,Y) pairs and the Y value standard deviation or uncertainty, SD, are in the respective arrays XDATA(*), YDATA(*), and No sorting of XDATA(*) is required.  Any non-negative value of NDATA is allowed.  A negative value of NDATA is an will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. |
| 5 | `NORD` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | NORD+1). 1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data.  No sorting of BKPT(*) is required. Internal to DEFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values.  The contents of must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition 4 when we are using piecewise cubics.) Output.. All TYPE REAL variables are DOUBLE PRECISION The first time DBVALU( ) is called, INBV=1 must be specified.  This value of INBV is the overwritten by DBVALU( ).  The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to DEFC( ). DBVALU( ) expects the breakpoint array BKPT(*) to be sorted. |
| 6 | `NBKPT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | spline of order NORD are in the array BKPT(*).  Normally the problem data interval will be included between NORD+1). .GE. 2*NORD. Other values are considered errors. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For example, NORD+3)*(NORD+1)+ Output.. All TYPE REAL variables are DOUBLE PRECISION Output.. All TYPE REAL variables are DOUBLE PRECISION NORD parameters are the B-spline coefficients. NORD+3)*(NORD+1) entries of W(*) and providing this data to DFC( ) with the "old problem" designation. The user should read the usage instructions for subprogram DFC( ) for further details. Working Array.. All TYPE REAL variables are DOUBLE PRECISION NORD+3)*(NORD+1) entries of NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be |
| 7 | `BKPT` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | NORD+1). NORD+1). 1,..., is not changed. NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be |
| 8 | `MDEIN` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1  The first time that DEFC( ) has been entered.  There are NDATA points to process. =2  This is another entry to DEFC().  The sub- 1 exactly once before for this problem.  There are NDATA new additional points to merge and process with any previous points. ant that the set of knots remain fixed at the same values for all entries to DEFC( ).) 1,2,2,... . |
| 9 | `MDEOUT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | An output flag that indicates the status of the curve fit. =-1  A usage error of DEFC( ) occurred.  The offending condition is noted with the SLATEC 1, this array contains the unknowns obtained from the least 2, not enough data was processed to uniquely determine the B-spline coefficients. 1, all values of COEFF(*) are set to zero. If the user is not satisfied with the fitted curve returned by DEFC( ), the constrained least squares curve fitting subprogram DFC( ) may be required.  The work done within DEFC( ) to accumulate the data can be utilized by the user, if so desired.  This involves 1 or 2. Evaluating the Fitted Curve.. To evaluate derivative number IDER at XVAL, use the function subprogram DBVALU( ). 1 was obtained from DEFC( ), XVAL is in the data interval, and IDER is nonnegative and .LT. |
| 10 | `COEFF` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be |
| 11 | `LW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in DEFC( ). Insufficient storage allocated for W(*) is an error.  This feature was included in DEFC because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy NORD+3)*(NORD+1)+ |
| 12 | `W` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION This array is typed DOUBLE PRECISION. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls are acceptable as direct input to DFC( ) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978). Input.. All TYPE REAL variables are DOUBLE PRECISION This array is typed DOUBLE PRECISION. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls are acceptable as direct input to DFC( )

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
