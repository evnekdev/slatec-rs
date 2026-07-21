# EFC

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Fit a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense.

## Description

This subprogram fits a piecewise polynomial curve to discrete data. The piecewise polynomials are represented as B-splines. The fitting is done in a weighted least squares sense. The data can be processed in groups of modest size. The size of the group is chosen by the user. This feature may be necessary for purposes of using constrained curve fitting with subprogram FC( ) on a very large data set. For a description of the B-splines and usage instructions to evaluate them, see C. W. de Boor, Package for Calculating with B-Splines. SIAM J. Numer. Anal., p. 441, (June, 1977). For further discussion of (constrained) curve fitting using B-splines, see R. J. Hanson, Constrained Least Squares Curve Fitting to Discrete Data Using B-Splines, a User's Guide. Sandia Labs. Tech. Rept. SAND-78-1291, December, (1978).

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

- Canonical provider: `main-src/src/efc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/efc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/efc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/efc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [EFC](https://www.netlib.org/slatec/src/efc.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDATA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `XDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `YDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `SDDATA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `NORD` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `NBKPT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `BKPT` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `MDEIN` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `MDEOUT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `COEFF` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `LW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

SDDATA(*) will weight that data point as 1. Otherwise the weight of that data point is the reciprocal of this entry. NORD,NBKPT, BKPT(*) The NBKPT knots of the B-spline of order NORD are in the array BKPT(*).  Normally the problem data interval will be included between the limits BKPT(NORD) and BKPT(NBKPT-NORD+1). The additional end knots BKPT(I),I=1,..., NORD-1 and I=NBKPT-NORD+2,...,NBKPT, are required to compute the functions used to fit the data.  No sorting of BKPT(*) is required. Internal to  EFC( ) the extreme end knots may be reduced and increased respectively to accommodate any data values that are exterior to the given knot values.  The contents of BKPT(*) is not changed. NORD must be in the range 1 .LE. NORD .LE. 20. The value of NBKPT must satisfy the condition NBKPT .GE. 2*NORD. (The order of the spline is one more than the degree of the piecewise polynomial defined on each interval.  This is consistent with the B-spline package convention.  For example, NORD=4 when we are using piecewise cubics.) MDEIN An integer flag, with one of two possible values (1 or 2), that directs the subprogram action with regard to new data points provided by the user. =1  The first time that EFC( ) has been entered.  There are NDATA points to process. =2  This is another entry to EFC( ).  The sub- program EFC( ) has been entered with MDEIN=1 exactly once before for this problem.  There are NDATA new additional points to merge and process with any previous points. (When using EFC( ) with MDEIN=2 it is import- ant that the set of knots remain fixed at the same values for all entries to EFC( ).) LW The amount of working storage actually allocated for the working array W(*). This quantity is compared with the actual amount of storage needed in EFC( ). Insufficient storage allocated for W(*) is because misreading the storage formula for W(*) might very well lead to subtle and hard-to-find programming bugs. The length of the array W(*) must satisfy LW .GE. (NBKPT-NORD+3)*(NORD+1)+ (NBKPT+1)*(NORD+1)+ 2*MAX(NDATA,NBKPT)+NBKPT+NORD**2 offending condition is noted with the SLATEC the working array W(*) is not long enough, the minimal acceptable length is printed. =1  The B-spline coefficients for the fitted curve have been returned in array COEFF(*). =2  Not enough data has been processed to determine the B-spline coefficients. The user has one of two options.  Continue to process more data until a unique set of coefficients is obtained, or use the subprogram FC( ) to obtain a specific set of coefficients.  The user should read the usage instructions for FC( ) for further details if this second option is chosen. COEFF(*) If the output value of MDEOUT=1, this array contains the unknowns obtained from the least squares fitting process.  These N=NBKPT-NORD parameters are the B-spline coefficients. For MDEOUT=2, not enough data was processed to uniquely determine the B-spline coefficients. In this case, and also when MDEOUT=-1, all values of COEFF(*) are set to zero. If the user is not satisfied with the fitted curve returned by EFC( ), the constrained least squares curve fitting subprogram FC( ) may be required.  The work done within EFC( ) to accumulate the data can be utilized by the user, if so desired.  This involves saving the first (NBKPT-NORD+3)*(NORD+1) entries of W(*) and providing this data to FC( ) with the "old problem" designation. The user should read the usage instructions for subprogram FC( ) for further details. Working Array.. W(*) This array is typed REAL. Its length is  specified as an input parameter in LW as noted above.  The contents of W(*) must not be modified by the user between calls to EFC( ) with values of MDEIN=1,2,2,... . The first (NBKPT-NORD+3)*(NORD+1) entries of W(*) are acceptable as direct input to FC( ) for an "old problem" only when MDEOUT=1 or 2. Evaluating the Fitted Curve.. To evaluate derivative number IDER at XVAL, use the function subprogram BVALU( ). F = BVALU(BKPT,COEFF,NBKPT-NORD,NORD,IDER, XVAL,INBV,WORKB) The output of this subprogram will not be defined unless an output value of MDEOUT=1 was obtained from EFC( ), XVAL is in the data interval, and IDER is nonnegative and .LT. NORD. The first time BVALU( ) is called, INBV=1 must be specified.  This value of INBV is the overwritten by BVALU( ).  The array WORKB(*) must be of length at least 3*NORD, and must not be the same as the W(*) array used in the call to EFC( ). BVALU( ) expects the breakpoint array BKPT(*) to be sorted.

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::efc`. Native symbol: `efc_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::efc`
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
